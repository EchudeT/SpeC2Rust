use std::io::{self, Write};

pub struct CloseStream;

impl CloseStream {
    pub fn close<W>(mut stream: W) -> io::Result<()>
    where
        W: Write,
    {
        stream.flush()
    }
}
