pub mod dynamic_fs;
pub mod static_fs;

use bitflags::bitflags;
use static_fs::{LocalFileSystem, LocalFileHandle};
use dynamic_fs::{DynFileSystem, DynFileHandle};

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

#[derive(Debug)]
pub enum UnifiedFileSystem {
    Local(LocalFileSystem),
    Plugin(Box<dyn DynFileSystem<'static> + 'static>),
}

pub enum UnifiedFileHandle<'a> {
    Local(LocalFileHandle<'a>),
    Plugin(Box<dyn DynFileHandle<'a> + 'a>),
}
