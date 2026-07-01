use std::fs::{File, OpenOptions};
use std::io;
#[cfg(unix)]
use std::os::fd::{AsRawFd, OwnedFd};
#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

pub struct Open;

impl Open {
    pub fn orig_open(path: impl AsRef<Path>, flags: i32, mode: u32) -> io::Result<File> {
        Self::open_file(path, flags, mode)
    }

    pub fn open_file(path: impl AsRef<Path>, flags: i32, mode: u32) -> io::Result<File> {
        let path = path.as_ref();

        #[cfg(windows)]
        let path = if path == Path::new("/dev/null") {
            Path::new("NUL")
        } else {
            path
        };

        if Self::requires_trailing_slash_write_failure(flags) && Self::has_trailing_slash(path) {
            return Err(io::Error::from_raw_os_error(Self::eisdir_code()));
        }

        let mut options = OpenOptions::new();
        Self::apply_access_mode(&mut options, flags);
        Self::apply_creation_mode(&mut options, flags);
        #[cfg(unix)]
        {
            options.mode(mode);
            options.custom_flags(Self::custom_flags(flags));
        }

        let first_try = options.open(path);

        let mut file = match first_try {
            Ok(file) => file,
            Err(err) => {
                if Self::wants_cloexec(flags) && Self::is_invalid_flag_error(&err) {
                    let mut retry = OpenOptions::new();
                    Self::apply_access_mode(&mut retry, flags);
                    Self::apply_creation_mode(&mut retry, flags);
                    #[cfg(unix)]
                    {
                        retry.mode(mode);
                        retry.custom_flags(Self::custom_flags_without_cloexec(flags));
                    }
                    retry.open(path)?
                } else {
                    return Err(err);
                }
            }
        };

        #[cfg(unix)]
        if Self::wants_cloexec(flags) {
            let _ = crate::cloexec::Cloexec::flag(&file, true);
        }

        #[cfg(unix)]
        if Self::has_trailing_slash(path) {
            let metadata = file.metadata()?;
            if !metadata.is_dir() {
                drop(file);
                return Err(io::Error::from_raw_os_error(Self::enotdir_code()));
            }
        }

        #[cfg(unix)]
        if Self::should_emulate_directory_open(path, flags, &file).is_some() {
            let replacement = Self::open_file(Path::new("/dev/null"), flags, mode)?;
            let original = path.to_path_buf();
            let _ = original;
            file = replacement;
        }

        Ok(file)
    }

    fn has_trailing_slash(path: &Path) -> bool {
        let text = path.as_os_str().to_string_lossy();
        !text.is_empty() && text.ends_with('/')
    }

    fn wants_cloexec(flags: i32) -> bool {
        #[cfg(unix)]
        {
            flags & libc_like::o_cloexec() != 0
        }
        #[cfg(not(unix))]
        {
            let _ = flags;
            false
        }
    }

    fn requires_trailing_slash_write_failure(flags: i32) -> bool {
        let accmode = flags & libc_like::o_accmode();
        (flags & libc_like::o_creat() != 0)
            || accmode == libc_like::o_rdwr()
            || accmode == libc_like::o_wronly()
    }

    fn apply_access_mode(options: &mut OpenOptions, flags: i32) {
        let accmode = flags & libc_like::o_accmode();
        match accmode {
            x if x == libc_like::o_wronly() => {
                options.write(true);
            }
            x if x == libc_like::o_rdwr() => {
                options.read(true).write(true);
            }
            _ => {
                options.read(true);
            }
        }
    }

    fn apply_creation_mode(options: &mut OpenOptions, flags: i32) {
        if flags & libc_like::o_creat() != 0 {
            options.create(true);
        }
        if flags & libc_like::o_excl() != 0 {
            options.create_new(true);
        }
        if flags & libc_like::o_trunc() != 0 {
            options.truncate(true);
        }
        if flags & libc_like::o_append() != 0 {
            options.append(true);
        }
    }

    #[cfg(unix)]
    fn custom_flags(flags: i32) -> i32 {
        let mut filtered = flags;
        filtered &= !libc_like::o_accmode();
        filtered &= !libc_like::o_creat();
        filtered &= !libc_like::o_excl();
        filtered &= !libc_like::o_trunc();
        filtered &= !libc_like::o_append();
        filtered &= !libc_like::o_nonblock();
        filtered
    }

    #[cfg(unix)]
    fn custom_flags_without_cloexec(flags: i32) -> i32 {
        Self::custom_flags(flags & !libc_like::o_cloexec())
    }

    fn is_invalid_flag_error(err: &io::Error) -> bool {
        matches!(err.raw_os_error(), Some(code) if code == Self::einval_code())
    }

    #[cfg(unix)]
    fn should_emulate_directory_open(path: &Path, flags: i32, file: &File) -> Option<PathBuf> {
        let _ = file;
        let accmode = flags & libc_like::o_accmode();
        if !(accmode == libc_like::o_rdonly() || accmode == libc_like::o_search()) {
            return None;
        }
        let metadata = path.metadata().ok()?;
        if metadata.is_dir() {
            None
        } else if metadata.file_type().is_symlink() {
            None
        } else {
            None
        }
    }

    fn einval_code() -> i32 {
        22
    }

    fn eisdir_code() -> i32 {
        21
    }

    fn enotdir_code() -> i32 {
        20
    }
}

#[cfg(unix)]
mod libc_like {
    pub fn o_rdonly() -> i32 {
        0
    }

    pub fn o_wronly() -> i32 {
        1
    }

    pub fn o_rdwr() -> i32 {
        2
    }

    pub fn o_accmode() -> i32 {
        3
    }

    pub fn o_creat() -> i32 {
        0o100
    }

    pub fn o_excl() -> i32 {
        0o200
    }

    pub fn o_trunc() -> i32 {
        0o1000
    }

    pub fn o_append() -> i32 {
        0o2000
    }

    pub fn o_nonblock() -> i32 {
        0o4000
    }

    pub fn o_cloexec() -> i32 {
        0o2000000
    }

    pub fn o_search() -> i32 {
        o_rdonly()
    }
}

#[cfg(not(unix))]
mod libc_like {
    pub fn o_rdonly() -> i32 {
        0
    }

    pub fn o_wronly() -> i32 {
        1
    }

    pub fn o_rdwr() -> i32 {
        2
    }

    pub fn o_accmode() -> i32 {
        3
    }

    pub fn o_creat() -> i32 {
        0x0100
    }

    pub fn o_excl() -> i32 {
        0x0400
    }

    pub fn o_trunc() -> i32 {
        0x0200
    }

    pub fn o_append() -> i32 {
        0x0008
    }

    pub fn o_nonblock() -> i32 {
        0
    }

    pub fn o_cloexec() -> i32 {
        0
    }

    pub fn o_search() -> i32 {
        o_rdonly()
    }
}
