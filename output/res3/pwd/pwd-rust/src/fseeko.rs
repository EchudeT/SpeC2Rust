use std::io::{self, Seek, SeekFrom, Write};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S>(stream: &mut S, target: SeekFrom) -> io::Result<u64>
    where
        S: Seek + Write,
    {
        stream.flush()?;
        stream.seek(target)
    }
}
