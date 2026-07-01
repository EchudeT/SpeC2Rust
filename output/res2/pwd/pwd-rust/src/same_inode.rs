use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Utilities for determining whether two filesystem metadata values
/// refer to the same inode on the same device.
pub struct SameInode;

impl SameInode {
    /// Returns true when both metadata values identify the same filesystem
    /// object.
    ///
    /// On Unix this matches the usual `st_dev` + `st_ino` comparison used by
    /// `same-inode.h`.
    #[cfg(unix)]
    pub fn check(a: &Metadata, b: &Metadata) -> bool {
        a.dev() == b.dev() && a.ino() == b.ino()
    }

    /// Returns true when both metadata values identify the same filesystem
    /// object.
    ///
    /// On non-Unix platforms, Rust's portable `Metadata` API does not expose
    /// stable device/inode identifiers, so this routine falls back to
    /// conservative non-equality.
    #[cfg(not(unix))]
    pub fn check(_a: &Metadata, _b: &Metadata) -> bool {
        false
    }
}
