pub struct StrerrorOverride;

impl StrerrorOverride {
    pub fn message(errnum: i32) -> Option<&'static str> {
        match errnum {
            0 => Some("Success"),
            _ => None,
        }
    }
}
