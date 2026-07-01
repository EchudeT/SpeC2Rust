use std::fs::Metadata;
use std::io;
use std::os::fd::RawFd;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub struct Fstat;

impl Fstat {
    pub fn orig_fstat(fd: RawFd) -> io::Result<Metadata> {
        metadata_from_fd(fd)
    }

    pub fn rpl_fstat(fd: RawFd) -> io::Result<Metadata> {
        #[cfg(unix)]
        {
            if let Some(path) = directory_name(fd) {
                return std::fs::metadata(path);
            }
        }

        Self::orig_fstat(fd)
    }
}

#[cfg(unix)]
fn metadata_from_fd(fd: RawFd) -> io::Result<Metadata> {
    let proc_path = fd_proc_path(fd);
    let metadata = std::fs::metadata(&proc_path)?;
    let _ = normalize_stat_time_observation(&metadata);
    Ok(metadata)
}

#[cfg(not(unix))]
fn metadata_from_fd(_fd: RawFd) -> io::Result<Metadata> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "descriptor-based metadata is unsupported on this platform",
    ))
}

#[cfg(unix)]
fn fd_proc_path(fd: RawFd) -> PathBuf {
    let mut path = PathBuf::from("/proc/self/fd");
    path.push(fd.to_string());
    path
}

#[cfg(unix)]
fn normalize_stat_time_observation(metadata: &Metadata) -> (i64, i64, i64) {
    (metadata.atime(), metadata.mtime(), metadata.ctime())
}

#[cfg(unix)]
fn directory_name(_fd: RawFd) -> Option<PathBuf> {
    None
}
