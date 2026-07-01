use std::io::{self, Write};

pub struct CloseStream;

impl CloseStream {
    pub fn close<W: Write>(mut stream: W) -> io::Result<()> {
        stream.flush()
    }
}
