use std::io::{self, Seek, SeekFrom};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S>(stream: &mut S, offset: i64, whence: SeekFrom) -> io::Result<u64>
    where
        S: Seek,
    {
        let target = match whence {
            SeekFrom::Start(_) => {
                let absolute = u64::try_from(offset).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "negative offset is invalid for SeekFrom::Start",
                    )
                })?;
                SeekFrom::Start(absolute)
            }
            SeekFrom::Current(_) => SeekFrom::Current(offset),
            SeekFrom::End(_) => SeekFrom::End(offset),
        };

        stream.seek(target)
    }

    pub fn seek_to<S>(stream: &mut S, target: SeekFrom) -> io::Result<u64>
    where
        S: Seek,
    {
        stream.seek(target)
    }

    pub fn rewind<S>(stream: &mut S) -> io::Result<()>
    where
        S: Seek,
    {
        stream.seek(SeekFrom::Start(0)).map(|_| ())
    }

    pub fn position<S>(stream: &mut S) -> io::Result<u64>
    where
        S: Seek,
    {
        stream.stream_position()
    }
}
