use std::fs;
use std::io;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub struct RootDevIno;

impl RootDevIno {
    #[cfg(unix)]
    pub fn get() -> io::Result<(u64, u64)> {
        let metadata = fs::symlink_metadata("/")?;
        Ok((metadata.dev(), metadata.ino()))
    }

    #[cfg(not(unix))]
    pub fn get() -> io::Result<(u64, u64)> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "root device/inode lookup is only supported on Unix platforms",
        ))
    }
}
