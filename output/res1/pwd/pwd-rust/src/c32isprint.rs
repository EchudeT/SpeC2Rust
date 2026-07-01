pub struct C32Isprint;

impl C32Isprint {
    pub fn is_print(codepoint: u32) -> bool {
        char::from_u32(codepoint).is_some_and(|ch| !ch.is_control())
    }
}
