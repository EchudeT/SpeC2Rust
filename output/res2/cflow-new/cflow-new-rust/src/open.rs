use std::fs::{File, OpenOptions};
use std::io;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub struct Open;

impl Open {
    pub fn orig_open(
        path: impl AsRef<Path>,
        flags: i32,
        mode: Option<u32>,
    ) -> io::Result<File> {
        let mut options = OpenOptions::new();
        Self::apply_access_mode(&mut options, flags)?;
        Self::apply_creation_mode(&mut options, flags);
        Self::apply_append_mode(&mut options, flags);
        Self::apply_truncate_mode(&mut options, flags);

        #[cfg(unix)]
        {
            let custom_flags = Self::supported_custom_flags(flags);
            options.custom_flags(custom_flags);
            if let Some(mode_bits) = mode {
                options.mode(mode_bits);
            }
        }

        options.open(path)
    }

    pub fn open_file(path: impl AsRef<Path>, flags: i32, mode: Option<u32>) -> io::Result<File> {
        let path = path.as_ref();

        let normalized_flags = Self::strip_ignored_flags(flags);

        if Self::requires_directory_trailing_slash_failure(path, normalized_flags) {
            return Err(io::Error::from_raw_os_error(21));
        }

        let mut first_attempt_flags = normalized_flags;
        #[cfg(unix)]
        {
            if Self::has_flag(normalized_flags, Self::o_cloexec()) {
                first_attempt_flags &= !Self::o_cloexec();
            }
        }

        let mut file = Self::orig_open(path, first_attempt_flags, mode)?;

        if path_has_trailing_slash(path) {
            let metadata = file.metadata()?;
            if !metadata.is_dir() {
                return Err(io::Error::from_raw_os_error(20));
            }
        }

        Ok(file)
    }

    fn apply_access_mode(options: &mut OpenOptions, flags: i32) -> io::Result<()> {
        match flags & Self::o_accmode() {
            x if x == Self::o_rdonly() => {
                options.read(true);
            }
            x if x == Self::o_wronly() => {
                options.write(true);
            }
            x if x == Self::o_rdwr() => {
                options.read(true).write(true);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "unsupported access mode",
                ));
            }
        }
        Ok(())
    }

    fn apply_creation_mode(options: &mut OpenOptions, flags: i32) {
        if Self::has_flag(flags, Self::o_creat()) {
            options.create(true);
        }
        if Self::has_flag(flags, Self::o_excl()) {
            options.create_new(true);
        }
    }

    fn apply_append_mode(options: &mut OpenOptions, flags: i32) {
        if Self::has_flag(flags, Self::o_append()) {
            options.append(true);
        }
    }

    fn apply_truncate_mode(options: &mut OpenOptions, flags: i32) {
        if Self::has_flag(flags, Self::o_trunc()) {
            options.truncate(true);
        }
    }

    fn strip_ignored_flags(flags: i32) -> i32 {
        #[cfg(unix)]
        {
            flags & !Self::o_nonblock()
        }
        #[cfg(not(unix))]
        {
            flags
        }
    }

    fn requires_directory_trailing_slash_failure(path: &Path, flags: i32) -> bool {
        path_has_trailing_slash(path)
            && (Self::has_flag(flags, Self::o_creat())
                || (flags & Self::o_accmode()) == Self::o_wronly()
                || (flags & Self::o_accmode()) == Self::o_rdwr())
    }

    #[cfg(unix)]
    fn supported_custom_flags(flags: i32) -> i32 {
        let ignored = Self::o_accmode()
            | Self::o_creat()
            | Self::o_excl()
            | Self::o_trunc()
            | Self::o_append();
        flags & !ignored
    }

    fn has_flag(flags: i32, flag: i32) -> bool {
        flag != 0 && (flags & flag) == flag
    }

    #[cfg(unix)]
    fn o_rdonly() -> i32 {
        0
    }

    #[cfg(not(unix))]
    fn o_rdonly() -> i32 {
        0
    }

    #[cfg(unix)]
    fn o_wronly() -> i32 {
        1
    }

    #[cfg(not(unix))]
    fn o_wronly() -> i32 {
        1
    }

    #[cfg(unix)]
    fn o_rdwr() -> i32 {
        2
    }

    #[cfg(not(unix))]
    fn o_rdwr() -> i32 {
        2
    }

    #[cfg(unix)]
    fn o_accmode() -> i32 {
        3
    }

    #[cfg(not(unix))]
    fn o_accmode() -> i32 {
        3
    }

    #[cfg(unix)]
    fn o_creat() -> i32 {
        64
    }

    #[cfg(not(unix))]
    fn o_creat() -> i32 {
        64
    }

    #[cfg(unix)]
    fn o_excl() -> i32 {
        128
    }

    #[cfg(not(unix))]
    fn o_excl() -> i32 {
        128
    }

    #[cfg(unix)]
    fn o_trunc() -> i32 {
        512
    }

    #[cfg(not(unix))]
    fn o_trunc() -> i32 {
        512
    }

    #[cfg(unix)]
    fn o_append() -> i32 {
        1024
    }

    #[cfg(not(unix))]
    fn o_append() -> i32 {
        1024
    }

    #[cfg(unix)]
    fn o_nonblock() -> i32 {
        2048
    }

    #[cfg(unix)]
    fn o_cloexec() -> i32 {
        0o2000000
    }
}

fn path_has_trailing_slash(path: &Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    !text.is_empty() && text.ends_with('/')
}
