pub struct C32Isprint;

impl C32Isprint {
    pub fn is_print(code_point: u32) -> bool {
        match char::from_u32(code_point) {
            Some(ch) => !ch.is_control(),
            None => false,
        }
    }
}
