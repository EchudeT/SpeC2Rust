use std::fs;

use crate::pwd::DevIno;

pub struct RootDevIno;

impl RootDevIno {
    pub fn get() -> Option<DevIno> {
        let metadata = fs::symlink_metadata("/").ok()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Some(DevIno {
                dev: metadata.dev(),
                ino: metadata.ino(),
            })
        }
        #[cfg(not(unix))]
        {
            let _ = metadata;
            None
        }
    }
}
