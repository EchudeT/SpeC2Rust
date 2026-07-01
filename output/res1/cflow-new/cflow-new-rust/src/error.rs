use std::fmt;
use std::io::{self, Write};
use std::process;

pub struct Error;

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

        let err = io::Error::from_raw_os_error(errnum);
        let text = err.to_string();
        let suffix = if text.is_empty() {
            "Unknown system error"
        } else {
            text.as_str()
        };

        format!(": {suffix}")
    }

    pub fn gl_attribute_format_printf_standard(_message: &str) -> bool {
        true
    }

    pub fn report(status: Option<i32>, errnum: Option<i32>, args: fmt::Arguments<'_>) {
        Self::flush_stdout();

        let program = std::env::args()
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| String::from("program"));

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{program}: ");
        Self::tail_locked(&mut stderr, status, errnum.unwrap_or(0), args);
    }

    pub fn at_line(
        status: Option<i32>,
        errnum: Option<i32>,
        file_name: Option<&str>,
        line_number: u32,
        args: fmt::Arguments<'_>,
    ) {

        Self::flush_stdout();

        let program = std::env::args()
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| String::from("program"));

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{program}:");
        if let Some(file_name) = file_name {
            let _ = write!(stderr, "{file_name}:{line_number}: ");
        } else {
            let _ = write!(stderr, " ");
        }

        Self::tail_locked(&mut stderr, status, errnum.unwrap_or(0), args);
    }

    pub fn tail(status: Option<i32>, errnum: Option<i32>, args: fmt::Arguments<'_>) {
        let mut stderr = io::stderr().lock();
        Self::tail_locked(&mut stderr, status, errnum.unwrap_or(0), args);
    }

    fn tail_locked(
        stderr: &mut io::StderrLock<'_>,
        status: Option<i32>,
        errnum: i32,
        args: fmt::Arguments<'_>,
    ) {
        let _ = stderr.write_fmt(args);

        if errnum != 0 {
            let _ = write!(stderr, "{}", Self::print_errno_message(errnum));
        }

        let _ = writeln!(stderr);
        let _ = stderr.flush();

        if let Some(code) = status.filter(|code| *code != 0) {
            process::exit(code);
        }
    }
}
