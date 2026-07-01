use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};

pub struct Dup2;

impl Dup2 {
    pub fn nothrow(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        Self::rpl_dup_2(fd, desired_fd)
    }

    pub fn ms_windows_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if fd == desired_fd {
            return Self::dirfd(fd).map(|_| fd);
        }

        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let result = Self::nothrow(fd, desired_fd)?;
        if result == 0 {
            Ok(desired_fd)
        } else {
            Ok(result)
        }
    }

    pub fn dirfd(fd: RawFd) -> io::Result<RawFd> {
        if fd < 0 {
            Err(io::Error::from_raw_os_error(9))
        } else {
            Ok(fd)
        }
    }

    pub fn klibc_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        Self::rpl_dup_2(fd, desired_fd)
    }

    pub fn rpl_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        if fd == desired_fd {
            return Self::dirfd(fd).map(|_| fd);
        }

        let path = format!("/proc/self/fd/{fd}");
        let file = OpenOptions::new().read(true).open(path)?;

        let duplicated = Self::duplicate_to_min_at_least(file, desired_fd)?;
        if duplicated.as_raw_fd() != desired_fd {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "requested descriptor number could not be reproduced safely",
            ));
        }

        Ok(duplicated.into_raw_fd())
    }

    fn duplicate_to_min_at_least(file: File, desired_fd: RawFd) -> io::Result<File> {
        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let current = file.as_raw_fd();
        if current == desired_fd {
            return Ok(file);
        }

        let _ = file;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "portable descriptor renumbering is unavailable in this Rust-only port",
        ))
    }
}
