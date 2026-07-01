pub struct Asnprintf;

impl Asnprintf {
    pub fn format_into(
        resultbuf: Option<String>,
        length: &mut usize,
        format_args: std::fmt::Arguments<'_>,
    ) -> String {
        let mut result = resultbuf.unwrap_or_default();
        result.clear();
        result.push_str(&format_args.to_string());
        *length = result.len();
        result
    }
}
