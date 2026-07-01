pub struct Version;

impl Version {
    pub const STRING: &'static str = "9.5.42-bbc97";

    pub fn as_str() -> &'static str {
        Self::STRING
    }
}
