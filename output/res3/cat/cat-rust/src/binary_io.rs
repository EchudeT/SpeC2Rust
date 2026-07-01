use std::io;

pub struct BinaryIo;

impl BinaryIo {
    pub const TEXT_MODE: i32 = 0;
    pub const BINARY_MODE: i32 = 1;

    pub fn set_mode(fd: i32, mode: i32) -> io::Result<i32> {
        #[cfg(windows)]
        {
            Self::set_mode_windows(fd, mode)
        }

        #[cfg(not(windows))]
        {
            let _ = fd;
            Ok(Self::normalize_mode(mode))
        }
    }

    pub fn ensure_mode(fd: i32, mode: i32) {
        if Self::set_mode(fd, mode).is_err() {
            Self::fail_to_set_mode();
        }
    }

    fn normalize_mode(mode: i32) -> i32 {
        if mode == Self::BINARY_MODE {
            Self::BINARY_MODE
        } else {
            Self::TEXT_MODE
        }
    }

    #[cfg(windows)]
    fn set_mode_windows(fd: i32, mode: i32) -> io::Result<i32> {
        if Self::is_console_descriptor(fd) {
            return Ok(Self::TEXT_MODE);
        }

        Ok(Self::normalize_mode(mode))
    }

    #[cfg(windows)]
    fn is_console_descriptor(fd: i32) -> bool {
        matches!(fd, 0..=2)
    }

    #[cfg(not(windows))]
    fn fail_to_set_mode() -> ! {
        panic!("failed to set file descriptor text/binary mode")
    }

    #[cfg(windows)]
    fn fail_to_set_mode() -> ! {
        let error = io::Error::last_os_error();
        panic!("failed to set file descriptor text/binary mode: {error}")
    }
}
