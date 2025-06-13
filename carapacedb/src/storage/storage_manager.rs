use std::{path::PathBuf, sync::Weak };
use crate::{core::database::DuckDB, storage::wal::WriteAheadLog};
use super::block_manager::BlockManager;

///! StorageManager is responsible for managing the physical storage of the
///! database on disk
pub struct StorageManager {
    database: Weak<DuckDB>,
    path: PathBuf,
    read_only: bool,
    block_manager: Box<dyn BlockManager>,
    wal: WriteAheadLog,
}

impl StorageManager {
}
