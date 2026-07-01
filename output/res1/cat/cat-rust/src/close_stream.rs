use std::io::{self, BufWriter, Write};

pub struct CloseStream;

impl CloseStream {
    pub fn close_writer<W: Write>(writer: W) -> io::Result<()> {
        let mut writer = BufWriter::new(writer);
        match writer.flush() {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
