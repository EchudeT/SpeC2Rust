use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum PrintfArg {
    Int(i64),
    UInt(u64),
    Float(f64),
    Char(char),
    Str(String),
    Bool(bool),
}

impl fmt::Display for PrintfArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{v}"),
            Self::UInt(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Char(v) => write!(f, "{v}"),
            Self::Str(v) => f.write_str(v),
            Self::Bool(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PrintfArgs {
    args: Vec<PrintfArg>,
}

impl PrintfArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            args: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, arg: PrintfArg) {
        self.args.push(arg);
    }

    pub fn push_int(&mut self, value: i64) {
        self.push(PrintfArg::Int(value));
    }

    pub fn push_uint(&mut self, value: u64) {
        self.push(PrintfArg::UInt(value));
    }

    pub fn push_float(&mut self, value: f64) {
        self.push(PrintfArg::Float(value));
    }

    pub fn push_char(&mut self, value: char) {
        self.push(PrintfArg::Char(value));
    }

    pub fn push_str(&mut self, value: impl Into<String>) {
        self.push(PrintfArg::Str(value.into()));
    }

    pub fn push_bool(&mut self, value: bool) {
        self.push(PrintfArg::Bool(value));
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&PrintfArg> {
        self.args.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, PrintfArg> {
        self.args.iter()
    }

    pub fn into_vec(self) -> Vec<PrintfArg> {
        self.args
    }

    pub fn clear(&mut self) {
        self.args.clear();
    }
}

impl IntoIterator for PrintfArgs {
    type Item = PrintfArg;
    type IntoIter = std::vec::IntoIter<PrintfArg>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}

impl<'a> IntoIterator for &'a PrintfArgs {
    type Item = &'a PrintfArg;
    type IntoIter = std::slice::Iter<'a, PrintfArg>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.iter()
    }
}
