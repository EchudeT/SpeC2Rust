use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};

#[cfg(not(unix))]
type RawFd = i32;

pub struct Dup2;

impl Dup2 {
    pub fn nothrow(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 || fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        if fd == desired_fd {
            return Ok(desired_fd);
        }

        Self::duplicate_to_target(fd, desired_fd)
    }

    #[cfg(windows)]
    pub fn ms_windows_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if fd == desired_fd {
            if Self::descriptor_exists(fd) {
                return Ok(fd);
            }
            return Err(io::Error::from_raw_os_error(9));
        }

        if desired_fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let result = Self::nothrow(fd, desired_fd)?;
        Ok(if result == 0 { desired_fd } else { result })
    }

    #[cfg(not(windows))]
    pub fn ms_windows_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        Self::nothrow(fd, desired_fd)
    }

    pub fn dirfd(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 || fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        Self::dirfd_recursive(fd, desired_fd)
    }

    pub fn klibc_dup_2(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        match Self::rpl_dup_2(fd, desired_fd) {
            Ok(v) => Ok(v),
            Err(err) => {
                if Self::is_enotsup(&err) && Self::descriptor_is_directory(fd) {
                    let _ = Self::close_target_path(desired_fd);
                    Self::dirfd(fd, desired_fd)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn rpl_dup_2(mut fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        if desired_fd < 0 {
            fd = desired_fd;
        }

        if fd == desired_fd {
            return if Self::descriptor_exists(fd) {
                Ok(fd)
            } else {
                Err(io::Error::from_raw_os_error(9))
            };
        }

        let result = Self::duplicate_to_target(fd, desired_fd);
        match result {
            Err(err) if err.raw_os_error() == Some(24) => Err(io::Error::from_raw_os_error(9)),
            other => other,
        }
    }

    fn duplicate_to_target(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        #[cfg(unix)]
        {
            let source = Self::fd_path(fd)?;
            let _ = Self::close_target_path(desired_fd);
            let file = OpenOptions::new().read(true).write(true).open(source)?;
            let duplicated = file.as_raw_fd();
            if duplicated == desired_fd {
                Ok(file.into_raw_fd())
            } else {
                let actual = file.into_raw_fd();
                let _ = actual;
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "dup2-style reassignment is not available in this safe Rust port",
                ))
            }
        }

        #[cfg(not(unix))]
        {
            let _ = fd;
            let _ = desired_fd;
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "dup2-style reassignment is not available on this platform in this safe Rust port",
            ))
        }
    }

    fn descriptor_exists(fd: RawFd) -> bool {
        Self::fd_path(fd).is_ok()
    }

    fn descriptor_is_directory(fd: RawFd) -> bool {
        Self::fd_path(fd)
            .ok()
            .and_then(|path| fs::metadata(path).ok())
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
    }

    fn is_enotsup(err: &io::Error) -> bool {
        matches!(err.raw_os_error(), Some(95) | Some(134))
            || err.kind() == io::ErrorKind::Unsupported
    }

    fn dirfd_recursive(fd: RawFd, desired_fd: RawFd) -> io::Result<RawFd> {
        let temp = Self::open_null_readonly()?;
        if temp == desired_fd {
            drop(temp);
            let path = Self::fd_path(fd)?;
            let reopened = OpenOptions::new().read(true).open(path)?;
            #[cfg(unix)]
            {
                return Ok(reopened.into_raw_fd());
            }
            #[cfg(not(unix))]
            {
                let _ = reopened;
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "directory descriptor reopening is unsupported on this platform",
                ));
            }
        }

        let result = Self::dirfd_recursive(fd, desired_fd);
        drop(temp);
        result
    }

    fn open_null_readonly() -> io::Result<std::fs::File> {
        #[cfg(windows)]
        {
            OpenOptions::new().read(true).open("NUL")
        }
        #[cfg(not(windows))]
        {
            OpenOptions::new().read(true).open("/dev/null")
        }
    }

    #[cfg(unix)]
    fn fd_path(fd: RawFd) -> io::Result<PathBuf> {
        let proc_path = PathBuf::from(format!("/proc/self/fd/{fd}"));
        fs::read_link(proc_path).or_else(|_| {
            let dev_path = PathBuf::from(format!("/dev/fd/{fd}"));
            fs::read_link(dev_path)
        })
    }

    #[cfg(not(unix))]
    fn fd_path(_fd: RawFd) -> io::Result<PathBuf> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "descriptor path resolution is unsupported on this platform",
        ))
    }

    fn close_target_path(_desired_fd: RawFd) -> io::Result<()> {
        Ok(())
    }
}
