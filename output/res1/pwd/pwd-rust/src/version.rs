use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Version(&'static str);

impl Version {
    pub const fn new(text: &'static str) -> Self {
        Self(text)
    }

    pub const fn current() -> Self {
        Self("9.5.42-bbc97")
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::current()
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
