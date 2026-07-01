use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

pub struct Unistd;

impl Unistd {
    pub fn current_dir() -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    pub fn change_dir(path: impl AsRef<Path>) -> io::Result<()> {
        std::env::set_current_dir(path)
    }

    pub fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    pub fn remove_dir(path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::remove_dir(path)
    }

    pub fn remove_path(path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        match std::fs::symlink_metadata(path) {
            Ok(metadata) => {
                if metadata.file_type().is_dir() {
                    std::fs::remove_dir(path)
                } else {
                    std::fs::remove_file(path)
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn program_arguments() -> Vec<OsString> {
        std::env::args_os().collect()
    }

    pub fn temporary_directory() -> PathBuf {
        std::env::temp_dir()
    }

    pub fn is_terminal_stdout() -> bool {
        use std::io::IsTerminal;
        std::io::stdout().is_terminal()
    }

    pub fn is_terminal_stderr() -> bool {
        use std::io::IsTerminal;
        std::io::stderr().is_terminal()
    }

    pub fn is_terminal_stdin() -> bool {
        use std::io::IsTerminal;
        std::io::stdin().is_terminal()
    }
}
