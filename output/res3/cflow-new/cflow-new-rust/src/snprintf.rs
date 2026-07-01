use std::fmt::{self, Write};

pub struct Snprintf;

impl Snprintf {
    pub fn format_into(buffer: &mut [u8], formatted: fmt::Arguments<'_>) -> Result<usize, fmt::Error> {
        let mut writer = BoundedBuffer::new(buffer);
        writer.write_fmt(formatted)?;
        writer.finish();
        Ok(writer.required_len())
    }
}

struct BoundedBuffer<'a> {
    buffer: &'a mut [u8],
    written: usize,
}

impl<'a> BoundedBuffer<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Self { buffer, written: 0 }
    }

    fn required_len(&self) -> usize {
        self.written
    }

    fn finish(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let nul_pos = self.written.min(self.buffer.len().saturating_sub(1));
        self.buffer[nul_pos] = 0;
    }
}

impl Write for BoundedBuffer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.written = self.written.saturating_add(s.len());

        if self.buffer.is_empty() {
            return Ok(());
        }

        let capacity = self.buffer.len() - 1;
        let already_copied = self.written.saturating_sub(s.len()).min(capacity);
        if already_copied >= capacity {
            return Ok(());
        }

        let remaining = capacity - already_copied;
        let to_copy = remaining.min(s.len());
        let end = already_copied + to_copy;
        self.buffer[already_copied..end].copy_from_slice(&s.as_bytes()[..to_copy]);
        Ok(())
    }
}
