use std::path::{Path, PathBuf};
use std::io::{Result, Error, ErrorKind};
use std::os::unix::io::RawFd;

use super::FileFlags;
use super::FileLockType;
use std::ffi::{CString, CStr, OsStr};
use std::os::unix::ffi::OsStrExt;

pub trait SFileHandle<FS: SFileSystem>: Send + Sync {
    fn file_system(&self) -> &FS; 

    fn path(&self) -> &Path;
    fn close(&mut self) -> Result<()>;
}

pub trait SFileSystem: Send + Sync + Sized {
    type Handle<'a>: SFileHandle<Self> 
    where 
        Self: 'a;

    fn read_at(&self, handle: &Self::Handle<'_>, buffer: &mut [u8], nr_bytes: i64, location: u64) -> Result<()>;
 
    fn write_at(&self, handle: &Self::Handle<'_>, buffer: &[u8], nr_bytes: i64, location: u64) -> Result<()>;
    
    fn open_file<'a>(&'a self, path: &Path, flags: FileFlags, lock: FileLockType) -> Result<Self::Handle<'a>>;

    fn set_file_pointer(&self, handle: &Self::Handle<'_>, location: u64) -> Result<()>;

    fn read(&self, handle: &Self::Handle<'_>, buffer: &mut [u8], nr_bytes: i64) -> Result<u64>;

    fn write(&self, handle: &Self::Handle<'_>, buffer: &[u8], nr_bytes: i64) -> Result<u64>;

    fn file_size(&self, handle: &Self::Handle<'_>) -> Result<u64>;
    
    fn directory_exists(&self, path: &Path) -> Result<bool>;

    fn file_exists(&self, file_name: &Path) -> Result<bool>;
    
    fn create_directory(&self, path: &Path) -> Result<()>;
    
    fn remove_directory(&self, path: &Path) -> Result<()>;

    fn remove_file(&self, file_name: &Path) -> Result<()>;

    fn list_files<F>(&self, directory: &Path, callback: F) -> Result<bool> where F: FnMut(String);

    fn path_separator(&self) -> &'static str;

    fn fsync(&self, handle: &Self::Handle<'_>) -> Result<()>;

    fn move_file(&self, src: &Path, dst: &Path) -> Result<()>;
    
    fn join_path(&self, l: &Path, r:&Path) -> Result<PathBuf>;
}

#[derive(Debug, Clone, Copy)]
pub struct LocalFileSystem;

pub struct LocalFileHandle<'a> {
    fs: &'a LocalFileSystem,
    pub path: PathBuf,
    #[cfg(unix)]
    pub fd: RawFd,

    #[cfg(windows)]
    pub fd: std::os::windows::io::RawHandle,
}

impl<'a> SFileHandle<LocalFileSystem> for LocalFileHandle<'a> {

    fn file_system(&self) -> &LocalFileSystem {
        return self.fs;
    }

    fn path(&self) -> &Path {
        &self.path
    }
    
    fn close(&mut self) -> Result<()> {
        #[cfg(unix)]
        {
            let ret = unsafe { libc::close(self.fd) };
            if ret < 0 {
                return Err(Error::last_os_error());
            }
        }
        
        #[cfg(windows)]
        {
            if unsafe { winapi::um::handleapi::CloseHandle(self.handle) } == 0 {
                return Err(Error::last_os_error());
            }
        }
        
        Ok(())
    }
}

impl<'a> Drop for LocalFileHandle<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("failed to close handle: {}", e);
        }
    }
}

#[cfg(unix)]
fn remove_directory(path: &Path) -> Result<()> {
    let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|e| {
        Error::new(ErrorKind::InvalidInput, e)
    })?;
    let dir_ptr = unsafe { libc::opendir(c_path.as_ptr()) };
    if dir_ptr.is_null() {
        return Err(Error::last_os_error());
    }

    struct DirGuard(*mut libc::DIR);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { libc::closedir(self.0) };
            }
        }
    }
    let dir_guard = DirGuard(dir_ptr);

    let mut last_error = None;
    unsafe {
        let mut entry: *mut libc::dirent;
        loop {
            if last_error.is_some() {
                break;
            }
            entry = libc::readdir(dir_ptr);
            if entry.is_null() {
                break;
            }
            let d_name = (*entry).d_name.as_ptr();
            let name_cstr = unsafe { CStr::from_ptr(d_name) };
            let name_bytes = name_cstr.to_bytes();

            if name_bytes == b"." || name_bytes == b".." {
                continue;
            }

            let name_os = OsStr::from_bytes(name_bytes);
            let sub_path = path.join(name_os);
            let sub_path_c = match CString::new(sub_path.as_os_str().as_bytes()) {
                Ok(c) => c,
                Err(e) => {
                    last_error = Some(Error::new(ErrorKind::InvalidInput, e));
                    continue;
                }
            };
            let mut stat_buf = std::mem::zeroed();
            if libc::stat(sub_path_c.as_ptr(), &mut stat_buf) != 0 {
                last_error = Some(Error::last_os_error());
                continue;
            }

            let res = if (stat_buf.st_mode & libc::S_IFMT) == libc::S_IFDIR {
                remove_directory(&sub_path)
            } else {
                let ret = libc::unlink(sub_path_c.as_ptr());
                if ret != 0 {
                    return Err(Error::last_os_error());
                }
                Ok(())
            };

            if let Err(e) = res {
                last_error = Some(e);
            }
        }
    }

    drop(dir_guard);

    let rmdir_ret = unsafe { libc::rmdir(c_path.as_ptr()) };
    if rmdir_ret == -1 {
        return Err(Error::last_os_error());
    }

    if let Some(e) = last_error {
        return Err(e);
    }

    Ok(())
}

#[cfg(unix)]
impl SFileSystem for LocalFileSystem {
    type Handle<'a> = LocalFileHandle<'a>;

    fn open_file<'a>(
        &'a self,
        path: &Path,
        flags: FileFlags,
        lock_type: FileLockType,
    ) -> Result<Self::Handle<'a>> {
        use std::ffi::CString;

        use libc::{fcntl, F_RDLCK, F_SETLK, F_WRLCK};

        debug_assert!(
            !flags.contains(FileFlags::READ | FileFlags::WRITE),
            "cannot combine READ and WRITE flags"
        );
        debug_assert!(
            !flags.contains(FileFlags::READ | FileFlags::CREATE),
            "cannot combine READ and CREATE flags"
        );

        let mut open_flags = if flags.contains(FileFlags::READ) {
            libc::O_RDONLY
        } else {
            libc::O_RDWR | libc::O_CLOEXEC
        };

        if flags.contains(FileFlags::CREATE) {
            open_flags |= libc::O_CREAT;
        }
    
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if flags.contains(FileFlags::DIRECT_IO) != 0 {
            open_flags |= libc::O_SYNC;
        }

        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        if flags.contains(FileFlags::DIRECT_IO) {
            open_flags |= libc::O_DIRECT | libc::O_SYNC;
        }

        let c_path = CString::new(path.as_os_str().as_bytes()).unwrap();
        let fd = unsafe { libc::open(c_path.as_ptr(), open_flags, 0o666) };

        if fd == -1 {
            return Err(Error::last_os_error());
        }

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if flags.contains(FileFlags::DIRECT_IO) {
            if unsafe { libc::fcntl(fd, libc::F_NOCACHE, 1) } == -1 {
                unsafe { libc::close(fd) };
                return Err(io::Error::last_os_error());
            }
        }
    
        if lock_type != FileLockType::NoLock {
            let lock_type = match lock_type {
                FileLockType::ReadLock => F_RDLCK,
                FileLockType::WriteLock => F_WRLCK,
                _ => unreachable!(),
            };

            let flock = libc::flock {
                l_type: lock_type as i16,
                l_whence: libc::SEEK_SET as i16,
                l_start: 0,
                l_len: 0,
                l_pid: 0,
            };

            if unsafe { fcntl(fd, F_SETLK, &flock) } == -1 {
                unsafe { libc::close(fd) };
                return Err(Error::last_os_error());
            }
        }

        Ok(LocalFileHandle {
            fs: self,  
            path: path.to_path_buf(),       
            fd,                            
        })
    }


    fn set_file_pointer(&self, handle: &Self::Handle<'_>, location: u64) -> Result<()> {
        let result = unsafe {
            libc::lseek(
                handle.fd,
                location as libc::off_t,
                libc::SEEK_SET,
            )
        };

        if result == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }


    fn read(&self, handle: &Self::Handle<'_>, buffer: &mut [u8], nr_bytes: i64) -> Result<u64> {
        let result = unsafe {
            libc::read(
                handle.fd,
                buffer.as_mut_ptr() as *mut libc::c_void,
                buffer.len(),
            )
        };

        if result == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(result as u64)
        }
    }

    fn write(&self, handle: &Self::Handle<'_>, buffer: &[u8], nr_bytes: i64) -> Result<u64> {
        let result = unsafe {
            libc::write(
                handle.fd,
                buffer.as_ptr() as *const libc::c_void,
                buffer.len(),
            )
        };

        if result == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(result as u64)
        }
    }

    fn file_size(&self, handle: &Self::Handle<'_>) -> Result<u64> {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        
        if unsafe { libc::fstat(handle.fd, stat.as_mut_ptr()) } == -1 {
            return Err(Error::last_os_error());
        }
        
        let stat = unsafe { stat.assume_init() };
        Ok(stat.st_size as u64)
    }

    fn directory_exists(&self, path: &Path) -> Result<bool> {
        if path.as_os_str().is_empty() {
            return Ok(false);
        }

        let c_path = CString::new(path.as_os_str().as_bytes())?;
        
        if unsafe { libc::access(c_path.as_ptr(), libc::F_OK) } != 0 {
            return Ok(false);
        }
        
        let mut status = std::mem::MaybeUninit::<libc::stat>::uninit();
        if unsafe { libc::stat(c_path.as_ptr(), status.as_mut_ptr()) } != 0 {
            return Err(Error::last_os_error());
        }
        
        let status = unsafe { status.assume_init() };
        
        Ok((status.st_mode & libc::S_IFMT) == libc::S_IFDIR)
    }

    fn file_exists(&self, file_name: &Path) -> Result<bool> {
        if file_name.as_os_str().is_empty() {
            return Ok(false);
        }

        let c_path = CString::new(file_name.as_os_str().as_bytes())?;
        
        if unsafe { libc::access(c_path.as_ptr(), libc::F_OK) } != 0 {
            return Ok(false);
        }
        
        let mut status = std::mem::MaybeUninit::<libc::stat>::uninit();
        if unsafe { libc::stat(c_path.as_ptr(), status.as_mut_ptr()) } != 0 {
            return Err(Error::last_os_error());
        }
        
        let status = unsafe { status.assume_init() };
        
        Ok((status.st_mode & libc::S_IFMT) != libc::S_IFDIR)
    }
   
    fn create_directory(&self, path: &Path) -> Result<()> {
        let mut status = std::mem::MaybeUninit::<libc::stat>::uninit();

        let c_path = CString::new(path.as_os_str().as_bytes())?;

        let stat_result = unsafe { libc::stat(c_path.as_ptr(), status.as_mut_ptr()) };
        
        if stat_result != 0 {
            let mkdir_result = unsafe { libc::mkdir(c_path.as_ptr(), 0o755) };
            if mkdir_result != 0 {
                let err = Error::last_os_error();
                if err.raw_os_error() != Some(libc::EEXIST) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Failed to create directory: {}", err)
                    ));
                }
            }
        } else {
            let status = unsafe { status.assume_init() };
            if (status.st_mode & libc::S_IFMT) != libc::S_IFDIR {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Could not create directory: path exists but is not a directory"
                ));
            }
        }
        
        Ok(())
    }

    fn remove_directory(&self, path: &Path) -> Result<()> {
        remove_directory(path)
    }

    fn remove_file(&self, file_name: &Path) -> Result<()> {
        std::fs::remove_file(file_name).map_err(|e| {
            Error::new(
                e.kind(),
                format!("failed to remove file '{}': {}", file_name.display(), e)
            )
        })
    }

    fn list_files<F>(&self, directory: &Path, callback: F) -> Result<bool>
     where F: FnMut(String) {
        if !self.directory_exists(directory)? {
            return Ok(false);
        }

        let mut callback = callback;

        let c_dir = CString::new(directory.as_os_str().as_bytes())?;
        
        let dir = unsafe { libc::opendir(c_dir.as_ptr()) };
        if dir.is_null() {
            return Ok(false);
        }
    
        loop {
            let entry = unsafe { libc::readdir(dir) };
            if entry.is_null() {
                break;
            }
    
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }
                .to_string_lossy()
                .into_owned();
    
            if !name.is_empty() && !name.starts_with('.') {
                callback(name);
            }
        }
    
        unsafe { libc::closedir(dir) };
        Ok(true)
     }

     fn path_separator(&self) -> &'static str {
        "/"
     }

     fn fsync(&self, handle: &Self::Handle<'_>) -> Result<()> {
        let result = unsafe { libc::fsync(handle.fd) };
        
        if result == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn move_file(&self, src: &Path, dst: &Path) -> Result<()> {
        //! FIXME: rename does not guarantee atomicity or overwriting target file if it exists
        let c_source = CString::new(src.as_os_str().as_bytes()).map_err(|e| {
            Error::new(ErrorKind::InvalidInput, format!("invalid source file path: {}", e))
        })?;
        
        let c_target = CString::new(dst.as_os_str().as_bytes()).map_err(|e| {
            Error::new(ErrorKind::InvalidInput, format!("invalid destination file path: {}", e))
        })?;
        
        let result = unsafe {
            libc::rename(c_source.as_ptr(), c_target.as_ptr())
        };
        
        if result == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn join_path(&self, l: &Path, r: &Path) -> Result<PathBuf> {
        let l_str = l.to_string_lossy();
        let r_str = r.to_string_lossy();
        let sep = self.path_separator();
    
        let full_path = l_str.to_string() + sep + &r_str;
        Ok(PathBuf::from(full_path))
    }

    fn read_at(&self, handle: &Self::Handle<'_>, buffer: &mut [u8], nr_bytes: i64, location: u64) -> Result<()> {
        self.set_file_pointer(handle, location)?;
        let bytes_read = self.read(handle, buffer, nr_bytes)?;

        if bytes_read as i64 != nr_bytes {
            return Err(Error::new(
                ErrorKind::UnexpectedEof, 
                format!("read_at failed: expected {} bytes, but read {}", nr_bytes, bytes_read)
            ));
        }
        
        Ok(())
    }

    fn write_at(&self, handle: &Self::Handle<'_>, buffer: &[u8], nr_bytes: i64, location: u64) -> Result<()> {
        self.set_file_pointer(handle, location)?;
        let bytes_written = self.write(handle, buffer, nr_bytes)?;
        if bytes_written as i64 != nr_bytes {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("write_at failed: expected {} bytes, but wrote {}", nr_bytes, bytes_written)
            ));
        }
        Ok(())
    }
}
  
