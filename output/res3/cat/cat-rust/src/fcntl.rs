use std::io;

#[cfg(unix)]
pub const F_DUPFD: i32 = 0;
#[cfg(unix)]
pub const F_GETFD: i32 = 1;
#[cfg(unix)]
pub const F_SETFD: i32 = 2;
#[cfg(unix)]
pub const F_GETFL: i32 = 3;
#[cfg(unix)]
pub const F_SETFL: i32 = 4;
#[cfg(unix)]
pub const FD_CLOEXEC: i32 = 1;
#[cfg(unix)]
pub const O_APPEND: i32 = 0o2000;
#[cfg(unix)]
pub const SEEK_SET: i32 = 0;
#[cfg(unix)]
pub const SEEK_CUR: i32 = 1;
#[cfg(unix)]
pub const SEEK_END: i32 = 2;

pub struct Fcntl;

#[cfg(unix)]
unsafe extern "C" {
    #[link_name = "fcntl"]
    fn libc_fcntl(fd: i32, cmd: i32, ...) -> i32;
    #[link_name = "lseek"]
    fn libc_lseek(fd: i32, offset: i64, whence: i32) -> i64;
}

#[cfg(unix)]
fn last_os_error() -> io::Error {
    io::Error::last_os_error()
}

#[cfg(unix)]
pub fn fcntl(fd: i32, cmd: i32, arg: i32) -> io::Result<i32> {
    let ret = unsafe { libc_fcntl(fd, cmd, arg) };
    if ret < 0 {
        Err(last_os_error())
    } else {
        Ok(ret)
    }
}

#[cfg(not(unix))]
pub fn fcntl(_fd: i32, _cmd: i32, _arg: i32) -> io::Result<i32> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "fcntl unsupported on this platform",
    ))
}

#[cfg(unix)]
pub fn lseek(fd: i32, offset: i64, whence: i32) -> io::Result<i64> {
    let ret = unsafe { libc_lseek(fd, offset, whence) };
    if ret < 0 {
        Err(last_os_error())
    } else {
        Ok(ret)
    }
}

#[cfg(not(unix))]
pub fn lseek(_fd: i32, _offset: i64, _whence: i32) -> io::Result<i64> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "lseek unsupported on this platform",
    ))
}

#[cfg(not(unix))]
pub fn lseek(_fd: i32, _offset: i64, _whence: i32) -> io::Result<i64> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "lseek unsupported on this platform",
    ))
}

impl Fcntl {
    pub fn dupfd(fd: i32, target_min: i32) -> io::Result<i32> {
        fcntl(fd, F_DUPFD, target_min)
    }

    pub fn getfd(fd: i32) -> io::Result<i32> {
        fcntl(fd, F_GETFD, 0)
    }

    pub fn setfd(fd: i32, flags: i32) -> io::Result<()> {
        fcntl(fd, F_SETFD, flags).map(|_| ())
    }

    pub fn getfl(fd: i32) -> io::Result<i32> {
        fcntl(fd, F_GETFL, 0)
    }

    pub fn setfl(fd: i32, flags: i32) -> io::Result<()> {
        fcntl(fd, F_SETFL, flags).map(|_| ())
    }

    pub fn set_cloexec(fd: i32) -> io::Result<()> {
        let flags = Self::getfd(fd)?;
        Self::setfd(fd, flags | FD_CLOEXEC)
    }
}
