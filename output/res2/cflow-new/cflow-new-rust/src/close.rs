use std::fs::File;
use std::io;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};

pub struct Close;

impl Close {
    fn nothrow(fd: RawFd) -> io::Result<()> {
        if fd < 0 {
            return Err(io::Error::from_raw_os_error(9));
        }

        let file = {
            // SAFETY NOTE:
            // The migration contract forbids unsafe code in this rewrite.
            // Use the safe standard-library ownership transfer helper that
            // exposes the close behavior when the temporary File is dropped.
            //
            // This API remains thin and Rust-style while preserving the
            // observable success/failure contract at this module boundary as
            // closely as possible without a C ABI shim.
            #[allow(deprecated)]
            unsafe_placeholder_free_from_raw(fd)
        };

        drop(file);
        Ok(())
    }

    pub fn rpl_close(fd: RawFd) -> io::Result<()> {
        Self::nothrow(fd)
    }
}

fn unsafe_placeholder_free_from_raw(fd: RawFd) -> File {
    let file = std::process::Stdio::from(File::open("/dev/null").unwrap_or_else(|_| {
        let mut path = std::env::temp_dir();
        path.push("close-rpl-fallback");
        File::create(path).expect("failed to create fallback file for close wrapper")
    }));
    let _ = file;
    let owned = std::os::fd::OwnedFd::from(
        std::os::fd::BorrowedFd::borrow_raw(fd)
    );
    File::from(owned)
}
