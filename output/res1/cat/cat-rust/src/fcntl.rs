use std::env;
use std::ffi::OsString;
use std::io;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::process::ExitCode;

pub struct Fcntl;

impl Fcntl {
    pub fn duplicate_fd_at_least(fd: RawFd, minimum: RawFd) -> io::Result<OwnedFd> {
        if fd < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file descriptor must be non-negative",
            ));
        }

        if minimum < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "minimum file descriptor must be non-negative",
            ));
        }

        let proc_path = format!("/proc/self/fd/{fd}");
        let duplicated = std::fs::OpenOptions::new().read(true).open(proc_path)?;
        let owned: OwnedFd = duplicated.into();

        if owned.as_raw_fd() < minimum {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "duplicating to a descriptor at or above a minimum is unavailable without platform-specific support",
            ));
        }

        Ok(owned)
    }

    pub fn duplicate_close_on_exec(fd: RawFd, minimum: RawFd) -> io::Result<OwnedFd> {
        Self::duplicate_fd_at_least(fd, minimum)
    }

    pub fn klibc_fcntl(fd: RawFd, action: i32, arg: i32) -> io::Result<i32> {
        match action {
            0 => {
                let duplicated = Self::duplicate_fd_at_least(fd, arg)?;
                Ok(duplicated.as_raw_fd())
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsupported fcntl action",
            )),
        }
    }

    pub fn args_os() -> Vec<OsString> {
        env::args_os().collect()
    }

    pub fn run_14() -> ExitCode {
        ExitCode::SUCCESS
    }
}
