use super::block::Block;
use super::storage_info::{BlockId, DatabaseHeader};


pub trait BlockManager {
    fn create_block(&mut self) -> Box<Block>;
    
    /// Return the next free block id
    fn get_free_block_id(&self) -> BlockId;
    
    /// Get the first meta block id
    fn get_meta_block(&self) -> BlockId;
    
    /// Read the content of a block from disk
    fn read(&self, block: & Block);
    
    fn write(&self, block: & Block);
    
    /// Write the header; should be the final step of a checkpoint
    fn write_header(&self, header: &DatabaseHeader);
}
