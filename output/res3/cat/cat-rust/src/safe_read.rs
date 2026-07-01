use std::cmp::min;
use std::io::{self, Read};

pub struct SafeRead;

impl SafeRead {
    pub fn read_some<R: Read>(reader: &mut R, buffer: &mut [u8]) -> io::Result<usize> {
        let mut requested = buffer.len();

        loop {
            match reader.read(&mut buffer[..requested]) {
                Ok(count) => return Ok(count),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) if error.kind() == io::ErrorKind::InvalidInput && requested > i32::MAX as usize => {
                    requested = min(requested, i32::MAX as usize);
                }
                Err(error) => return Err(error),
            }
        }
    }
}
