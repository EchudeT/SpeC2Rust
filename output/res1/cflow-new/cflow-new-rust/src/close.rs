use std::fs::File;
use std::io;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

pub struct Close;

impl Close {
    pub fn nothrow(fd: RawFd) -> io::Result<()> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let file = File::from_raw_fd(fd);
        match file.sync_all() {
            Ok(()) => {
                drop(file);
                Ok(())
            }
            Err(_) => {
                drop(file);
                Ok(())
            }
        }
    }

    pub fn rpl_close(fd: RawFd) -> io::Result<()> {
        Self::nothrow(fd)
    }
}
