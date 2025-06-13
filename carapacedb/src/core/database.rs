use std::marker::PhantomPinned;
use std::sync::Arc;

use crate::catalog::catalog::Catalog;
use crate::common::file_system::UnifiedFileSystem;
use super::connection_manager::ConnectionManager;
use crate::storage::storage_manager::StorageManager;
use crate::transaction::transaction_manager::TransactionManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Undefined,
    ReadOnly,
    ReadWrite,
}

pub struct DBConfig {
    pub access_mode: AccessMode,
    pub file_system: Option<Box<UnifiedFileSystem>>,
}

impl Default for DBConfig {
    fn default() -> Self {
        DBConfig {
            access_mode: AccessMode::Undefined,
            file_system: None,
        }
    }
}

//
// currently, we just only arc and weak reference for safe, not use pin 
//  and raw pointer, maybe optimize later
//
pub struct DuckDB {
    pub file_system: Arc<UnifiedFileSystem>,
    pub storage: Arc<StorageManager>,
    pub catalog: Arc<Catalog>,
    pub transaction_manager: Box<TransactionManager>,
    pub connection_manager: Box<ConnectionManager>,
    pub access_mode: AccessMode,
}
