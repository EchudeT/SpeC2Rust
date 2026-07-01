use std::process::ExitCode;

pub struct Exitfail;

const EXIT_FAILURE_STATUS: u8 = 1;

impl Exitfail {
    pub fn get() -> ExitCode {
        ExitCode::from(EXIT_FAILURE_STATUS)
    }

    pub fn code() -> u8 {
        EXIT_FAILURE_STATUS
    }

}
