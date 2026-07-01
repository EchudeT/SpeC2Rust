use std::cmp::min;
use std::io;
use std::os::unix::io::RawFd;

pub struct SafeWrite;

impl SafeWrite {
    const SYS_BUFSIZE_MAX: usize = isize::MAX as usize;

    pub fn write(fd: RawFd, buffer: &[u8], count: usize) -> io::Result<usize> {
        let mut requested = min(count, buffer.len());

        loop {
            match nix::unistd::write(fd, &buffer[..requested]) {
                Ok(result) => return Ok(result),
                Err(errno) if errno == nix::errno::Errno::EINTR => continue,
                Err(errno)
                    if errno == nix::errno::Errno::EINVAL
                        && requested > Self::SYS_BUFSIZE_MAX =>
                {
                    requested = Self::SYS_BUFSIZE_MAX;
                }
                Err(errno) => return Err(io::Error::from_raw_os_error(errno as i32)),
            }
        }
    }
}
