use std::io::{self, Write};
use std::process;

#[derive(Debug, Default)]
struct CloseoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}

thread_local! {
    static STATE: std::cell::RefCell<CloseoutState> =
        std::cell::RefCell::new(CloseoutState::default());
}

fn broken_pipe(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::BrokenPipe
}

fn flush_stdout() -> io::Result<()> {
    io::stdout().flush()
}

fn flush_stderr() -> io::Result<()> {
    io::stderr().flush()
}

fn with_state<R>(f: impl FnOnce(&CloseoutState) -> R) -> R {
    STATE.with(|state| f(&state.borrow()))
}

fn with_state_mut<R>(f: impl FnOnce(&mut CloseoutState) -> R) -> R {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

fn error_message(file_name: Option<&str>, err: &io::Error) -> String {
    let write_error = "write error";
    match file_name {
        Some(file_name) => format!("{file_name}: {write_error}: {err}"),
        None => format!("{write_error}: {err}"),
    }
}

fn close_stdout_result() -> io::Result<()> {
    if let Err(err) = flush_stdout() {
        let ignore = with_state(|state| state.ignore_epipe);
        if !(ignore && broken_pipe(&err)) {
            let message = with_state(|state| error_message(state.file_name.as_deref(), &err));
            let _ = writeln!(io::stderr(), "{message}");
            return Err(err);
        }
    }

    flush_stderr()
}

pub struct Closeout;

impl Closeout {
    pub fn close_stdout_set_file_name(file_name: impl Into<String>) {
        with_state_mut(|state| {
            state.file_name = Some(file_name.into());
        });
    }

    pub fn close_stdout_set_ignore_epipe(ignore: bool) {
        with_state_mut(|state| {
            state.ignore_epipe = ignore;
        });
    }

    pub fn close_stdout() {
        if close_stdout_result().is_err() {
            process::exit(1);
        }
    }

    pub fn main_root_close_stdout_06() {
        Self::close_stdout();
    }
}
