use crate::error::Error;
use std::process;

pub struct XallocDie;

impl XallocDie {
    pub fn die() -> ! {
        Error::report(Some(1), Some(0), format_args!("memory exhausted"));
        process::abort();
    }

    pub fn from_main_context() -> ! {
        Error::report(Some(1), Some(12), format_args!("Exiting"));
        process::abort();
    }
}
