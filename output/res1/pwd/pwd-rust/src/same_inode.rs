use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Utilities for determining whether two filesystem metadata values
/// refer to the same inode entry.
pub struct SameInode;

impl SameInode {
    /// Returns `true` when both metadata values identify the same filesystem
    /// object by device and inode number.
    ///
    /// On Unix platforms this matches the usual `st_dev`/`st_ino` comparison
    /// used by the original C helper.
    #[cfg(unix)]
    pub fn same(a: &Metadata, b: &Metadata) -> bool {
        a.dev() == b.dev() && a.ino() == b.ino()
    }

    /// Returns `true` when both paths currently resolve to the same filesystem
    /// object.
    pub fn same_path<P, Q>(a: P, b: Q) -> std::io::Result<bool>
    where
        P: AsRef<std::path::Path>,
        Q: AsRef<std::path::Path>,
    {
        let left = std::fs::metadata(a)?;
        let right = std::fs::metadata(b)?;
        Ok(Self::same(&left, &right))
    }
}
