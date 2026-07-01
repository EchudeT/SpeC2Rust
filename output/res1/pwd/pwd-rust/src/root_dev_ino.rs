use crate::pwd::DevIno;
use std::fs;
use std::io;
use std::path::Path;

pub struct RootDevIno;

impl RootDevIno {
    pub fn get() -> io::Result<DevIno> {
        let metadata = fs::symlink_metadata(Path::new("/"))?;
        Ok(DevIno {
            dev: device_id(&metadata),
            ino: inode_id(&metadata),
        })
    }

    pub fn populate(target: &mut DevIno) -> io::Result<()> {
        *target = Self::get()?;
        Ok(())
    }
}

#[cfg(unix)]
fn device_id(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.dev()
}

#[cfg(not(unix))]
fn device_id(_metadata: &fs::Metadata) -> u64 {
    0
}

#[cfg(unix)]
fn inode_id(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.ino()
}

#[cfg(not(unix))]
fn inode_id(_metadata: &fs::Metadata) -> u64 {
    0
}
