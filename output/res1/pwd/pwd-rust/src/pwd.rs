use crate::progname::Progname;
use crate::same_inode::SameInode;
use crate::xgetcwd::Xgetcwd;
use std::env;
use std::ffi::OsString;
use std::fs::{self, Metadata};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub struct Pwd;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileName {
    pub path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DevIno {
    pub dev: u64,
    pub ino: u64,
}

impl Pwd {
    pub fn print_usage_and_exit(status: i32) -> ! {
        if status == 0 {
            let program = env::args()
                .next()
                .and_then(|p| {
                    Path::new(&p)
                        .file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                })
                .unwrap_or_else(|| "pwd".to_string());

            let mut out = io::stdout().lock();
            let _ = writeln!(out, "Usage: {} [OPTION]...", program);
            let _ = writeln!(out, "Print the full filename of the current working directory.");
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "  -L, --logical   use PWD from environment, even if it contains symlinks"
            );
            let _ = writeln!(out, "  -P, --physical  resolve all symlinks");
            let _ = writeln!(out, "      --help      display this help and exit");
            let _ = writeln!(out, "      --version   output version information and exit");
            let _ = writeln!(out);
            let _ = writeln!(out, "If no option is specified, -P is assumed.");
        } else {
            let program = env::args()
                .next()
                .and_then(|p| {
                    Path::new(&p)
                        .file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                })
                .unwrap_or_else(|| "pwd".to_string());
            let _ = writeln!(
                io::stderr().lock(),
                "Try '{} --help' for more information.",
                program
            );
        }

        process::exit(status);
    }

    pub fn new_file_name() -> FileName {
        FileName {
            path: String::new(),
        }
    }

    pub fn file_name_prepend(file_name: &mut FileName, segment: &str) {
        if segment.is_empty() {
            if file_name.path.is_empty() {
                file_name.path.push('/');
            }
            return;
        }

        let mut new_path = String::with_capacity(segment.len() + 1 + file_name.path.len());
        new_path.push('/');
        new_path.push_str(segment);
        new_path.push_str(&file_name.path);
        file_name.path = new_path;
    }

    pub fn nth_parent(n: usize) -> String {
        if n == 0 {
            String::new()
        } else {
            "../".repeat(n).trim_end_matches('/').to_string()
        }
    }

    pub fn find_dir_entry(
        dot_metadata: &mut Metadata,
        file_name: &mut FileName,
        parent_height: usize,
    ) -> io::Result<()> {
        let parent = Path::new("..");
        env::set_current_dir(parent)?;

        let parent_metadata = fs::metadata(".")?;
        let use_metadata_match = Self::cross_device(&parent_metadata, dot_metadata);

        let mut found_name: Option<OsString> = None;
        for entry_result in fs::read_dir(".")? {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let name = entry.file_name();
            if name == "." || name == ".." {
                continue;
            }

            let entry_metadata = match fs::symlink_metadata(entry.path()) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let inode_match = if use_metadata_match {
                Self::same_inode_value(&entry_metadata, dot_metadata)
            } else {
                Self::same_inode_value(&entry_metadata, dot_metadata)
            };

            if inode_match {
                found_name = Some(name);
                break;
            }
        }

        match found_name {
            Some(name) => {
                Self::file_name_prepend(file_name, &name.to_string_lossy());
                *dot_metadata = parent_metadata;
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "couldn't find directory entry in {} with matching i-node",
                    Self::nth_parent(parent_height)
                ),
            )),
        }
    }

    pub fn robust_getcwd(file_name: &mut FileName) -> io::Result<()> {
        let original = env::current_dir()?;
        let root = PathBuf::from("/");
        let mut dot_metadata = fs::metadata(".")?;
        let root_metadata = fs::metadata(&root)?;

        while !SameInode::same(&dot_metadata, &root_metadata) {
            let height = file_name
                .path
                .bytes()
                .filter(|b| *b == b'/')
                .count()
                .saturating_add(1);
            Self::find_dir_entry(&mut dot_metadata, file_name, height)?;
        }

        if file_name.path.is_empty() {
            Self::file_name_prepend(file_name, "");
        }

        env::set_current_dir(original)?;
        Ok(())
    }

    pub fn logical_getcwd() -> Option<String> {
        let wd = env::var("PWD").ok()?;
        if !wd.starts_with('/') {
            return None;
        }

        let bytes = wd.as_bytes();
        let mut i = 0usize;
        while i + 1 < bytes.len() {
            if bytes[i] == b'/' && bytes[i + 1] == b'.' {
                let next = bytes.get(i + 2).copied();
                if next.is_none() || next == Some(b'/') {
                    return None;
                }
                if next == Some(b'.') {
                    let next2 = bytes.get(i + 3).copied();
                    if next2.is_none() || next2 == Some(b'/') {
                        return None;
                    }
                }
            }
            i += 1;
        }

        if SameInode::same_path(&wd, ".").ok()? {
            Some(wd)
        } else {
            None
        }
    }

    pub fn main(args: &[String]) -> i32 {
        Progname::set_program_name(args.first().map(String::as_str));

        let mut logical = env::var_os("POSIXLY_CORRECT").is_some();
        let mut saw_non_option = false;

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-L" | "--logical" => logical = true,
                "-P" | "--physical" => logical = false,
                "--help" => Self::print_usage_and_exit(0),
                "--version" => {
                    let _ = writeln!(io::stdout().lock(), "pwd");
                    return 0;
                }
                _ if arg.starts_with('-') => Self::print_usage_and_exit(1),
                _ => saw_non_option = true,
            }
        }

        if saw_non_option {
            let _ = writeln!(io::stderr().lock(), "ignoring non-option arguments");
        }

        if logical {
            if let Some(wd) = Self::logical_getcwd() {
                let _ = writeln!(io::stdout().lock(), "{}", wd);
                return 0;
            }
        }

        if let Some(wd) = Xgetcwd::xgetcwd() {
            let _ = writeln!(io::stdout().lock(), "{}", wd.to_string_lossy());
            return 0;
        }

        let mut file_name = Self::new_file_name();
        match Self::robust_getcwd(&mut file_name) {
            Ok(()) => {
                let _ = writeln!(io::stdout().lock(), "{}", file_name.path);
                0
            }
            Err(err) => {
                let _ = writeln!(io::stderr().lock(), "pwd: {}", err);
                1
            }
        }
    }

    pub fn file_name() -> FileName {
        Self::new_file_name()
    }

    pub fn dev_ino(metadata: &Metadata) -> DevIno {
        #[cfg(unix)]
        {
            DevIno {
                dev: metadata.dev(),
                ino: metadata.ino(),
            }
        }

        #[cfg(not(unix))]
        {
            let _ = metadata;
            DevIno { dev: 0, ino: 0 }
        }
    }

    pub fn main_root_file_name_03() -> FileName {
        Self::new_file_name()
    }

    fn same_inode_value(a: &Metadata, b: &Metadata) -> bool {
        SameInode::same(a, b)
    }

    fn cross_device(a: &Metadata, b: &Metadata) -> bool {
        #[cfg(unix)]
        {
            a.dev() != b.dev()
        }

        #[cfg(not(unix))]
        {
            let _ = (a, b);
            false
        }
    }
}

fn _normalize_parent_reference(path: &Path, n: usize) -> PathBuf {
    let mut result = PathBuf::from(path);
    for _ in 0..n {
        if !result.pop() {
            break;
        }
    }
    if result.as_os_str().is_empty() {
        PathBuf::from(match path.components().next() {
            Some(Component::RootDir) => "/",
            _ => ".",
        })
    } else {
        result
    }
}
