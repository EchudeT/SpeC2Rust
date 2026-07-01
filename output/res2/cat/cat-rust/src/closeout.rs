thread_local! {
    static STATE: RefCell<CloseoutState> = RefCell::new(CloseoutState::default());
}
use std::io::{self, Write};
use std::process;
use std::cell::RefCell;

pub struct Closeout;

#[derive(Default)]
struct CloseoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}

impl Closeout {
    fn with_state<R>(f: impl FnOnce(&mut CloseoutState) -> R) -> R {
        STATE.with(|state| f(&mut state.borrow_mut()))
    }

    fn current_state() -> (Option<String>, bool) {
        Self::with_state(|state| (state.file_name.clone(), state.ignore_epipe))
    }

    fn is_broken_pipe(error: &io::Error) -> bool {
        error.kind() == io::ErrorKind::BrokenPipe
    }

    fn fail_for_stdout_error(error: &io::Error, file_name: Option<&str>) -> ! {
        match file_name {
            Some(name) => eprintln!("{name}: write error: {error}"),
            None => eprintln!("write error: {error}"),
        }
        process::exit(1);
    }

    pub fn close_stdout_set_file_name(file: impl AsRef<str>) {
        let file = file.as_ref().to_owned();
        Self::with_state(|state| {
            state.file_name = Some(file);
        });
    }

    pub fn close_stdout_set_ignore_epipe(ignore: bool) {
        Self::with_state(|state| {
            state.ignore_epipe = ignore;
        });
    }

    pub fn close_stdout() {
        let (file_name, ignore_epipe) = Self::current_state();

        if let Err(error) = io::stdout().flush() {
            if !(ignore_epipe && Self::is_broken_pipe(&error)) {
                Self::fail_for_stdout_error(&error, file_name.as_deref());
            }
        }

        if io::stderr().flush().is_err() {
            process::exit(1);
        }
    }
}
