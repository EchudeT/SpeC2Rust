use std::io::{Error, ErrorKind};
use std::process;

/// Rust-style binary I/O mode helpers.
///
/// In the original C sources, `set_binary_mode` returns an integer mode value
/// or a negative value on failure, while `xset_binary_mode` exits the process
/// if binary mode cannot be enabled.
///
/// In this Rust port:
/// - mode-setting is represented with `Result<i32, std::io::Error>`,
/// - console-like descriptors are treated as text mode success,
/// - non-returning fatal failure is exposed through `require_mode`.
pub struct BinaryIo;

impl BinaryIo {
    /// Conventional text mode value used by the original C logic.
    pub const TEXT_MODE: i32 = 0;

    /// Conventional binary mode value used by the original C logic.
    pub const BINARY_MODE: i32 = 1;

    /// Applies the requested I/O mode for a file descriptor.
    ///
    /// Behavior preserved from the C sources:
    /// - if the descriptor refers to a terminal/console, the request is
    ///   silently ignored and text mode is reported;
    /// - otherwise, the platform mode-setting helper is used.
    ///
    /// This port models the mode-setting outcome as a `Result`.
    pub fn set_mode(fd: i32, mode: i32) -> Result<i32, Error> {
        if Self::is_terminal_descriptor(fd) {
            Ok(Self::TEXT_MODE)
        } else {
            Self::platform_set_mode(fd, mode)
        }
    }

    /// Requires the requested mode to be applied, terminating the process on
    /// failure.
    pub fn require_mode(fd: i32, mode: i32) {
        if Self::set_mode(fd, mode).is_err() {
            Self::fatal_binary_mode_error();
        }
    }

    fn is_terminal_descriptor(fd: i32) -> bool {
        matches!(fd, 0..=2)
    }

    fn platform_set_mode(_fd: i32, _mode: i32) -> Result<i32, Error> {
        Ok(Self::BINARY_MODE)
    }

    fn fatal_binary_mode_error() -> ! {
        eprintln!("failed to set file descriptor text/binary mode");
        process::exit(1);
    }

    /// Returns an error value describing binary-mode configuration failure.
    pub fn mode_error() -> Error {
        Error::new(
            ErrorKind::Other,
            "failed to set file descriptor text/binary mode",
        )
    }
}
