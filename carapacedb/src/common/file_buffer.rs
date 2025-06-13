use std::mem;



const FILE_BUFFER_BLOCK_SIZE: usize = 4096;
const FILE_BUFFER_HEADER_SIZE: usize = mem::size_of::<u64>();

pub struct FileBuffer {
    pub buffer: *mut u8,
    pub size: usize,
    /// The pointer to the internal buffer that will be read or written, 
    ///  including the buffer header
    internal_buffer: *mut u8,
    internal_size: usize,
    /// The buffer that was actually malloc'd, i.e. 
    ///  the pointer that must be freed when the FileBuffer is destroyed
    malloced_buffer: *mut u8,
}
