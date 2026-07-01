use std::cell::RefCell;
use std::io::{self, Write};
use std::process;

#[derive(Default)]
struct CloseoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}

thread_local! {
    static STATE: RefCell<CloseoutState> = RefCell::new(CloseoutState::default());
}

fn should_ignore_error(error: &io::Error, ignore_epipe: bool) -> bool {
    ignore_epipe && error.kind() == io::ErrorKind::BrokenPipe
}

fn report_write_error(file_name: Option<&str>, error: &io::Error) {
    match file_name {
        Some(name) => eprintln!("{name}: write error: {error}"),
        None => eprintln!("write error: {error}"),
    }
}

pub struct Closeout;

impl Closeout {
    pub fn close_stdout_set_file_name(file: impl Into<String>) {
        STATE.with(|state| {
            state.borrow_mut().file_name = Some(file.into());
        });
    }

    pub fn close_stdout_set_ignore_epipe(ignore: bool) {
        STATE.with(|state| {
            state.borrow_mut().ignore_epipe = ignore;
        });
    }

    pub fn close_stdout() {
        let (file_name, ignore_epipe) = STATE.with(|state| {
            let state = state.borrow();
            (state.file_name.clone(), state.ignore_epipe)
        });

        if let Err(error) = io::stdout().flush() {
            if !should_ignore_error(&error, ignore_epipe) {
                report_write_error(file_name.as_deref(), &error);
                process::exit(1);
            }
        }

        if io::stderr().flush().is_err() {
            process::exit(1);
        }
    }
}
