use std::io::{self, BufRead, Read};

pub struct StdioRead;

impl StdioRead {
    pub fn read_byte<R: Read>(reader: &mut R) -> io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        match reader.read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buf[0])),
            Err(err) => Err(err),
        }
    }

    pub fn read_line<R: BufRead>(reader: &mut R, limit: usize) -> io::Result<Option<String>> {
        if limit == 0 {
            return Ok(Some(String::new()));
        }

        let mut buf = Vec::new();
        let bytes_read = reader.by_ref().take(limit as u64 - 1).read_until(b'\n', &mut buf)?;

        if bytes_read == 0 {
            return Ok(None);
        }

        let text = String::from_utf8_lossy(&buf).into_owned();
        Ok(Some(text))
    }

    pub fn read_exact_items<R: Read>(
        reader: &mut R,
        item_size: usize,
        item_count: usize,
    ) -> io::Result<Vec<u8>> {
        if item_size == 0 || item_count == 0 {
            return Ok(Vec::new());
        }

        let total = item_size
            .checked_mul(item_count)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "read size overflow"))?;

        let mut buffer = vec![0u8; total];
        let mut filled = 0usize;

        while filled < total {
            match reader.read(&mut buffer[filled..]) {
                Ok(0) => {
                    buffer.truncate(filled);
                    return Ok(buffer);
                }
                Ok(n) => filled += n,
                Err(err) => return Err(err),
            }
        }

        Ok(buffer)
    }

    pub fn read_to_string<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        Ok(s)
    }
}
