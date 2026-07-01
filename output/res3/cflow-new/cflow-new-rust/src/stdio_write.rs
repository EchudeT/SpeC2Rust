use std::fmt;
use std::io::{self, Write};

pub struct StdioWrite;

impl StdioWrite {
    pub fn write_byte<W: Write>(writer: &mut W, byte: u8) -> io::Result<()> {
        writer.write_all(&[byte])
    }

    pub fn write_str<W: Write>(writer: &mut W, text: &str) -> io::Result<usize> {
        writer.write_all(text.as_bytes())?;
        Ok(text.len())
    }

    pub fn write_bytes<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<usize> {
        writer.write_all(bytes)?;
        Ok(bytes.len())
    }

    pub fn write_repeated<W: Write>(writer: &mut W, byte: u8, count: usize) -> io::Result<usize> {
        if count == 0 {
            return Ok(0);
        }

        let buffer = vec![byte; count];
        writer.write_all(&buffer)?;
        Ok(count)
    }

    pub fn write_line<W: Write>(writer: &mut W, text: &str) -> io::Result<usize> {
        writer.write_all(text.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(text.len() + 1)
    }

    pub fn print<W: Write>(writer: &mut W, args: fmt::Arguments<'_>) -> io::Result<usize> {
        let rendered = fmt::format(args);
        writer.write_all(rendered.as_bytes())?;
        Ok(rendered.len())
    }

    pub fn print_line<W: Write>(writer: &mut W, args: fmt::Arguments<'_>) -> io::Result<usize> {
        let rendered = fmt::format(args);
        writer.write_all(rendered.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(rendered.len() + 1)
    }

    pub fn flush<W: Write>(writer: &mut W) -> io::Result<()> {
        writer.flush()
    }
}
