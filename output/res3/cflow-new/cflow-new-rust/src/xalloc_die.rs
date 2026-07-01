use crate::error::Error;
use std::process;

pub struct XallocDie;

impl XallocDie {
    pub fn die() -> ! {
        Error::report(Some(1), Some(0), "memory exhausted");
        process::abort();
    }
}
