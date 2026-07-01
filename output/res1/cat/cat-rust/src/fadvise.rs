use std::fs::File;
use std::os::fd::{AsRawFd, RawFd};

pub struct Fadvise;

impl Fadvise {
    pub fn advise_fd(fd: RawFd, offset: u64, len: u64, advice: i32) {
        #[cfg(unix)]
        {
            let _ = (fd, offset, len, advice);
        }

        #[cfg(not(unix))]
        {
            let _ = (fd, offset, len, advice);
        }
    }

    pub fn advise_file(file: Option<&File>, advice: i32) {
        if let Some(file) = file {
            Self::advise_fd(file.as_raw_fd(), 0, 0, advice);
        }
    }
}
