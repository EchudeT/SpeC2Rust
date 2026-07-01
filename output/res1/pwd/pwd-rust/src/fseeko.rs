use std::io::{self, Seek, SeekFrom, Write};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S>(stream: &mut S, offset: i64, whence: SeekFrom) -> io::Result<u64>
    where
        S: Seek + Write,
    {
        match whence {
            SeekFrom::Start(_) => {
                let start = u64::try_from(offset).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "negative offset is invalid for SeekFrom::Start",
                    )
                })?;
                stream.seek(SeekFrom::Start(start))
            }
            SeekFrom::Current(_) => stream.seek(SeekFrom::Current(offset)),
            SeekFrom::End(_) => stream.seek(SeekFrom::End(offset)),
        }
    }

    pub fn seek_to<S>(stream: &mut S, position: u64) -> io::Result<u64>
    where
        S: Seek + Write,
    {
        Self::seek(stream, position as i64, SeekFrom::Start(0))
    }

    pub fn rewind<S>(stream: &mut S) -> io::Result<u64>
    where
        S: Seek + Write,
    {
        Self::seek(stream, 0, SeekFrom::Start(0))
    }
}
