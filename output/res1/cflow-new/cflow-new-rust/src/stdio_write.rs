use std::fmt;
use std::io::{self, Write};

pub struct StdioWrite;

impl StdioWrite {
    pub fn write_byte<W: Write>(writer: &mut W, byte: u8) -> io::Result<()> {
        writer.write_all(&[byte])
    }

    pub fn write_str<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
        writer.write_all(text.as_bytes())
    }

    pub fn write_all<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<usize> {
        writer.write_all(bytes)?;
        Ok(bytes.len())
    }

    pub fn write_items<W: Write>(
        writer: &mut W,
        bytes: &[u8],
        item_size: usize,
        item_count: usize,
    ) -> io::Result<usize> {
        if item_size == 0 || item_count == 0 {
            return Ok(0);
        }

        let total = item_size
            .checked_mul(item_count)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "item count overflow"))?;

        if total > bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "buffer shorter than requested item span",
            ));
        }

        writer.write_all(&bytes[..total])?;
        Ok(item_count)
    }

    pub fn write_line<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
        writer.write_all(text.as_bytes())?;
        writer.write_all(b"\n")
    }

    pub fn print<W: Write>(writer: &mut W, args: fmt::Arguments<'_>) -> io::Result<()> {
        writer.write_fmt(args)
    }

    pub fn print_line<W: Write>(writer: &mut W, args: fmt::Arguments<'_>) -> io::Result<()> {
        writer.write_fmt(args)?;
        writer.write_all(b"\n")
    }

    pub fn flush<W: Write>(writer: &mut W) -> io::Result<()> {
        writer.flush()
    }
}
