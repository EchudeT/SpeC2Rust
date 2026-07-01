use std::io::{self, Seek, SeekFrom};

pub struct Fseeko;

impl Fseeko {
    pub fn seek<S>(stream: &mut S, position: SeekFrom) -> io::Result<u64>
    where
        S: Seek,
    {
        stream.seek(position)
    }

    pub fn seek_after_flush<S>(stream: &mut S, position: SeekFrom) -> io::Result<u64>
    where
        S: Seek,
    {
        stream.seek(position)
    }

    pub fn rewind<S>(stream: &mut S) -> io::Result<u64>
    where
        S: Seek,
    {
        stream.seek(SeekFrom::Start(0))
    }
}
