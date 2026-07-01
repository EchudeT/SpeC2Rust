pub struct XallocDie;

impl XallocDie {
    pub fn die() -> ! {
        eprintln!("memory exhausted");
        std::process::abort();
    }
}
