use std::sync::atomic::{AtomicI32, Ordering};

static EXIT_FAILURE: AtomicI32 = AtomicI32::new(1);

pub struct Exitfail;

impl Exitfail {
    pub fn get() -> i32 {
        EXIT_FAILURE.load(Ordering::Relaxed)
    }

    pub fn set(status: i32) {
        EXIT_FAILURE.store(status, Ordering::Relaxed);
    }
}

pub fn exit_failure() -> i32 {
    Exitfail::get()
}

pub fn set_exit_failure(status: i32) {
    Exitfail::set(status);
}
