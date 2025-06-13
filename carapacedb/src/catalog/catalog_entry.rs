use std::sync::{Arc, Weak};
use std::cell::RefCell;

use crate::catalog::catalog_set::CatalogSet;
use crate::common::catalog_type::CatalogType;
use crate::parser::parsed_data::alter_table_info::AlterInfo;
use super::catalog::Catalog;

pub type CatalogEntryId = u64;


#[derive(Debug)]
pub struct CatalogError {
    message: String,
}

impl CatalogError {
    pub fn new(msg: &str) -> Self {
        CatalogError {
            message: msg.to_string(),
        }
    }
}

pub struct ClientContext;

pub trait CatalogEntryTrait {
    fn alter_entry(
        &self,
        context: &ClientContext,
        info: &AlterInfo,
    ) -> Result<Arc<RefCell<dyn CatalogEntryTrait>>, CatalogError> {
        Err(CatalogError::new("Unsupported alter type for catalog entry!"))
    }

    fn id(&self) -> CatalogEntryId;

    fn get_type(&self) -> CatalogType;

    fn get_catalog(&self) -> Weak<Catalog>;

    fn get_catalog_set(&self) -> Weak<CatalogSet>;
    
    fn get_name(&self) -> &str;
    
    fn is_deleted(&self) -> bool;
    
    fn get_timestamp(&self) -> u64;
    
    fn get_child(&self) -> Option<Arc<dyn CatalogEntryTrait>>;
    
    fn set_child(&mut self, child: Option<Arc<dyn CatalogEntryTrait>>);
    
    fn get_parent(&self) -> Option<Weak<dyn CatalogEntryTrait>>;
    
    fn set_parent(&mut self, parent: Option<Weak<dyn CatalogEntryTrait>>);
}


pub struct BaseCatalogEntry {
    type_: CatalogType,
    catalog: Weak<Catalog>,
    set: Weak<CatalogSet>,
    ///! The name of the entry
    name: String,
    ///! Whether or not the object is deleted
    deleted: bool,
    ///! Timestamp at which the catalog entry was created
    timestamp: u64,
    child: Option<Arc<dyn CatalogEntryTrait>>,
    parent: Option<Weak<dyn CatalogEntryTrait>>,
}

impl BaseCatalogEntry {
    pub fn new(
        type_: CatalogType,
        catalog: Weak<RefCell<Catalog>>,
        name: String,
    ) -> Self {
        Self {
            type_,
            catalog,
            set: None,
            name,
            deleted: false,
            timestamp: 0, // 实际应用中应使用事务ID
            child: None,
            parent: None,
        }
    }
}
