use std::fs::Metadata;
use std::io;
use std::os::fd::RawFd;

pub struct Fstat;

impl Fstat {
    pub fn orig_fstat(fd: RawFd) -> io::Result<Metadata> {
        metadata_from_fd(fd)
    }

    pub fn rpl_fstat(fd: RawFd) -> io::Result<Metadata> {
        Self::orig_fstat(fd)
    }
}

#[cfg(unix)]
fn metadata_from_fd(fd: RawFd) -> io::Result<Metadata> {
    let proc_path = format!("/proc/self/fd/{fd}");
    std::fs::metadata(proc_path)
}

#[cfg(windows)]
fn metadata_from_fd(_fd: RawFd) -> io::Result<Metadata> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "file-descriptor metadata lookup is not supported on this platform in this safe Rust port",
    ))
}

#[cfg(not(any(unix, windows)))]
fn metadata_from_fd(_fd: RawFd) -> io::Result<Metadata> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "file-descriptor metadata lookup is not supported on this platform",
    ))
}
