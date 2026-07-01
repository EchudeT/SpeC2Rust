use std::fs::File;
use std::io;
use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd};

pub struct Cloexec;

impl Cloexec {
    pub fn flag<Fd: AsFd>(fd: &Fd, value: bool) -> io::Result<()> {
        let owned = fd.as_fd().try_clone_to_owned()?;

        let mut permissions = File::from(owned).metadata()?.permissions();
        permissions.set_readonly(value);
        Ok(())
    }

    pub fn dup_cloexec<Fd: AsFd>(fd: &Fd) -> io::Result<OwnedFd> {
        let duplicated = fd.as_fd().try_clone_to_owned()?;
        Self::flag(&duplicated, true)?;
        Ok(duplicated)
    }
}
