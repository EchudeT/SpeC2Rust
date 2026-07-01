use std::fmt;

pub struct Snprintf;

impl Snprintf {
    pub fn write(buffer: &mut [u8], args: fmt::Arguments<'_>) -> Result<usize, fmt::Error> {
        let rendered = fmt::format(args);
        let bytes = rendered.as_bytes();
        let len = bytes.len();

        if buffer.is_empty() {
            return Ok(len);
        }

        let pruned_len = len.min(buffer.len().saturating_sub(1));
        if pruned_len > 0 {
            buffer[..pruned_len].copy_from_slice(&bytes[..pruned_len]);
        }
        buffer[pruned_len] = 0;

        Ok(len)
    }
}
