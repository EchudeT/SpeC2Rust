use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, IntoRawFd, OwnedFd, RawFd};

use crate::cloexec::Cloexec;

pub struct Fcntl;


impl Fcntl {
    pub fn duplicate_from(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
        Self::validate_target(target)?;
        let borrowed = Self::borrow_valid_fd(fd)?;
        let duplicated = Self::clone_to_min(borrowed, target)?;
        Ok(duplicated.into_raw_fd())
    }

    pub fn dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
        Self::validate_target(target)?;
        let borrowed = Self::borrow_valid_fd(fd)?;

        match Self::clone_to_min_cloexec(borrowed, target) {
            Ok(duplicated) => Ok(duplicated.into_raw_fd()),
            Err(err) if err.raw_os_error() == Some(22) => {
                let duplicated = Self::clone_to_min(borrowed, target)?;
                Cloexec::flag(&duplicated, true)?;
                Ok(duplicated.into_raw_fd())
            }
            Err(err) => Err(err),
        }

        let duplicated = Self::clone_to_min(borrowed, target)?;
        Cloexec::flag(&duplicated, true)?;
        Ok(duplicated.into_raw_fd())
    }

    pub fn klibc_fcntl(fd: RawFd, action: i32, arg: i32) -> io::Result<i32> {
        const F_DUPFD: i32 = 0;
        const F_GETFD: i32 = 1;
        const F_SETFD: i32 = 2;
        const F_GETFL: i32 = 3;
        const F_SETFL: i32 = 4;
        const FD_CLOEXEC: i32 = 1;

        let borrowed = Self::borrow_valid_fd(fd)?;

        match action {
            F_DUPFD => Self::duplicate_from(fd, arg),
            F_GETFD => {
                let clone = borrowed.try_clone_to_owned()?;
                let path = format!("/proc/self/fd/{}", clone.as_raw_fd());
                let reopened = std::fs::File::options().read(true).open(path)?;
                let reopened_fd: OwnedFd = reopened.into();
                let cloexec_like = reopened_fd.as_raw_fd() != clone.as_raw_fd();
                Ok(if cloexec_like { FD_CLOEXEC } else { 0 })
            }
            F_SETFD => {
                if arg & !FD_CLOEXEC != 0 {
                    return Err(io::Error::from_raw_os_error(22));
                }
                if arg & FD_CLOEXEC != 0 {
                    Cloexec::flag(&borrowed, true)?;
                    Ok(0)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "clearing close-on-exec is not supported by this Rust implementation",
                    ))
                }
            }
            F_GETFL => Ok(0),
            F_SETFL => {
                if arg == 0 {
                    Ok(0)
                } else {
                    Err(io::Error::from_raw_os_error(22))
                }
            }
            _ => Err(io::Error::from_raw_os_error(22)),
        }
    }

    fn validate_target(target: RawFd) -> io::Result<()> {
        if target < 0 {
            Err(io::Error::from_raw_os_error(22))
        } else {
            Ok(())
        }
    }

    fn borrow_valid_fd(fd: RawFd) -> io::Result<BorrowedFd<'static>> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let path = format!("/proc/self/fd/{fd}");
        let file = std::fs::File::options().read(true).open(path)?;
        let owned: OwnedFd = file.into();
        Ok(owned.as_fd().try_clone_to_owned()?.as_fd().to_owned().as_fd())
    }

    fn clone_to_min(fd: BorrowedFd<'_>, target: RawFd) -> io::Result<OwnedFd> {
        let first = fd.try_clone_to_owned()?;
        if first.as_raw_fd() >= target {
            return Ok(first);
        }

        let mut too_small = vec![first];
        loop {
            let next = fd.try_clone_to_owned()?;
            if next.as_raw_fd() >= target {
                return Ok(next);
            }
            too_small.push(next);
        }
    }

    fn clone_to_min_cloexec(fd: BorrowedFd<'_>, target: RawFd) -> io::Result<OwnedFd> {
        let duplicated = Self::clone_to_min(fd, target)?;
        Cloexec::flag(&duplicated, true)?;
        Ok(duplicated)
    }
}
