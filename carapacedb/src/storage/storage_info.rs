
/// The version number of the database storage format
pub static VERSION_NUMBER: u64 = 1; 

// Size of a memory slot managed by the StorageManager. 
// This is the quantum of allocation for Blocks on DuckDB. default to 256KB. (1 << 18)
pub const BLOCK_SIZE: usize = 262144;

/// The size of the headers. This should be small and written more or less atomically by the hard disk. Default to
/// the page size, which is 4KB. (1 << 12)
pub const HEADER_SIZE: usize = 4096;

/// Block ID type alias
pub type BlockId = i64;

/// Invalid block identifier
pub const INVALID_BLOCK: BlockId = -1;

/// The MainHeader is the first header in the storage file. 
/// it is typically written only once for a database file.
#[derive(Debug, Clone, Copy)]
pub struct MainHeader {
    pub version_number: u64,
    pub flags: [u64; 4],
}

/// The DatabaseHeader contains information about the current state of the database. Every storage file has two
/// DatabaseHeaders. On startup, the DatabaseHeader with the highest iteration count is used as the active header.
/// When a checkpoint is performed, the active DatabaseHeader is switched by increasing the iteration count of the
/// DatabaseHeader.
#[derive(Debug, Clone, Copy)]
pub struct DatabaseHeader {
    /// The iteration count, increases by 1 every time the storage is checkpointed.
    pub iteration: u64,
    /// A pointer to the initial meta block
    pub meta_block: BlockId,
    /// A pointer to the block containing the free list
    pub free_list: BlockId,
    /// The number of blocks that is in the file as of this database header. If the file is larger than BLOCK_SIZE *
    /// block_count any blocks appearing AFTER block_count are implicitly part of the free_list.
    pub block_count: u64,
}
