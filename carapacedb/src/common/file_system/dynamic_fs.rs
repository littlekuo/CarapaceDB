use super::FileFlags;
use super::FileLockType;
use std::io::{Result};
use std::sync::Arc;
use std::path::{Path, PathBuf};


pub trait DynFileHandle: Send + Sync {
    fn file_system(&self) -> Arc<dyn DynFileSystem>;
    fn path(&self) -> &Path;
}

pub trait DynFileSystem: Send + Sync {
    fn read_at(&self, handle: &dyn DynFileHandle, buffer: &mut [u8], nr_bytes: i64, location: u64) -> Result<()>;
 
    fn write_at(&self, handle: &dyn DynFileHandle, buffer: &[u8], nr_bytes: i64, location: u64) -> Result<()>;

    fn open_file(self:Arc<Self>, path: &Path, flags: FileFlags, lock: Option<FileLockType>) -> Result<Box<dyn DynFileHandle>>;

    fn set_file_pointer(&self, handle: &dyn DynFileHandle, location: u64) -> Result<()>;
    
    fn read(&self, handle: &dyn DynFileHandle, buffer: &mut [u8], nr_bytes: i64) -> Result<()>;

    fn write(&self, handle: &dyn DynFileHandle, buffer: &[u8], nr_bytes: i64) -> Result<()>;

    fn file_size(&self, handle: &dyn DynFileHandle) -> Result<u64>;
    
    fn directory_exists(&self, path: &Path) -> Result<bool>;

    fn file_exists(&self, file_name: &Path) -> Result<bool>;
    
    fn create_directory(&self, path: &Path) -> Result<()>;
    
    fn remove_directory(&self, path: &Path) -> Result<()>;
    
    fn remove_file(&self, file_name: &Path) -> Result<()>;

    fn list_files(&self, directory: &Path, callback: &mut dyn FnMut(String)) -> Result<bool>;

    fn path_separator(&self) -> &'static str;

    fn fsync(&self, handle: &dyn DynFileHandle) -> Result<()>;

    fn move_file(&self, src: &Path, dst: &Path) -> Result<()>;
    
    fn join_path(&self, l: &Path, r:&Path) -> Result<PathBuf>;
}
