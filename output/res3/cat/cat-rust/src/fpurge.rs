use std::io::{self, Seek, SeekFrom, Write};

pub struct Fpurge;

impl Fpurge {
    pub fn fpurge<S>(stream: &mut S) -> io::Result<()>
    where
        S: Write + Seek,
    {
        stream.flush()?;
        let position = stream.stream_position()?;
        stream.seek(SeekFrom::Start(position))?;
        Ok(())
    }
}
