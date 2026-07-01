use std::io::{self, ErrorKind, Write};

pub struct FullWrite;

impl FullWrite {
    pub fn write_all_or_count<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<usize> {
        let mut total = 0;

        while total < buf.len() {
            match writer.write(&buf[total..]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        ErrorKind::WriteZero,
                        "write returned zero bytes before completing the buffer",
                    ));
                }
                Ok(written) => {
                    total += written;
                }
                Err(error) if error.kind() == ErrorKind::Interrupted => continue,
                Err(error) => {
                    if total == 0 {
                        return Err(error);
                    }
                    return Ok(total);
                }
            }
        }

        Ok(total)
    }
}
