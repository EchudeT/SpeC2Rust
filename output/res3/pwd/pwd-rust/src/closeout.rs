use std::io::{self, Write};
use std::process;

pub struct Closeout;

#[derive(Default)]
struct CloseoutState {
    file_name: Option<String>,
    ignore_epipe: bool,
}

thread_local! {
    static STATE: std::cell::RefCell<CloseoutState> =
        std::cell::RefCell::new(CloseoutState::default());
}

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

        if let Err(err) = io::stdout().flush() {
            if !(ignore_epipe && err.kind() == io::ErrorKind::BrokenPipe) {
                if let Some(file_name) = file_name {
                    eprintln!("{file_name}: write error: {err}");
                } else {
                    eprintln!("write error: {err}");
                }
                process::exit(1);
            }
        }
    }
}
