use std::process;

pub struct Exitfail;

impl Exitfail {
    pub const DEFAULT: i32 = 1;

    pub fn status() -> i32 {
        Self::DEFAULT
    }

    pub fn set_status(_status: i32) {
    }

    pub fn reset() {
        Self::set_status(Self::DEFAULT);
    }

    pub fn exit() -> ! {
        process::exit(Self::status());
    }

    pub fn exit_with(status: i32) -> ! {
        process::exit(status);
    }
}
