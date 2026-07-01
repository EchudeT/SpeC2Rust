use crate::exitfail::Exitfail;
use std::process;

/// Handles fatal out-of-memory termination for allocation helpers.
pub struct XallocDie;

impl XallocDie {
    /// Terminates the process with a standard "memory exhausted" message.
    pub fn fail() -> ! {
        eprintln!("memory exhausted");
        let code = Exitfail::code();
        if code == 0 {
            process::abort();
        }
        process::exit(code as i32);
    }
}
