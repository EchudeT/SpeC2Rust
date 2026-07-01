use std::fmt::{self, Write as _};

pub struct Snprintf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnprintfError;

impl Snprintf {
    pub fn format_into(buffer: &mut [u8], format_args: fmt::Arguments<'_>) -> Result<usize, SnprintfError> {
        let mut rendered = String::new();
        rendered
            .write_fmt(format_args)
            .map_err(|_| SnprintfError)?;

        let bytes = rendered.as_bytes();
        let len = bytes.len();

        if !buffer.is_empty() {
            let copy_len = len.min(buffer.len().saturating_sub(1));
            if copy_len > 0 {
                buffer[..copy_len].copy_from_slice(&bytes[..copy_len]);
            }
            buffer[copy_len] = 0;
        }

        Ok(len)
    }
}
