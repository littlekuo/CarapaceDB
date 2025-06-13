use crate::common::file_buffer::FileBuffer;
use super::storage_info::BlockId;

pub struct Block {
    file_buffer: FileBuffer,
    pub block_id: BlockId,
}
