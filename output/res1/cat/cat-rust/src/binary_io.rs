use std::io;
use std::process;

pub struct BinaryIo;

impl BinaryIo {
    pub const TEXT_MODE: i32 = 0;
    pub const BINARY_MODE: i32 = 1;

    pub fn set_mode(fd: i32, mode: i32) -> io::Result<i32> {
        if Self::is_console_descriptor(fd) {
            return Ok(Self::TEXT_MODE);
        }

        Self::apply_mode(fd, mode)
    }

    fn is_console_descriptor(fd: i32) -> bool {
        match fd {
            0 => io::stdin().is_terminal(),
            1 => io::stdout().is_terminal(),
            2 => io::stderr().is_terminal(),
            _ => false,
        }
    }

    #[cfg(windows)]
    fn apply_mode(_fd: i32, mode: i32) -> io::Result<i32> {
        match mode {
            Self::TEXT_MODE | Self::BINARY_MODE => Ok(mode),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsupported file descriptor text/binary mode",
            )),
        }
    }

    #[cfg(not(windows))]
    fn apply_mode(_fd: i32, mode: i32) -> io::Result<i32> {
        Ok(mode)
    }

    pub fn set_binary(fd: i32) -> io::Result<()> {
        Self::set_mode(fd, Self::BINARY_MODE).map(|_| ())
    }

    pub fn set_binary_or_exit(fd: i32) -> ! {
        if let Err(error) = Self::set_binary(fd) {
            eprintln!("failed to set file descriptor text/binary mode: {error}");
            process::exit(1);
        }

        process::exit(0)
    }
}

use std::io::IsTerminal;
