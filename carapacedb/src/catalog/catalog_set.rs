use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::catalog::Catalog;
use super::catalog_entry::{CatalogEntryTrait, CatalogEntryId};


type SetLockMap = HashMap<Arc<CatalogSet>, MutexGuard<'static, CatalogSet>>;

///! The Catalog Set stores (key, value) map of a set of AbstractCatalogEntries
pub struct CatalogSet {
    catalog: Weak<Catalog>,
    ///! The set of entries present in the CatalogSet.
    name_map: Mutex<HashMap<String, CatalogEntryId>>,
    data: Mutex<HashMap<CatalogEntryId, Arc<dyn CatalogEntryTrait + Send + Sync>>>,
}

impl CatalogSet {
}
