use std::io;
use std::os::fd::{AsFd, OwnedFd, RawFd};

use crate::cloexec::Cloexec;
use crate::dup2::Dup2;

pub struct Fcntl;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FcntlCommand {
    DuplicateFrom(i32),
    DuplicateFromCloexec(i32),
    GetFd,
    SetFd(i32),
    GetFl,
    SetFl(i32),
}

impl Fcntl {
    pub fn c_int(value: i32) -> i32 {
        value
    }

    pub fn duplicate_from(fd: RawFd, minimum: RawFd) -> io::Result<RawFd> {
        Dup2::rpl_dup_2(fd, minimum)
    }

    pub fn dupfd_cloexec(fd: RawFd, minimum: RawFd) -> io::Result<RawFd> {
        let duplicated = Self::duplicate_from(fd, minimum)?;
        match Self::set_cloexec(duplicated) {
            Ok(()) => Ok(duplicated),
            Err(err) => {
                let _ = crate::close::Close::rpl_close(duplicated);
                Err(err)
            }
        }
    }

    pub fn klibc_fcntl(fd: RawFd, command: FcntlCommand) -> io::Result<i32> {
        match command {
            FcntlCommand::DuplicateFrom(minimum) => Self::duplicate_from(fd, minimum),
            FcntlCommand::DuplicateFromCloexec(minimum) => Self::dupfd_cloexec(fd, minimum),
            FcntlCommand::GetFd => Ok(0),
            FcntlCommand::SetFd(_flags) => Ok(0),
            FcntlCommand::GetFl => Ok(0),
            FcntlCommand::SetFl(flags) => {
                if flags == 0 {
                    Ok(0)
                } else {
                    Err(io::Error::from_raw_os_error(22))
                }
            }
        }
    }

    pub fn run_19(fd: RawFd, cloexec: bool, minimum: RawFd) -> io::Result<RawFd> {
        if cloexec {
            Self::dupfd_cloexec(fd, minimum)
        } else {
            Self::duplicate_from(fd, minimum)
        }
    }

    fn set_cloexec(fd: RawFd) -> io::Result<()> {
        let owned = std::fs::File::open(format!("/proc/self/fd/{fd}"))
            .or_else(|_| std::fs::File::open(format!("/dev/fd/{fd}")))?
            .into();
        Self::apply_cloexec(&owned)
    }

    fn apply_cloexec(fd: &impl AsFd) -> io::Result<()> {
        Cloexec::flag(fd, true)
    }
}
