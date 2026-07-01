use std::io;
use std::os::unix::fs::FileExt;
use std::os::unix::io::RawFd;
use std::{fs::File, path::PathBuf};

pub struct CopyFileRange;

impl CopyFileRange {
    pub fn copy(
        infd: RawFd,
        mut input_offset: Option<u64>,
        outfd: RawFd,
        mut output_offset: Option<u64>,
        length: usize,
        flags: u32,
    ) -> io::Result<usize> {
        if flags != 0 {
            return Err(io::Error::from(io::ErrorKind::Unsupported));
        }

        if !Self::kernel_supports_copy_file_range() {
            return Err(io::Error::from(io::ErrorKind::Unsupported));
        }

        let input = Self::open_fd_path(infd)?;
        let output = Self::open_fd_path(outfd)?;

        let copied = match (input_offset.as_mut(), output_offset.as_mut()) {
            (Some(in_off), Some(out_off)) => {
                Self::copy_between_offsets(&input, *in_off, &output, *out_off, length)?
            }
            (Some(in_off), None) => {
                Self::copy_from_offset_to_stream(&input, *in_off, &output, length)?
            }
            (None, Some(out_off)) => {
                Self::copy_from_stream_to_offset(&input, &output, *out_off, length)?
            }
            (None, None) => Self::copy_stream_to_stream(&input, &output, length)?,
        };

        if let Some(in_off) = input_offset.as_mut() {
            *in_off = in_off.saturating_add(copied as u64);
        }
        if let Some(out_off) = output_offset.as_mut() {
            *out_off = out_off.saturating_add(copied as u64);
        }

        Ok(copied)
    }

    fn kernel_supports_copy_file_range() -> bool {
        #[cfg(target_os = "linux")]
        {
            let release = std::fs::read_to_string("/proc/sys/kernel/osrelease")
                .ok()
                .or_else(|| {
                    std::process::Command::new("uname")
                        .arg("-r")
                        .output()
                        .ok()
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                });

            if let Some(release) = release {
                return Self::linux_release_is_after_5_2(&release);
            }

            false
        }

        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    #[cfg(target_os = "linux")]
    fn linux_release_is_after_5_2(release: &str) -> bool {
        let mut parts = release
            .split(|c: char| !(c.is_ascii_digit()))
            .filter(|s| !s.is_empty());

        let major = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let minor = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

        major > 5 || (major == 5 && minor > 2)
    }

    fn open_fd_path(fd: RawFd) -> io::Result<File> {
        let path = Self::fd_path(fd);
        File::options().read(true).write(true).open(path)
    }

    fn fd_path(fd: RawFd) -> PathBuf {
        let mut path = PathBuf::from("/proc/self/fd");
        path.push(fd.to_string());
        path
    }

    fn copy_between_offsets(
        input: &File,
        input_offset: u64,
        output: &File,
        output_offset: u64,
        length: usize,
    ) -> io::Result<usize> {
        let mut buffer = vec![0_u8; length.min(1024 * 1024)];
        let read = input.read_at(&mut buffer, input_offset)?;
        if read == 0 {
            return Ok(0);
        }
        output.write_at(&buffer[..read], output_offset)
    }

    fn copy_from_offset_to_stream(
        input: &File,
        input_offset: u64,
        output: &File,
        length: usize,
    ) -> io::Result<usize> {
        let mut buffer = vec![0_u8; length.min(1024 * 1024)];
        let read = input.read_at(&mut buffer, input_offset)?;
        if read == 0 {
            return Ok(0);
        }
        Self::write_sequential(output, &buffer[..read])
    }

    fn copy_from_stream_to_offset(
        input: &File,
        output: &File,
        output_offset: u64,
        length: usize,
    ) -> io::Result<usize> {
        let mut buffer = vec![0_u8; length.min(1024 * 1024)];
        let read = Self::read_sequential(input, &mut buffer[..length.min(buffer.len())])?;
        if read == 0 {
            return Ok(0);
        }
        output.write_at(&buffer[..read], output_offset)
    }

    fn copy_stream_to_stream(input: &File, output: &File, length: usize) -> io::Result<usize> {
        let mut buffer = vec![0_u8; length.min(1024 * 1024)];
        let read = Self::read_sequential(input, &mut buffer[..length.min(buffer.len())])?;
        if read == 0 {
            return Ok(0);
        }
        Self::write_sequential(output, &buffer[..read])
    }

    fn read_sequential(file: &File, buffer: &mut [u8]) -> io::Result<usize> {
        let mut handle = file;
        io::Read::read(&mut handle, buffer)
    }

    fn write_sequential(file: &File, buffer: &[u8]) -> io::Result<usize> {
        let mut handle = file;
        io::Write::write(&mut handle, buffer)
    }
}
