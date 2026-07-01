use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};

pub struct Stat;

impl Stat {
    pub fn orig_stat(path: impl AsRef<Path>) -> io::Result<Metadata> {
        fs::metadata(path)
    }

    pub fn is_unc_root(path: &str) -> bool {
        let bytes = path.as_bytes();
        if bytes.len() < 5 {
            return false;
        }
        if !Self::is_slash(bytes[0]) || !Self::is_slash(bytes[1]) {
            return false;
        }

        let mut q = 2;
        while q < bytes.len() && !Self::is_slash(bytes[q]) {
            q += 1;
        }
        if q == 2 || q >= bytes.len() {
            return false;
        }

        q += 1;
        let r_start = q;
        while q < bytes.len() && !Self::is_slash(bytes[q]) {
            q += 1;
        }

        q > r_start && q == bytes.len()
    }

    pub fn rpl_stat(path: impl AsRef<Path>) -> io::Result<Metadata> {
        let path = path.as_ref();

        #[cfg(windows)]
        {
            return Self::rpl_stat_windows(path);
        }

        #[cfg(not(windows))]
        {
            let metadata = Self::orig_stat(path)?;
            if Self::has_trailing_slash(path) && !metadata.is_dir() {
                return Err(io::Error::from_raw_os_error(notdir_code()));
            }
            Ok(metadata)
        }
    }

    fn is_slash(b: u8) -> bool {
        b == b'/' || b == b'\\'
    }

    fn has_trailing_slash(path: &Path) -> bool {
        let text = path.as_os_str().to_string_lossy();
        let bytes = text.as_bytes();
        !bytes.is_empty() && Self::is_slash(bytes[bytes.len() - 1])
    }

    #[cfg(windows)]
    fn rpl_stat_windows(path: &Path) -> io::Result<Metadata> {
        let original = path.as_os_str().to_string_lossy();
        let normalized = Self::normalize_windows_name(&original)?;

        match fs::metadata(PathBuf::from(&normalized)) {
            Ok(metadata) => {
                if normalized.check_dir && !metadata.is_dir() {
                    Err(io::Error::from_raw_os_error(notdir_code()))
                } else {
                    Ok(metadata)
                }
            }
            Err(primary_err) => {
                if normalized.is_root_dir || Self::is_unc_root(&normalized.rname) {
                    return Err(primary_err);
                }

                if normalized.rname.contains('?') || normalized.rname.contains('*') {
                    return Err(io::Error::from_raw_os_error(enoent_code()));
                }

                let fallback = fs::metadata(PathBuf::from(&normalized.rname));
                match fallback {
                    Ok(metadata) => {
                        if normalized.check_dir && !metadata.is_dir() {
                            Err(io::Error::from_raw_os_error(notdir_code()))
                        } else {
                            Ok(metadata)
                        }
                    }
                    Err(_) => Err(primary_err),
                }
            }
        }
    }

    #[cfg(windows)]
    fn normalize_windows_name(name: &str) -> io::Result<NormalizedWindowsPath> {
        let collapsed = Self::collapse_leading_slashes(name);
        let drive_prefix_len = Self::drive_prefix_len(&collapsed);
        let len = collapsed.len();

        let mut rlen = len;
        let mut check_dir = false;
        while rlen > drive_prefix_len {
            let b = collapsed.as_bytes()[rlen - 1];
            if !Self::is_slash(b) {
                break;
            }
            check_dir = true;
            if rlen == drive_prefix_len + 1 {
                break;
            }
            rlen -= 1;
        }

        if !check_dir && rlen == drive_prefix_len {
            return Err(io::Error::from_raw_os_error(enoent_code()));
        }

        if rlen == 1 && collapsed.as_bytes().first().is_some_and(|b| Self::is_slash(*b)) && len >= 2 {
            return Err(io::Error::from_raw_os_error(enoent_code()));
        }

        let rname = if rlen == len {
            collapsed
        } else {
            collapsed[..rlen].to_string()
        };

        let is_root_dir =
            rlen == drive_prefix_len + 1 && rname.as_bytes().get(drive_prefix_len).is_some_and(|b| Self::is_slash(*b));

        Ok(NormalizedWindowsPath {
            rname,
            check_dir,
            is_root_dir,
        })
    }

    #[cfg(windows)]
    fn collapse_leading_slashes(name: &str) -> String {
        let bytes = name.as_bytes();
        if bytes.len() >= 3 && Self::is_slash(bytes[0]) && Self::is_slash(bytes[1]) && Self::is_slash(bytes[2]) {
            let mut idx = 2;
            while idx + 1 < bytes.len() && Self::is_slash(bytes[idx + 1]) {
                idx += 1;
            }
            name[idx..].to_string()
        } else {
            name.to_string()
        }
    }

    #[cfg(windows)]
    fn drive_prefix_len(name: &str) -> usize {
        let bytes = name.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            2
        } else {
            0
        }
    }
}

#[cfg(windows)]
struct NormalizedWindowsPath {
    rname: String,
    check_dir: bool,
    is_root_dir: bool,
}

fn enoent_code() -> i32 {
    2
}

fn notdir_code() -> i32 {
    #[cfg(windows)]
    {
        20
    }
    #[cfg(not(windows))]
    {
        20
    }
}
