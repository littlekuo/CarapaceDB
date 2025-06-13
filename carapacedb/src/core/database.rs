use std::sync::Arc;
use crate::common::file_system::{FileSystem, FileHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Undefined,
    ReadOnly,
    ReadWrite,
}

pub struct DBConfig {
    pub access_mode: AccessMode,
    pub file_system: Option<FileSystem>,
}

impl Default for DBConfig {
    fn default() -> Self {
        DBConfig {
            access_mode: AccessMode::Undefined,
            file_system: None,
        }
    }
}

pub struct DuckDB {
    pub file_system: Box<FileSystem>,
    /// 存储管理器
    pub storage: Arc<StorageManager>,
    /// 目录管理器
    pub catalog: Arc<Catalog>,
    /// 事务管理器
    pub transaction_manager: Arc<TransactionManager>,
    /// 连接管理器
    pub connection_manager: Arc<ConnectionManager>,
    /// 当前访问模式
    pub access_mode: AccessMode,
}

pub struct StorageManager;
pub struct Catalog;
pub struct TransactionManager;
pub struct ConnectionManager;
