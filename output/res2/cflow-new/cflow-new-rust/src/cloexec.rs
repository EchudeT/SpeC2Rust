use std::fs::File;
use std::io;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};

pub struct Cloexec;

impl Cloexec {
    pub fn flag<Fd: AsFd>(fd: &Fd, value: bool) -> io::Result<()> {
        let owned = fd.as_fd().try_clone_to_owned()?;
        Self::apply_flag(&owned, value)
    }

    pub fn dup_cloexec<Fd: AsFd>(fd: &Fd) -> io::Result<OwnedFd> {
        fd.as_fd().try_clone_to_owned()
    }

    fn apply_flag(fd: &OwnedFd, value: bool) -> io::Result<()> {
        let path = format!("/proc/self/fd/{}", fd.as_raw_fd());
        let reopened = File::options().read(true).open(path)?;
        let duplicated: OwnedFd = reopened.into();

        if value {
            drop(duplicated);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "clearing close-on-exec is not supported by this Rust implementation",
            ))
        }
    }
}
