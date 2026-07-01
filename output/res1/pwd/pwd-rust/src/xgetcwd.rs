use std::env;
use std::io;
use std::path::PathBuf;

use crate::xalloc_die::XallocDie;

pub struct Xgetcwd;

impl Xgetcwd {
    pub fn xgetcwd() -> Option<PathBuf> {
        match Self::current_dir() {
            Ok(path) => Some(path),
            Err(err) if err.kind() == io::ErrorKind::OutOfMemory => XallocDie::fail(),
            Err(_) => None,
        }
    }

    pub fn current_dir() -> io::Result<PathBuf> {
        env::current_dir()
    }
}
