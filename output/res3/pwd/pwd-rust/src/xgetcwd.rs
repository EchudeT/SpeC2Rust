use std::env;
use std::io;
use std::path::PathBuf;

use crate::xalloc_die::XallocDie;

pub struct Xgetcwd;

impl Xgetcwd {
    pub fn getcwd() -> io::Result<PathBuf> {
        match Self::current_dir() {
            Ok(path) => Ok(path),
            Err(error) if error.kind() == io::ErrorKind::OutOfMemory => XallocDie::fail(),
            Err(error) => Err(error),
        }
    }

    pub fn xgetcwd() -> Option<String> {
        Self::getcwd()
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
    }

    pub fn current_dir() -> io::Result<PathBuf> {
        env::current_dir()
    }
}
