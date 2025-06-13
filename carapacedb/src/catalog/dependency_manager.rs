use std::{collections::{HashMap, HashSet}, sync::Weak};

use crate::catalog::catalog_entry::CatalogEntryId;

use super::catalog::Catalog;
/// The DependencyManager is in charge of managing dependencies between 
/// catalog entries
pub struct DependencyManager {
    catalog: Weak<Catalog>,
    /// Map of objects that DEPEND on [object], i.e. [object] can only be deleted when all entries in the dependency map
	/// are deleted.
    dependents_map: HashMap<CatalogEntryId, HashSet<CatalogEntryId>>,
    /// Map of objects that the source object DEPENDS on, i.e. when any of the entries in the vector perform a CASCADE
	/// drop then [object] is deleted as wel
    dependencies_map: HashMap<CatalogEntryId, HashSet<CatalogEntryId>>,
}
