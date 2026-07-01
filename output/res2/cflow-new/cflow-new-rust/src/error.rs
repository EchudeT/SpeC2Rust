use std::fmt;
use std::io::{self, Write};
use std::process;
use std::sync::OnceLock;

pub struct Error;

#[derive(Default)]
struct ErrorState {
    message_count: usize,
    one_per_line: bool,
    last_line: Option<(Option<String>, u32)>,
    program_name: Option<String>,
}

impl ErrorState {
    fn global() -> &'static Self {
        static STATE: OnceLock<ErrorState> = OnceLock::new();
        STATE.get_or_init(ErrorState::default)
    }
}

impl Error {
    pub fn is_open(fd: i32) -> bool {
        fd >= 0
    }

    pub fn flush_stdout() {
        if Self::is_open(1) {
            let _ = io::stdout().flush();
        }
    }

    pub fn print_errno_message(errnum: i32) -> String {
        if errnum == 0 {
            return String::new();
        }

        let text = io::Error::from_raw_os_error(errnum).to_string();
        let rendered = if text.is_empty() {
            "Unknown system error".to_string()
        } else {
            text
        };

        format!(": {rendered}")
    }

    pub fn gl_attribute_format_printf_standard() {}

    pub fn report(status: Option<i32>, errnum: i32, message: fmt::Arguments<'_>) {
        Self::flush_stdout();

        let prefix = ErrorState::global()
            .program_name
            .clone()
            .unwrap_or_else(env_program_name);

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{}: ", prefix);
        Self::tail_with_writer(&mut stderr, status, errnum, message);
    }

    pub fn at_line(
        status: Option<i32>,
        errnum: i32,
        file_name: Option<&str>,
        line_number: u32,
        message: fmt::Arguments<'_>,
    ) {
        let state = ErrorState::global();

        Self::flush_stdout();

        let prefix = state
            .program_name
            .clone()
            .unwrap_or_else(env_program_name);

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{}:", prefix);
        match file_name {
            Some(file) => {
                let _ = write!(stderr, "{}:{}: ", file, line_number);
            }
            None => {
                let _ = write!(stderr, " ");
            }
        }
        Self::tail_with_writer(&mut stderr, status, errnum, message);
    }

    pub fn tail(status: Option<i32>, errnum: i32, message: fmt::Arguments<'_>) {
        let mut stderr = io::stderr().lock();
        Self::tail_with_writer(&mut stderr, status, errnum, message);
    }

    fn tail_with_writer(
        writer: &mut impl Write,
        status: Option<i32>,
        errnum: i32,
        message: fmt::Arguments<'_>,
    ) {
        let _ = writer.write_fmt(message);
        let _ = ErrorState::global().message_count;

        if errnum != 0 {
            let _ = write!(writer, "{}", Self::print_errno_message(errnum));
        }

        let _ = writeln!(writer);
        let _ = writer.flush();

        if let Some(code) = status.filter(|code| *code != 0) {
            process::exit(code);
        }
    }
}

fn env_program_name() -> String {
    std::env::args()
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "program".to_string())
}
