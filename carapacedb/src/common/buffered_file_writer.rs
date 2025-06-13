use std::sync::Weak;
use crate::common::file_system::{UnifiedFileHandle, UnifiedFileSystem};

const FILE_BUFFER_SIZE: usize = 4096;

pub struct BufferedFileWriter {
    fs: Weak<UnifiedFileSystem>,
    buffer: [u8; FILE_BUFFER_SIZE],
    offset: usize,
    handle: Box<UnifiedFileHandle<'static>>,
}
