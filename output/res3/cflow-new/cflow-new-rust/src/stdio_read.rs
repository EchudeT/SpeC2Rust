use std::io::{self, BufRead, Read};

pub struct StdioRead;

impl StdioRead {
    pub fn read_byte<R: Read>(reader: &mut R) -> io::Result<Option<u8>> {
        let mut byte = [0_u8; 1];
        match reader.read(&mut byte)? {
            0 => Ok(None),
            _ => Ok(Some(byte[0])),
        }
    }

    pub fn read_line<R: BufRead>(reader: &mut R, buffer: &mut String) -> io::Result<usize> {
        reader.read_line(buffer)
    }

    pub fn read_to_vec<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn read_exact_vec<R: Read>(reader: &mut R, len: usize) -> io::Result<Vec<u8>> {
        let mut data = vec![0_u8; len];
        reader.read_exact(&mut data)?;
        Ok(data)
    }

    pub fn read_into_slice<R: Read>(reader: &mut R, buffer: &mut [u8]) -> io::Result<usize> {
        reader.read(buffer)
    }

    pub fn read_all_text<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        Ok(text)
    }

    pub fn read_n_objects<R: Read>(
        reader: &mut R,
        object_size: usize,
        count: usize,
    ) -> io::Result<Vec<u8>> {
        let total = object_size.checked_mul(count).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "requested read size overflow")
        })?;

        let mut buffer = vec![0_u8; total];
        let mut filled = 0;

        while filled < total {
            let n = reader.read(&mut buffer[filled..])?;
            if n == 0 {
                buffer.truncate(filled);
                break;
            }
            filled += n;
        }

        Ok(buffer)
    }
}
