use std::fs::{self, Metadata};
use std::io;
use std::path::Path;

pub struct Stat;

impl Stat {
    pub fn orig_stat(path: impl AsRef<Path>) -> io::Result<Metadata> {
        fs::metadata(path.as_ref())
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
        if q == 2 || q == bytes.len() {
            return false;
        }

        let r_start = q + 1;
        if r_start >= bytes.len() {
            return false;
        }

        let mut r = r_start;
        while r < bytes.len() && !Self::is_slash(bytes[r]) {
            r += 1;
        }

        r > r_start && r == bytes.len()
    }

    pub fn rpl_stat(path: impl AsRef<Path>) -> io::Result<Metadata> {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy();

        let metadata = Self::orig_stat(path_ref)?;

        if Self::has_trailing_slash(&path_str) && !metadata.is_dir() {
            return Err(io::Error::from(io::ErrorKind::NotADirectory));
        }

        Ok(metadata)
    }

    fn is_slash(b: u8) -> bool {
        b == b'/' || b == b'\\'
    }

    fn has_trailing_slash(path: &str) -> bool {
        path.as_bytes()
            .last()
            .is_some_and(|b| Self::is_slash(*b))
    }
}
