use std::io::{self, BufRead, Read};

pub struct StdioRead;

impl StdioRead {
    pub fn read_char<R: BufRead>(reader: &mut R) -> io::Result<Option<char>> {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            return Ok(None);
        }

        let width = utf8_char_width(buf[0]);
        if width == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid UTF-8 leading byte",
            ));
        }

        if buf.len() < width {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incomplete UTF-8 character",
            ));
        }

        let s = std::str::from_utf8(&buf[..width])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8 sequence"))?;
        let ch = s.chars().next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "failed to decode character")
        })?;
        reader.consume(width);
        Ok(Some(ch))
    }

    pub fn read_line<R: BufRead>(reader: &mut R, target: &mut String) -> io::Result<usize> {
        reader.read_line(target)
    }

    pub fn read_exact_or_to_end<R: Read>(
        reader: &mut R,
        buffer: &mut [u8],
    ) -> io::Result<usize> {
        let mut total = 0;
        while total < buffer.len() {
            let n = reader.read(&mut buffer[total..])?;
            if n == 0 {
                break;
            }
            total += n;
        }
        Ok(total)
    }

    pub fn read_all<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn read_to_string<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        Ok(s)
    }
}

fn utf8_char_width(first: u8) -> usize {
    match first {
        0x00..=0x7f => 1,
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => 0,
    }
}
