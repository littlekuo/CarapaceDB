pub mod dynamic_fs;
pub mod static_fs;

use std::sync::Arc;
use bitflags::bitflags;
use static_fs::{StaticFileSystem, StaticFileHandle};
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

pub enum FileSystem {
    Static(Arc<StaticFileSystem>),
    Dynamic(Box<dyn DynFileSystem>),
}

pub enum FileHandle {
    Static(StaticFileHandle),
    Dynamic(Box<dyn DynFileHandle>),
}
