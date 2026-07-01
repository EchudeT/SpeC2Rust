use std::fs::Metadata;
use std::io;
use std::os::fd::RawFd;

pub struct Fstat;

impl Fstat {
    pub fn orig_fstat(fd: RawFd) -> io::Result<Metadata> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let proc_path = format!("/proc/self/fd/{fd}");
        std::fs::metadata(proc_path)
    }

    pub fn rpl_fstat(fd: RawFd) -> io::Result<Metadata> {
        Self::orig_fstat(fd)
    }
}
