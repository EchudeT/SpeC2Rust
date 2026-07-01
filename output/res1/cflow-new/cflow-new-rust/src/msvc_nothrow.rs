use std::fs::File;

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// Utilities for obtaining a platform OS handle from a Rust file descriptor-like value
/// without using exception-style failure.
///
/// The original C module centered on a nothrow wrapper around MSVC's
/// `_get_osfhandle(int) -> intptr_t`. In this Rust rewrite, the API is expressed in
/// Rust terms and returns the result through ordinary values.
pub struct MsvcNothrow;

impl MsvcNothrow {
    /// Returns the platform OS handle associated with a borrowed file.
    ///
    /// On Windows, this is the native raw handle value cast to `isize`.
    /// On Unix and other non-Windows targets, this returns the raw file descriptor
    /// value cast to `isize`, providing a portable integer-sized representation for
    /// callers that only need an integer handle identity.
    pub fn os_handle(file: &File) -> isize {
        #[cfg(windows)]
        {
            file.as_raw_handle() as isize
        }

        #[cfg(unix)]
        {
            file.as_raw_fd() as isize
        }

        #[cfg(not(any(windows, unix)))]
        {
            let _ = file;
            -1
        }
    }

    /// Returns the platform OS handle associated with an optional borrowed file.
    ///
    /// Missing files map to `None`, preserving Rust-style absence semantics.
    pub fn os_handle_option(file: Option<&File>) -> Option<isize> {
        file.map(Self::os_handle)
    }
}
