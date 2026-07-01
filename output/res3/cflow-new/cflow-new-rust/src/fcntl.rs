use std::io;
use std::os::fd::{AsFd, OwnedFd, RawFd};

use crate::cloexec::Cloexec;

pub struct Fcntl;

impl Fcntl {
    pub fn duplicate_from(fd: RawFd, minimum: RawFd) -> io::Result<RawFd> {
        Self::duplicate_internal(fd, minimum, false)
    }

    pub fn dupfd_cloexec(fd: RawFd, minimum: RawFd) -> io::Result<RawFd> {
        Self::duplicate_internal(fd, minimum, true)
    }

    pub fn klibc_fcntl(fd: RawFd, action: i32, arg: i32) -> io::Result<i32> {
        const F_DUPFD_ACTION: i32 = 0;
        const F_DUPFD_CLOEXEC_ACTION: i32 = 1030;

        match action {
            F_DUPFD_ACTION => Self::duplicate_from(fd, arg),
            F_DUPFD_CLOEXEC_ACTION => Self::dupfd_cloexec(fd, arg),
            _ => Err(io::Error::from_raw_os_error(22)),
        }
    }

    fn duplicate_internal(fd: RawFd, minimum: RawFd, cloexec: bool) -> io::Result<RawFd> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }
        if minimum < 0 {
            return Err(io::Error::from_raw_os_error(22));
        }

        let owned = Self::owned_fd(fd)?;
        let duplicate = Self::duplicate_owned(&owned, minimum, cloexec)?;
        Ok(std::os::fd::IntoRawFd::into_raw_fd(duplicate))
    }

    fn owned_fd(fd: RawFd) -> io::Result<OwnedFd> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        std::fs::File::open(format!("/proc/self/fd/{fd}"))
            .map(OwnedFd::from)
            .map_err(|_| io::Error::from_raw_os_error(9))
    }

    fn duplicate_owned(fd: &OwnedFd, minimum: RawFd, cloexec: bool) -> io::Result<OwnedFd> {
        let mut held = Vec::new();

        loop {
            let candidate = if cloexec {
                Cloexec::dup_cloexec(fd.as_fd())?
            } else {
                fd.try_clone()?
            };

            if candidate.as_fd().as_raw_fd() >= minimum {
                return Ok(candidate);
            }

            held.push(candidate);
        }
    }
}
