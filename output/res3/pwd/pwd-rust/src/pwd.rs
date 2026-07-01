use crate::closeout::Closeout;
use crate::progname::Progname;
use crate::same_inode::SameInode;
use crate::xgetcwd::Xgetcwd;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub struct Pwd;

#[derive(Clone, Debug, Default)]
pub struct FileName {
    value: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DevIno {
    pub dev: u64,
    pub ino: u64,
}

impl Pwd {

    pub fn print_usage(success: bool) {
        let program = env::args()
            .next()
            .and_then(|arg| {
                Path::new(&arg)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "pwd".to_string());

        if success {
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
            let mut err = io::stderr().lock();
            let _ = writeln!(err, "Try '{} --help' for more information.", program);
        }
    }


    pub fn file_name_prepend(file_name: &mut FileName, segment: &str) {
        if segment.is_empty() {
            if file_name.value.is_empty() {
                file_name.value.push('/');
            } else if !file_name.value.starts_with('/') {
                file_name.value.insert(0, '/');
            }
            return;
        }

        if file_name.value.is_empty() {
            file_name.value.push('/');
            file_name.value.push_str(segment);
        } else {
            let mut next = String::with_capacity(segment.len() + 1 + file_name.value.len());
            next.push('/');
            next.push_str(segment);
            next.push_str(&file_name.value);
            file_name.value = next;
        }
    }

    pub fn nth_parent(n: usize) -> String {
        if n == 0 {
            return String::new();
        }

        let mut buf = String::with_capacity(3 * n);
        for i in 0..n {
            if i + 1 == n {
                buf.push_str("..");
            } else {
                buf.push_str("../");
            }
        }
        buf
    }

    pub fn find_dir_entry(
        dot: &mut DevIno,
        file_name: &mut FileName,
        parent_height: usize,
    ) -> io::Result<()> {
        let current = env::current_dir()?;
        let parent = current
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("/"));

        let parent_meta = fs::metadata(&parent)?;
        let use_lstat = parent_meta.dev() != dot.dev;

        let entries = fs::read_dir(&parent).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("reading directory {}: {}", Self::nth_parent(parent_height), e),
            )
        })?;

        let mut found_name: Option<OsString> = None;
        let mut found_meta_dev = None;

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let name = entry.file_name();
            if name == "." || name == ".." {
                continue;
            }

            let metadata = match fs::symlink_metadata(entry.path()) {
                Ok(m) => m,
                Err(_) => continue,
            };

            if metadata.ino() != dot.ino {
                continue;
            }

            if !use_lstat || metadata.dev() == dot.dev {
                found_meta_dev = Some(parent_meta.dev());
                found_name = Some(name);
                break;
            }
        }

        let found_name = found_name.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "couldn't find directory entry in {} with matching i-node",
                    Self::nth_parent(parent_height)
                ),
            )
        })?;

        let segment = found_name.to_string_lossy().into_owned();
        Self::file_name_prepend(file_name, &segment);
        dot.dev = found_meta_dev.unwrap_or_else(|| parent_meta.dev());
        dot.ino = parent_meta.ino();
        Ok(())
    }

    pub fn robust_getcwd(file_name: &mut FileName) -> io::Result<()> {
        let original = env::current_dir()?;
        let root_meta = fs::metadata("/")?;
        let mut current_meta = fs::metadata(".")?;
        let mut current = original.clone();
        let mut height = 1usize;

        loop {
            if SameInode::from_metadata(&current_meta, &root_meta) {
                break;
            }

            Self::find_dir_entry(
                &mut DevIno {
                    dev: current_meta.dev(),
                    ino: current_meta.ino(),
                },
                file_name,
                height,
            )?;

            current = current
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("/"));
            current_meta = fs::metadata(&current)?;
            env::set_current_dir(&current)?;
            height += 1;
        }

        env::set_current_dir(original)?;

        if file_name.value.is_empty() {
            Self::file_name_prepend(file_name, "");
        }

        Ok(())
    }

    pub fn logical_getcwd() -> Option<String> {
        let wd = env::var("PWD").ok()?;
        if !wd.starts_with('/') {
            return None;
        }

        let mut p = 0usize;
        while let Some(rel) = wd[p..].find("/.") {
            let idx = p + rel;
            let bytes = wd.as_bytes();
            let after_dot = idx + 2;

            if after_dot >= bytes.len()
                || bytes[after_dot] == b'/'
                || (bytes[after_dot] == b'.'
                    && (after_dot + 1 >= bytes.len() || bytes[after_dot + 1] == b'/'))
            {
                return None;
            }

            p = idx + 1;
        }

        let wd_meta = fs::metadata(&wd).ok()?;
        let dot_meta = fs::metadata(".").ok()?;
        if SameInode::from_metadata(&wd_meta, &dot_meta) {
            Some(wd)
        } else {
            None
        }
    }

    pub fn main(args: &[String]) -> io::Result<String> {
        let argv0 = args
            .first()
            .cloned()
            .unwrap_or_else(|| "pwd".to_string());
        Progname::set_program_name(&argv0);
        Closeout::close_stdout_set_ignore_epipe(true);

        let mut logical = env::var_os("POSIXLY_CORRECT").is_some();
        let mut saw_non_option = false;

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-L" | "--logical" => logical = true,
                "-P" | "--physical" => logical = false,
                "--help" => {
                    Self::print_usage(true);
                    return Ok(String::new());
                }
                "--version" => {
                    return Ok("pwd".to_string());
                }
                _ if arg.starts_with('-') => {
                    Self::print_usage(false);
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("unrecognized option: {arg}"),
                    ));
                }
                _ => saw_non_option = true,
            }
        }

        if saw_non_option {
            let _ = writeln!(io::stderr().lock(), "ignoring non-option arguments");
        }

        if logical {
            if let Some(wd) = Self::logical_getcwd() {
                return Ok(wd);
            }
        }

        if let Some(wd) = Xgetcwd::xgetcwd() {
            return Ok(wd);
        }

        let mut file_name = FileName::default();
        let robust = Self::robust_getcwd(&mut file_name).map(|_| file_name.value.clone());
        robust
    }
}
