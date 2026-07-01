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
        let message = io::Error::from_raw_os_error(errnum).to_string();
        if message.is_empty() {
            "Unknown system error".to_string()
        } else {
            message
        }
    }

    pub fn gl_attribute_format_printf_standard() {}

    pub fn report(status: Option<i32>, errnum: Option<i32>, message: impl fmt::Display) {
        Self::flush_stdout();

        let program_name = std::env::args()
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "program".to_string());

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{program_name}: ");
        Self::tail(&mut stderr, status, errnum, &message.to_string());
    }

    pub fn at_line(
        status: Option<i32>,
        errnum: Option<i32>,
        file_name: Option<&str>,
        line_number: u32,
        message: impl fmt::Display,
    ) {

        Self::flush_stdout();

        let program_name = std::env::args()
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "program".to_string());

        let mut stderr = io::stderr().lock();
        let _ = write!(stderr, "{program_name}:");
        match file_name {
            Some(file_name) => {
                let _ = write!(stderr, "{file_name}:{line_number}: ");
            }
            None => {
                let _ = write!(stderr, " ");
            }
        }
        Self::tail(&mut stderr, status, errnum, &message.to_string());
    }

    pub fn tail(
        stderr: &mut dyn Write,
        status: Option<i32>,
        errnum: Option<i32>,
        message: &str,
    ) {
        let _ = write!(stderr, "{message}");

        if let Some(errnum) = errnum.filter(|value| *value != 0) {
            let _ = write!(stderr, ": {}", Self::print_errno_message(errnum));
        }

        let _ = writeln!(stderr);
        let _ = stderr.flush();

        if let Some(code) = status.filter(|value| *value != 0) {
            process::exit(code);
        }
    }
}
