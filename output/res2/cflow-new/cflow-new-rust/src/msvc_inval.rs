use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidParameter;

pub struct MsvcInval;

thread_local! {
    static THREAD_STATE: RefCell<ThreadState> = RefCell::new(ThreadState::default());
}

static INSTALL_HANDLER: Once = Once::new();

#[derive(Default)]
struct ThreadState {
    restart_valid: bool,
}

impl MsvcInval {
    pub fn ensure_handler() {
        INSTALL_HANDLER.call_once(|| {});
    }

    pub fn with_invalid_parameter_handler<R, F>(operation: F) -> Result<R, InvalidParameter>
    where
        F: FnOnce() -> R,
    {
        Self::ensure_handler();

        THREAD_STATE.with(|state| {
            let previous = {
                let mut state = state.borrow_mut();
                let previous = state.restart_valid;
                state.restart_valid = true;
                previous
            };

            let result = panic::catch_unwind(AssertUnwindSafe(operation));

            {
                let mut state = state.borrow_mut();
                state.restart_valid = previous;
            }

            match result {
                Ok(value) => Ok(value),
                Err(payload) => {
                    if payload.is::<InvalidParameter>() {
                        Err(InvalidParameter)
                    } else {
                        panic::resume_unwind(payload)
                    }
                }
            }
        })
    }

    pub fn trigger_invalid_parameter() -> ! {
        let restart_valid = THREAD_STATE.with(|state| state.borrow().restart_valid);
        if restart_valid {
            panic::panic_any(InvalidParameter);
        }
        panic!("invalid parameter")
    }
}
