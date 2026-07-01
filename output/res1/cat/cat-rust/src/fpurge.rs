use std::io::{self, BufRead, Seek, SeekFrom, Write};

pub struct Fpurge;

impl Fpurge {
    pub fn fpurge<S>(stream: &mut S) -> io::Result<()>
    where
        S: BufRead + Write + Seek,
    {
        let position = stream.stream_position()?;
        stream.seek(SeekFrom::Start(position))?;
        stream.flush()?;
        Ok(())
    }
}
