pub struct PrintfArgs {
    values: Vec<PrintfArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintfArg {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Char(char),
    Str(String),
    Pointer(usize),
    Count(usize),
}

impl PrintfArgs {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn push_signed(&mut self, value: i64) -> &mut Self {
        self.values.push(PrintfArg::Signed(value));
        self
    }

    pub fn push_unsigned(&mut self, value: u64) -> &mut Self {
        self.values.push(PrintfArg::Unsigned(value));
        self
    }

    pub fn push_float(&mut self, value: f64) -> &mut Self {
        self.values.push(PrintfArg::Float(value));
        self
    }

    pub fn push_char(&mut self, value: char) -> &mut Self {
        self.values.push(PrintfArg::Char(value));
        self
    }

    pub fn push_str(&mut self, value: impl Into<String>) -> &mut Self {
        self.values.push(PrintfArg::Str(value.into()));
        self
    }

    pub fn push_pointer(&mut self, value: usize) -> &mut Self {
        self.values.push(PrintfArg::Pointer(value));
        self
    }

    pub fn push_count(&mut self, value: usize) -> &mut Self {
        self.values.push(PrintfArg::Count(value));
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn get(&self, index: usize) -> Option<&PrintfArg> {
        self.values.get(index)
    }

    pub fn as_slice(&self) -> &[PrintfArg] {
        &self.values
    }

    pub fn into_vec(self) -> Vec<PrintfArg> {
        self.values
    }
}

impl Default for PrintfArgs {
    fn default() -> Self {
        Self::new()
    }
}
