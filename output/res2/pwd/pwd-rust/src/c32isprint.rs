pub struct C32Isprint;

impl C32Isprint {
    pub fn is_print(c: char) -> bool {
        !c.is_control()
    }

    pub fn is_print_u32(value: u32) -> bool {
        char::from_u32(value).is_some_and(Self::is_print)
    }
}
