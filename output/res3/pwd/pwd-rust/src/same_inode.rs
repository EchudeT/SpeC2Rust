use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Utilities for determining whether two filesystem metadata values
/// refer to the same inode on the same device.
pub struct SameInode;

impl SameInode {
    /// Returns true when both metadata values identify the same filesystem object.
    ///
    /// On Unix, this matches the traditional `same-inode` behavior by comparing
    /// both device and inode numbers.
    #[cfg(unix)]
    pub fn from_metadata(a: &Metadata, b: &Metadata) -> bool {
        a.dev() == b.dev() && a.ino() == b.ino()
    }

    /// Returns true when both metadata values identify the same filesystem object.
    ///
    /// On non-Unix targets, a portable device/inode comparison is unavailable in
    /// the standard library, so this always returns false.
    #[cfg(not(unix))]
    pub fn from_metadata(_a: &Metadata, _b: &Metadata) -> bool {
        false
    }

    /// Returns true when the provided `(device, inode)` pairs are equal.
    pub fn from_dev_ino(a: (u64, u64), b: (u64, u64)) -> bool {
        a == b
    }
}
