pub struct C32Isprint;

impl C32Isprint {
    pub fn is_print(codepoint: u32) -> bool {
        match char::from_u32(codepoint) {
            Some(ch) => !ch.is_control(),
            None => false,
        }
    }
}
