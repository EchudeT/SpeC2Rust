use std::io::{self, Error, ErrorKind, Write};

pub struct FullWrite;

impl FullWrite {
    pub fn write_all<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<usize> {
        let mut total = 0usize;
        let mut remaining = buf;

        while !remaining.is_empty() {
            match writer.write(remaining) {
                Ok(0) => {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "zero-byte transfer while data remained to be written",
                    ));
                }
                Ok(n) => {
                    total += n;
                    remaining = &remaining[n..];
                }
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err),
            }
        }

        Ok(total)
    }
}
