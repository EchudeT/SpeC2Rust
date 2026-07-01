pub struct XallocDie;

impl XallocDie {
    pub fn fail() -> ! {
        eprintln!("memory exhausted");
        std::process::abort();
    }

    pub fn xalloc_die() -> ! {
        Self::fail()
    }
}
