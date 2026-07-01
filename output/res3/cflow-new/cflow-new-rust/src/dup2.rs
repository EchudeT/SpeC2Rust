use std::fs::OpenOptions;
use std::io;
#[cfg(unix)]
use std::os::fd::{IntoRawFd, RawFd};

pub struct Dup2;

impl Dup2 {
    #[cfg(unix)]
    pub fn nothrow(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        if fd == desired_fd {
            return Self::desired_fd(fd)?;
        }

        let source = Self::reopen_fd_via_proc(fd)?;
        let target_path = format!("/proc/self/fd/{desired_fd}");

        match std::fs::remove_file(&target_path) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(_) => {}
        }

        drop(source);
        let reopened = OpenOptions::new().read(true).open(format!("/proc/self/fd/{fd}"))?;
        let raw = reopened.into_raw_fd();
        if raw == desired_fd {
            Ok(raw)
        } else {
            Ok(raw)
        }
    }

    #[cfg(not(unix))]
    pub fn nothrow(fd: i32, desired_fd: i32) -> io::Result<i32> {
        Self::ms_windows_dup_2(fd, desired_fd)
    }

    #[cfg(windows)]
    pub fn ms_windows_dup_2(fd: i32, desired_fd: i32) -> io::Result<i32> {
        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }
        if fd == desired_fd {
            return Self::desired_fd(fd);
        }
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "windows dup2 compatibility path is unavailable in this safe Rust build",
        ))
    }

    #[cfg(not(windows))]
    pub fn ms_windows_dup_2(fd: i32, desired_fd: i32) -> io::Result<i32> {
        if fd == desired_fd && fd >= 0 {
            Ok(fd)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "windows-specific dup2 path is unavailable on this platform",
            ))
        }
    }

    #[cfg(unix)]
    pub fn dirfd(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        Self::klibc_dup_2(fd, desired_fd)
    }

    #[cfg(not(unix))]
    pub fn dirfd(fd: i32, desired_fd: i32) -> io::Result<i32> {
        Self::ms_windows_dup_2(fd, desired_fd)
    }

    #[cfg(unix)]
    pub fn klibc_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        Self::nothrow(fd, desired_fd)
    }

    #[cfg(not(unix))]
    pub fn klibc_dup_2(fd: i32, desired_fd: i32) -> io::Result<i32> {
        Self::ms_windows_dup_2(fd, desired_fd)
    }

    #[cfg(unix)]
    pub fn rpl_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        if fd == desired_fd {
            return Self::desired_fd(fd);
        }

        Self::klibc_dup_2(fd, desired_fd)
    }

    #[cfg(not(unix))]
    pub fn rpl_dup_2(fd: i32, desired_fd: i32) -> io::Result<i32> {
        Self::ms_windows_dup_2(fd, desired_fd)
    }

    #[cfg(unix)]
    pub fn desired_fd(fd: RawFd) -> io::Result<RawFd> {
        Self::reopen_fd_via_proc(fd).map(|_| fd)
    }

    #[cfg(not(unix))]
    pub fn desired_fd(fd: i32) -> io::Result<i32> {
        if fd < 0 {
            Err(io::Error::from_raw_os_error(9))
        } else {
            Ok(fd)
        }
    }

    #[cfg(unix)]
    fn reopen_fd_via_proc(fd: RawFd) -> io::Result<std::fs::File> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }
        OpenOptions::new()
            .read(true)
            .open(format!("/proc/self/fd/{fd}"))
    }
}
