use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct D;

impl D {
    const DEFAULT_MAX_LEVEL: usize = usize::MAX;
    const IGNORED_NAMES: &'static [&'static str] = &[".", ".."];

    pub fn is_dir<P: AsRef<Path>>(name: P) -> bool {
        match fs::metadata(name.as_ref()) {
            Ok(metadata) => metadata.is_dir(),
            Err(err) => {
                eprintln!("{}: {}", name.as_ref().display(), err);
                false
            }
        }
    }

    pub fn is_ignored(name: &OsStr) -> bool {
        Self::IGNORED_NAMES
            .iter()
            .any(|ignored| name == OsStr::new(ignored))
    }

    pub fn printdir<P: AsRef<Path>>(level: usize, name: P) -> io::Result<()> {
        Self::printdir_with_limit(level, name.as_ref(), Self::DEFAULT_MAX_LEVEL)
    }

    pub fn main(args: &[String]) -> i32 {
        if args.len() < 2 {
            eprintln!("usage: d [-MAX] DIR [DIR...]");
            return 1;
        }

        let mut max_level = Self::DEFAULT_MAX_LEVEL;
        let mut start = 1;

        if args
            .get(1)
            .is_some_and(|arg| arg.as_bytes().first() == Some(&b'-'))
        {
            if args[1] != "--" {
                max_level = args[1][1..].parse::<usize>().unwrap_or(0);
            }
            start = 2;
        }

        for arg in &args[start..] {
            if let Err(err) = Self::printdir_with_limit(0, Path::new(arg), max_level) {
                eprintln!("{}: {}", arg, err);
                return 1;
            }
        }

        1
    }

    fn printdir_with_limit(level: usize, name: &Path, max_level: usize) -> io::Result<()> {
        let saved_cwd = env::current_dir()
            .map_err(|err| io::Error::new(err.kind(), format!("cannot save cwd: {err}")))?;

        env::set_current_dir(name)?;

        let result = (|| -> io::Result<()> {
            for entry_result in fs::read_dir(".")? {
                let entry = entry_result?;
                let entry_name = entry.file_name();

                print!("{:width$}{}", "", entry_name.to_string_lossy(), width = level);

                if Self::is_ignored(&entry_name) {
                    println!();
                    continue;
                }

                if Self::is_dir(Path::new(&entry_name)) {
                    print!("/");
                    if level + 1 == max_level {
                        println!();
                    } else {
                        println!(" contains:");
                        Self::printdir_with_limit(level + 1, Path::new(&entry_name), max_level)?;
                    }
                } else {
                    println!();
                }

                io::stdout().flush()?;
            }
            Ok(())
        })();

        let restore_result = env::set_current_dir(PathBuf::from(saved_cwd));

        match (result, restore_result) {
            (Err(err), _) => Err(err),
            (Ok(()), Err(err)) => Err(err),
            (Ok(()), Ok(())) => Ok(()),
        }
    }
}
