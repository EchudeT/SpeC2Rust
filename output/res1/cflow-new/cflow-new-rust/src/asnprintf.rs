use std::fmt;

pub struct Asnprintf;

impl Asnprintf {
    pub fn format_to_string(resultbuf: Option<String>, _length: &mut usize, args: fmt::Arguments<'_>) -> String {
        let mut rendered = fmt::format(args);
        *_length = rendered.len();

        if let Some(mut buffer) = resultbuf {
            buffer.clear();
            buffer.push_str(&rendered);
            buffer
        } else {
            std::mem::take(&mut rendered)
        }
    }
}
