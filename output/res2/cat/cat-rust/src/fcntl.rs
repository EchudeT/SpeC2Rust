use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};

pub struct Fcntl;

impl Fcntl {
    pub fn duplicate_fd_from(fd: RawFd, target: RawFd) -> io::Result<OwnedFd> {
        Self::duplicate_from_min(fd, target, false)
    }

    pub fn dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<OwnedFd> {
        Self::duplicate_from_min(fd, target, true)
    }

    pub fn klibc_fcntl(fd: RawFd, action: i32, arg: i32) -> io::Result<i32> {
        match action {
            0 => {
                let duplicated = Self::duplicate_fd_from(fd, arg)?;
                Ok(duplicated.as_raw_fd())
            }
            1 => {
                let file = Self::file_from_fd(fd)?;
                let metadata = file.metadata()?;
                #[cfg(unix)]
                {
                    if metadata.file_type().is_dir() {
                        Ok(0)
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::Unsupported,
                            "fcntl action is not supported for this descriptor",
                        ))
                    }
                }
                #[cfg(not(unix))]
                {
                    let _ = metadata;
                    Ok(0)
                }
            }
            2 => {
                if arg == 0 {
                    Ok(0)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "only FD_CLOEXEC-compatible flag value 0 is supported",
                    ))
                }
            }
            3 => Ok(0),
            4 => {
                if arg == 0 {
                    Ok(0)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "only zero status flag updates are supported",
                    ))
                }
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsupported fcntl action",
            )),
        }
    }

    fn duplicate_from_min(fd: RawFd, target: RawFd, cloexec: bool) -> io::Result<OwnedFd> {
        if target < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "target descriptor must be non-negative",
            ));
        }

        let source = Self::file_from_fd(fd)?;
        let mut held: Vec<File> = Vec::new();

        loop {
            let duplicated = source.try_clone()?;
            let duplicated_fd = duplicated.as_raw_fd();

            if duplicated_fd >= target {
                let _ = cloexec;
                return Ok(duplicated.into());
            }

            held.push(duplicated);
        }
    }

    fn file_from_fd(fd: RawFd) -> io::Result<File> {
        let path = format!("/proc/self/fd/{fd}");
        OpenOptions::new().read(true).open(path)
    }
}
