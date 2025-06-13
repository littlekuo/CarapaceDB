pub mod dynamic_fs;
pub mod static_fs;

use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLockType {
    NoLock,
    ReadLock,
    WriteLock,
}

bitflags! {
    pub struct FileFlags: u16 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const DIRECT_IO = 1 << 2;
        const CREATE = 1 << 3;
    }
}
