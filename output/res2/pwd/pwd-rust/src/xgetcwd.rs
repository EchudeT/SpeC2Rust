use std::env;
use std::io;
use std::path::PathBuf;

use crate::xalloc_die::XallocDie;

pub struct Xgetcwd;

impl Xgetcwd {
    pub fn getcwd() -> io::Result<PathBuf> {
        match env::current_dir() {
            Ok(path) => Ok(path),
            Err(err) if err.kind() == io::ErrorKind::OutOfMemory => XallocDie::die(),
            Err(err) => Err(err),
        }
    }
}
