pub struct Version;

impl Version {
    pub const CURRENT: &'static str = "9.5.42-bbc97";

    pub fn current() -> &'static str {
        Self::CURRENT
    }
}
