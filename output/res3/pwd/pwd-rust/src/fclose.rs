use std::fs::File;
use std::io;
#[cfg(unix)]
use std::os::fd::AsRawFd;

/// Rust-style close helpers corresponding to the behavior evidenced by `fclose.c`.
///
/// This module models two layers:
/// - `nothrow`: perform the final close attempt and report only that close result.
/// - `rpl_fclose`: preserve any earlier stream error in preference to a later close error.
///
/// Because Rust's `std::fs::File` does not expose C `FILE*` state such as `ferror`,
/// the replacement API accepts an optional previously observed I/O error from the
/// caller and preserves it across the final close attempt.
pub struct Fclose;

impl Fclose {
    /// Perform the underlying close attempt.
    ///
    /// In Rust, consuming the `File` and allowing it to be dropped is the closest
    /// safe analogue to `fclose`. The standard library does not provide an explicit
    /// close operation that reports errors, so this function synchronizes metadata
    /// first to surface close-path writeback failures where possible, then consumes
    /// the file handle.
    pub fn nothrow(file: File) -> io::Result<()> {
        #[cfg(unix)]
        {
            let _ = file.as_raw_fd();
        }

        let sync_result = file.sync_all();
        drop(file);
        sync_result
    }

    /// Close a file while preserving any earlier stream error as the effective result.
    ///
    /// If `prior_error` is present, the file is still closed, but that earlier error
    /// is returned regardless of whether the final close attempt also fails.
    pub fn rpl_fclose(file: File, prior_error: Option<io::Error>) -> io::Result<()> {
        let close_result = Self::nothrow(file);

        match prior_error {
            Some(err) => Err(err),
            None => close_result,
        }
    }
}
