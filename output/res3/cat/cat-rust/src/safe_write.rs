use std::io::{self, ErrorKind, Write};

pub struct SafeWrite;

impl SafeWrite {
    pub fn write_some<W: Write>(writer: &mut W, buffer: &[u8]) -> io::Result<usize> {
        loop {
            match writer.write(buffer) {
                Ok(count) => return Ok(count),
                Err(error) if error.kind() == ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
    }
}
