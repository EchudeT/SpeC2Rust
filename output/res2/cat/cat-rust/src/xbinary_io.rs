use crate::binary_io::BinaryIo;

/// Fallible, higher-level binary I/O helper that mirrors the role of the
/// original `xbinary-io` module while exposing a Rust-style API.
pub struct XbinaryIo;

impl XbinaryIo {
    /// Ensure that the file descriptor is placed in the requested mode.
    ///
    /// On platforms where text/binary mode distinctions do not matter, this is
    /// effectively a no-op delegated to `BinaryIo`.
    ///
    /// If the mode cannot be applied, this function terminates the process in
    /// the same spirit as the original `x*` helper family.
    pub fn require_mode(fd: i32, mode: i32) {
        BinaryIo::require_mode(fd, mode);
    }
}
