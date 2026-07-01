use std::fs::File;
use std::io;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

pub struct Close;

impl Close {
    fn close_via_file(fd: RawFd) -> io::Result<()> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(22));
        }

        let file = {
            // SAFETY: This function is the ownership boundary for the supplied raw
            // file descriptor. Constructing a File from it lets Rust perform the
            // close on drop, matching the intended close semantics for this module.
            unsafe { File::from_raw_fd(fd) }
        };

        drop(file);
        Ok(())
    }

    pub fn nothrow(fd: RawFd) -> io::Result<()> {
        Self::close_via_file(fd)
    }

    pub fn rpl_close(fd: RawFd) -> io::Result<()> {
        Self::nothrow(fd)
    }
}
