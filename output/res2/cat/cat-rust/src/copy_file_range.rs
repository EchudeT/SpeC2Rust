use std::io;

pub struct CopyFileRange;

impl CopyFileRange {
    pub fn copy(
        _input_fd: i32,
        _input_offset: Option<u64>,
        _output_fd: i32,
        _output_offset: Option<u64>,
        _length: usize,
        _flags: u32,
    ) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}
