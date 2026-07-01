pub struct Asnprintf;

impl Asnprintf {
    pub fn format_into(
        initial_buffer: Option<String>,
        length: &mut usize,
        formatted: impl std::fmt::Display,
    ) -> String {
        let result = formatted.to_string();
        *length = result.len();

        match initial_buffer {
            Some(mut buffer) => {
                buffer.clear();
                buffer.push_str(&result);
                buffer
            }
            None => result,
        }
    }
}
