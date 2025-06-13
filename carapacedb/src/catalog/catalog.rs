use std::{sync::{Mutex, RwLock, Weak}};

use super::{catalog_set::CatalogSet, dependency_manager::DependencyManager};
use crate::storage::storage_manager::StorageManager;

pub struct Catalog {
    storage: Weak<StorageManager>,

    schemas: RwLock<CatalogSet>,
    dependency_manager: DependencyManager,
    
    write_lock: Mutex<()>,
}
