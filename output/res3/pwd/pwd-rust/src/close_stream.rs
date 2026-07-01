use std::io::{self, Write};

pub struct CloseStream;

impl CloseStream {
    pub fn close<W: Write>(stream: &mut W) -> io::Result<()> {
        stream.flush()
    }
}
