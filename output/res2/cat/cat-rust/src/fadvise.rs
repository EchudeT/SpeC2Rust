use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;

pub struct Fadvise;

impl Fadvise {
    pub fn advise_fd(fd: i32, offset: i64, len: i64, advice: i32) {
        let _ = (fd, offset, len, advice);
    }

    pub fn advise(file: Option<&File>, advice: i32) -> io::Result<()> {
        if let Some(file) = file {
            Self::advise_fd(file.as_raw_fd(), 0, 0, advice);
        }
        Ok(())
    }
}
