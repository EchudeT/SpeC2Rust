use std::process;

pub struct XallocDie;

impl XallocDie {
    pub fn fail() -> ! {
        eprintln!("memory exhausted");
        process::abort();
    }
}
