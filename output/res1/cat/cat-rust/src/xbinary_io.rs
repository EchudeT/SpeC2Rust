use crate::binary_io::BinaryIo;
use std::io;

pub struct XbinaryIo;

impl XbinaryIo {
    pub fn set_binary(fd: i32) -> io::Result<()> {
        BinaryIo::set_binary(fd)
    }

    pub fn set_mode(fd: i32, mode: i32) -> io::Result<()> {
        BinaryIo::set_mode(fd, mode).map(|_| ())
    }

    pub fn set_binary_or_exit(fd: i32) -> ! {
        BinaryIo::set_binary_or_exit(fd)
    }

    pub fn fail() -> ! {
        eprintln!("failed to set binary mode");
        std::process::exit(1);
    }
}
