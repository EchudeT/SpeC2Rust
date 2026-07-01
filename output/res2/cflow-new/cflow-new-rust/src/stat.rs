use std::fs::{self, Metadata};
use std::io;
use std::path::Path;

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
        Self::orig_stat(path)
    }

    fn is_slash(byte: u8) -> bool {
        byte == b'/' || byte == b'\\'
    }
}
