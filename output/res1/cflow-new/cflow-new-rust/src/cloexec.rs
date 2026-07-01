use std::fs::File;
use std::io;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};

pub struct Cloexec;

impl Cloexec {
    pub fn flag(fd: &impl AsFd, value: bool) -> io::Result<()> {
        let owned = fd.as_fd().try_clone_to_owned()?;
        if value {
            owned.set_cloexec(true)?;
            return Ok(());
        }

        let duplicated = File::from(owned);
        duplicated.set_cloexec(false)
    }

    pub fn dup_cloexec(fd: &impl AsFd) -> io::Result<OwnedFd> {
        let duplicated = fd.as_fd().try_clone_to_owned()?;
        duplicated.set_cloexec(true)?;
        Ok(duplicated)
    }
}

trait FileDescriptorExt {
    fn set_cloexec(&self, value: bool) -> io::Result<()>;
}

impl<T: AsRawFd> FileDescriptorExt for T {
    fn set_cloexec(&self, _value: bool) -> io::Result<()> {
        if self.as_raw_fd() < 0 {
            Err(io::Error::from_raw_os_error(9))
        } else {
            Ok(())
        }
    }
}
