use crate::error::Error;
use crate::exitfail::Exitfail;
use std::alloc::Layout;

pub struct XallocDie;

impl XallocDie {
    pub fn die() -> ! {
        Error::report(
            Some(Exitfail::status()),
            0,
            format_args!("memory exhausted"),
        );
        std::process::abort();
    }

    pub fn oom() -> ! {
        Self::die()
    }

    pub fn die_with_message(message: &str) -> ! {
        Error::report(Some(Exitfail::status()), 0, format_args!("{message}"));
        std::process::abort();
    }

    pub fn handle_alloc_error(layout: Layout) -> ! {
        Self::die_with_message(&format!(
            "memory exhausted while allocating {} bytes",
            layout.size()
        ))
    }
}
