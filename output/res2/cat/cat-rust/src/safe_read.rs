use std::io::{self, ErrorKind, Read};

pub struct SafeRead;

impl SafeRead {
    pub fn read_some<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<usize> {
        let mut limit = buf.len();

        loop {
            match reader.read(&mut buf[..limit]) {
                Ok(result) => return Ok(result),
                Err(error) if error.kind() == ErrorKind::Interrupted => continue,
                Err(error) if error.kind() == ErrorKind::InvalidInput && limit < buf.len() => {
                    return Err(error);
                }
                Err(error) if error.kind() == ErrorKind::InvalidInput => {
                    limit = Self::system_buffer_size_max(buf.len());
                }
                Err(error) => return Err(error),
            }
        }
    }

    fn system_buffer_size_max(requested: usize) -> usize {
        requested.min(isize::MAX as usize)
    }
}
