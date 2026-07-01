use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};

thread_local! {
    static INVALID_PARAMETER_ACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Rust-style wrapper for the per-thread invalid-parameter handling behavior
/// used by the original `msvc_inval` support code.
///
/// In the C implementation this logic installs an MSVC invalid-parameter
/// handler and tracks whether the current thread is already inside that
/// handler, so nested failures can avoid looping or re-entering recursively.
///
/// This Rust rewrite exposes the behavior as a small per-thread guard and
/// execution helper, without any C ABI surface.
pub struct MsvcInval;

impl MsvcInval {
    /// Ensures the module is ready for use.
    ///
    /// In Rust there is no process-global MSVC invalid-parameter hook to
    /// install here, so readiness is represented by the availability of the
    /// per-thread state managed by this module.
    pub fn ensure_handler() {
        INVALID_PARAMETER_ACTIVE.with(|_| {});
    }

    /// Returns whether the current thread is presently inside an invalid
    /// parameter handling scope.
    pub fn is_handling() -> bool {
        INVALID_PARAMETER_ACTIVE.with(Cell::get)
    }

    /// Executes `f` while marking the current thread as handling an invalid
    /// parameter condition.
    ///
    /// If the thread is already in such a scope, `f` is executed directly and
    /// the nested call is reported by returning `true` in the tuple.
    ///
    /// The first tuple element is the closure result, and the second indicates
    /// whether the call was nested inside an already-active handling scope.
    pub fn with_handler<F, R>(f: F) -> (R, bool)
    where
        F: FnOnce() -> R,
    {
        Self::ensure_handler();

        INVALID_PARAMETER_ACTIVE.with(|flag| {
            let already_active = flag.get();
            if already_active {
                return (f(), true);
            }

            flag.set(true);
            let result = catch_unwind(AssertUnwindSafe(f));
            flag.set(false);

            match result {
                Ok(value) => (value, false),
                Err(payload) => std::panic::resume_unwind(payload),
            }
        })
    }

    /// Marks the current thread as actively handling an invalid parameter
    /// condition and returns a guard that clears the mark when dropped.
    ///
    /// The returned boolean indicates whether handling was already active
    /// before this call.
    pub fn enter() -> InvalGuard {
        Self::ensure_handler();

        let nested = INVALID_PARAMETER_ACTIVE.with(|flag| {
            let nested = flag.get();
            if !nested {
                flag.set(true);
            }
            nested
        });

        InvalGuard { nested }
    }
}

/// RAII guard for `MsvcInval::enter`.
pub struct InvalGuard {
    nested: bool,
}

impl InvalGuard {
    /// Returns `true` if the thread was already inside an invalid-parameter
    /// handling scope when this guard was created.
    pub fn is_nested(&self) -> bool {
        self.nested
    }
}

impl Drop for InvalGuard {
    fn drop(&mut self) {
        if !self.nested {
            INVALID_PARAMETER_ACTIVE.with(|flag| flag.set(false));
        }
    }
}
