use std::io::{self, Write};

pub struct FullWrite;

impl FullWrite {
    pub fn write_all<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<usize> {
        let mut total = 0usize;
        let mut remaining = buf;

        while !remaining.is_empty() {
            match writer.write(remaining) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to complete full write",
                    ));
                }
                Ok(written) => {
                    total += written;
                    remaining = &remaining[written..];
                }
                Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
                Err(err) => {
                    if total == 0 {
                        return Err(err);
                    }
                    return Ok(total);
                }
            }
        }

        Ok(total)
    }
}
