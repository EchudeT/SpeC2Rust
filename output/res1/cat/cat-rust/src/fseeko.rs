use std::io::{self, Seek, SeekFrom};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S: Seek>(stream: &mut S, offset: i64, whence: SeekFrom) -> io::Result<u64> {
        let target = Self::target_from(offset, whence)?;
        stream.seek(target)
    }

    fn target_from(offset: i64, whence: SeekFrom) -> io::Result<SeekFrom> {
        match whence {
            SeekFrom::Start(_) => {
                let absolute = u64::try_from(offset).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "negative offset is invalid for seek-from-start",
                    )
                })?;
                Ok(SeekFrom::Start(absolute))
            }
            SeekFrom::Current(_) => Ok(SeekFrom::Current(offset)),
            SeekFrom::End(_) => Ok(SeekFrom::End(offset)),
        }
    }
}
