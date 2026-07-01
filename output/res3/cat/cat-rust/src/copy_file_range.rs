use std::fs::File;
use std::io;

pub struct CopyFileRange;

impl CopyFileRange {
    pub fn copy_file_range(
        input: &mut File,
        input_offset: Option<u64>,
        output: &mut File,
        output_offset: Option<u64>,
        length: usize,
        flags: u32,
    ) -> io::Result<usize> {
        let _ = (input, input_offset, output, output_offset, length, flags);
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}
