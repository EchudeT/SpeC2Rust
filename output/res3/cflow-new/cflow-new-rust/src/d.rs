use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct D;

impl D {
    const DEFAULT_MAX_LEVEL: usize = usize::MAX;
    const IGNORED_NAMES: &'static [&'static str] = &[".", ".."];

    pub fn is_dir(path: &Path) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => metadata.is_dir(),
            Err(error) => {
                eprintln!("{}: {}", path.display(), error);
                false
            }
        }
    }

    pub fn is_ignored_name(name: &OsStr) -> bool {
        match name.to_str() {
            Some(text) => Self::IGNORED_NAMES.iter().any(|ignored| text == *ignored),
            None => false,
        }
    }

    pub fn print_dir(level: usize, name: &Path) -> io::Result<()> {
        Self::printdir_with_limit(level, name, Self::DEFAULT_MAX_LEVEL)
    }

    fn printdir_with_limit(level: usize, name: &Path, max_level: usize) -> io::Result<()> {
        let dir = fs::read_dir(name)?;

        for entry_result in dir {
            let entry = entry_result?;
            let entry_name = entry.file_name();

            print!(
                "{:width$}{}",
                "",
                entry_name.to_string_lossy(),
                width = level
            );

            if Self::is_ignored_name(&entry_name) {
                println!();
                continue;
            }

            let child_path: PathBuf = name.join(&entry_name);
            if Self::is_dir(&child_path) {
                print!("/");
                if level + 1 == max_level {
                    println!();
                } else {
                    println!(" contains:");
                    Self::printdir_with_limit(level + 1, &child_path, max_level)?;
                }
            } else {
                println!();
            }
        }

        io::stdout().flush()
    }

    pub fn main(args: &[String]) -> i32 {
        if args.len() < 2 {
            eprintln!("usage: d [-MAX] DIR [DIR...]");
            return 1;
        }

        let mut max_level = Self::DEFAULT_MAX_LEVEL;
        let mut start = 1;

        if let Some(first) = args.get(1) {
            if let Some(rest) = first.strip_prefix('-') {
                if first != "--" {
                    max_level = rest.parse::<usize>().unwrap_or(0);
                }
                start = 2;
            }
        }

        for arg in args.iter().skip(start) {
            if let Err(error) = Self::printdir_with_limit(0, Path::new(arg), max_level) {
                eprintln!("{}: {}", arg, error);
                return 1;
            }
        }

        1
    }
}
