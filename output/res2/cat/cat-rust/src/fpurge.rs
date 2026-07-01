use std::io::{self, BufRead, BufReader, Seek, Write};

pub struct Fpurge;

impl Fpurge {
    pub fn fpurge<S>(stream: &mut S) -> io::Result<()>
    where
        S: PurgeableStream,
    {
        stream.purge_buffered_state()
    }
}

pub trait PurgeableStream {
    fn purge_buffered_state(&mut self) -> io::Result<()>;
}

impl<R> PurgeableStream for BufReader<R>
where
    R: io::Read + Seek,
{
    fn purge_buffered_state(&mut self) -> io::Result<()> {
        self.consume(self.buffer().len());
        Ok(())
    }
}

impl PurgeableStream for std::fs::File {
    fn purge_buffered_state(&mut self) -> io::Result<()> {
        self.flush()
    }
}
