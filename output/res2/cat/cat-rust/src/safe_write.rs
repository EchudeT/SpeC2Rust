use std::io::{self, Write};

pub struct SafeWrite;

impl SafeWrite {
    pub fn write_some<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<usize> {
        loop {
            match writer.write(buf) {
                Ok(n) => return Ok(n),
                Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }
    }

    pub fn system_buffer_size_max(requested: usize) -> usize {
        const DEFAULT_MAX: usize = 0x7fff_f000;
        requested.min(DEFAULT_MAX)
    }
}
