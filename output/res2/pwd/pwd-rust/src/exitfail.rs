use std::process::ExitCode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Exitfail {
    code: u8,
}

impl Exitfail {
    pub const fn new(code: u8) -> Self {
        Self { code }
    }

    pub const fn failure() -> Self {
        Self { code: 1 }
    }

    pub const fn code(self) -> u8 {
        self.code
    }

    pub const fn as_exit_code(self) -> ExitCode {
        ExitCode::from(self.code)
    }
}

impl Default for Exitfail {
    fn default() -> Self {
        Self::failure()
    }
}

impl From<Exitfail> for ExitCode {
    fn from(value: Exitfail) -> Self {
        value.as_exit_code()
    }
}
