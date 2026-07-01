use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct D;

impl D {
    const DEFAULT_MAX_LEVEL: usize = usize::MAX;
    const IGNORED_NAMES: &'static [&'static str] = &[".", ".."];

    pub fn is_dir(path: impl AsRef<Path>) -> bool {
        match fs::metadata(path.as_ref()) {
            Ok(metadata) => metadata.is_dir(),
            Err(err) => {
                let _ = writeln!(io::stderr(), "{}: {}", path.as_ref().display(), err);
                false
            }
        }
    }

    pub fn is_ignored_name(name: impl AsRef<OsStr>) -> bool {
        match name.as_ref().to_str() {
            Some(text) => Self::IGNORED_NAMES.iter().any(|ignored| text == *ignored),
            None => false,
        }
    }

    pub fn print_dir(level: usize, name: impl AsRef<Path>) {
        Self::printdir_with_limit(level, name.as_ref(), Self::DEFAULT_MAX_LEVEL)
            .unwrap_or_else(|err| {
                let _ = writeln!(io::stderr(), "{}: {}", name.as_ref().display(), err);
                std::process::exit(1);
            });
    }

    pub fn main(args: &[String]) -> i32 {
        if args.len() < 2 {
            let _ = writeln!(io::stderr(), "usage: d [-MAX] DIR [DIR...]");
            return 1;
        }

        let mut max_level = Self::DEFAULT_MAX_LEVEL;
        let mut start = 1;

        if let Some(first) = args.get(1) {
            if let Some(rest) = first.strip_prefix('-') {
                if first != "--" && !rest.is_empty() {
                    max_level = rest.parse::<usize>().unwrap_or(0);
                }
                start = 2;
            }
        }

        for arg in args.iter().skip(start) {
            if let Err(err) = Self::printdir_with_limit(0, Path::new(arg), max_level) {
                let _ = writeln!(io::stderr(), "{}: {}", arg, err);
                std::process::exit(1);
            }
        }

        1
    }

    fn printdir_with_limit(level: usize, name: &Path, max_level: usize) -> io::Result<()> {
        let entries = fs::read_dir(name)?;

        for entry_result in entries {
            let entry = entry_result?;
            let entry_name = entry.file_name();
            let display_name = entry_name.to_string_lossy();

            print!("{:width$}{}", "", display_name, width = level);

            if Self::is_ignored_name(&entry_name) {
                println!();
                continue;
            }

            let entry_path = entry.path();
            if Self::is_dir(&entry_path) {
                print!("/");
                if level + 1 == max_level {
                    println!();
                } else {
                    println!(" contains:");
                    Self::printdir_with_limit(level + 1, &entry_path, max_level)?;
                }
            } else {
                println!();
            }
        }

        Ok(())
    }
}
