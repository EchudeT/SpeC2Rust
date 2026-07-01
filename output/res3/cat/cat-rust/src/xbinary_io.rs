use crate::binary_io::BinaryIo;

pub struct XbinaryIo;

impl XbinaryIo {
    pub fn set_mode_or_fail(fd: i32, mode: i32) {
        if BinaryIo::set_mode(fd, mode).is_err() {
            Self::fail_to_set_mode();
        }
    }

    fn fail_to_set_mode() -> ! {
        panic!("failed to set file descriptor text/binary mode")
    }
}
