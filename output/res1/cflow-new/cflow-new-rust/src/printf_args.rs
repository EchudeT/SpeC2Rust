use std::fmt;

/// Collected arguments for a Rust-style printf-like formatting pipeline.
///
/// This module intentionally provides a safe Rust representation rather than
/// mirroring the original C varargs machinery. It can be used as a generic
/// container for positional arguments discovered during format parsing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PrintfArgs {
    values: Vec<PrintfArg>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PrintfArg {
    Signed(i64),
    Unsigned(u64),
    Text(String),
    Char(char),
    Bool(bool),
}

impl PrintfArgs {
    /// Creates an empty argument list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty argument list with reserved capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of stored arguments.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` when no arguments are stored.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Removes all stored arguments.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Appends a signed integer argument.
    pub fn push_signed(&mut self, value: i64) {
        self.values.push(PrintfArg::Signed(value));
    }

    /// Appends an unsigned integer argument.
    pub fn push_unsigned(&mut self, value: u64) {
        self.values.push(PrintfArg::Unsigned(value));
    }

    /// Appends a string argument.
    pub fn push_string(&mut self, value: impl Into<String>) {
        self.values.push(PrintfArg::Text(value.into()));
    }

    /// Appends a character argument.
    pub fn push_char(&mut self, value: char) {
        self.values.push(PrintfArg::Char(value));
    }

    /// Appends a boolean argument.
    pub fn push_bool(&mut self, value: bool) {
        self.values.push(PrintfArg::Bool(value));
    }

    /// Returns a displayable view of the argument at `index`.
    pub fn get(&self, index: usize) -> Option<impl fmt::Display + '_> {
        self.values.get(index).map(DisplayArg)
    }

    /// Iterates over displayable argument views.
    pub fn iter(&self) -> impl Iterator<Item = impl fmt::Display + '_> {
        self.values.iter().map(DisplayArg)
    }
}

struct DisplayArg<'a>(&'a PrintfArg);

impl fmt::Display for DisplayArg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            PrintfArg::Signed(v) => write!(f, "{v}"),
            PrintfArg::Unsigned(v) => write!(f, "{v}"),
            PrintfArg::Text(v) => f.write_str(v),
            PrintfArg::Char(v) => write!(f, "{v}"),
            PrintfArg::Bool(v) => write!(f, "{v}"),
        }
    }
}
