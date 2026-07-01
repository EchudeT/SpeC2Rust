use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use crate::progname::Progname;
use crate::root_dev_ino::RootDevIno;
use crate::same_inode::SameInode;
use crate::xgetcwd::Xgetcwd;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileName {
    value: PathBuf,
}

impl FileName {
    fn new() -> Self {
        Self {
            value: PathBuf::new(),
        }
    }

    fn prepend_component<S: AsRef<Path>>(&mut self, component: S) {
        let component = component.as_ref();
        if component.as_os_str().is_empty() {
            if self.value.as_os_str().is_empty() {
                self.value.push(Path::new("/"));
            }
            return;
        }

        if self.value.as_os_str().is_empty() {
            let mut absolute = PathBuf::from("/");
            absolute.push(component);
            self.value = absolute;
        } else {
            let mut next = PathBuf::from("/");
            next.push(component);
            for c in self.value.components() {
                if let Component::Normal(part) = c {
                    next.push(part);
                }
            }
            self.value = next;
        }
    }

    pub fn as_path(&self) -> &Path {
        if self.value.as_os_str().is_empty() {
            Path::new("")
        } else {
            &self.value
        }
    }

    fn into_path_buf(self) -> PathBuf {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevIno {
    pub dev: u64,
    pub ino: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentDir {
    pub path: PathBuf,
}

pub struct Pwd;

impl Pwd {
    pub fn print_usage_and_exit(status: i32) -> ! {
        let program = env::args().next().unwrap_or_else(|| String::from("pwd"));
        if status != 0 {
            let _ = writeln!(
                io::stderr(),
                "Try '{} --help' for more information.",
                program
            );
        } else {
            let mut out = io::stdout();
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
        }
        std::process::exit(status);
    }


    pub fn file_name_prepend(file_name: &mut FileName, segment: &Path) {
        file_name.prepend_component(segment);
    }

    pub fn nth_parent(n: usize) -> PathBuf {
        if n == 0 {
            return PathBuf::from(".");
        }
        let mut path = PathBuf::new();
        for _ in 0..n {
            path.push("..");
        }
        path
    }

    pub fn find_dir_entry(
        current_metadata: &fs::Metadata,
        file_name: &mut FileName,
        parent_height: usize,
    ) -> io::Result<fs::Metadata> {
        let parent_ref = Self::nth_parent(parent_height);
        let parent_dir = fs::read_dir(".")?;
        let parent_metadata = fs::metadata(".")?;

        let use_metadata_match = true;
        let mut found_name: Option<OsString> = None;

        for entry_result in parent_dir {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let name = entry.file_name();
            if name == "." || name == ".." {
                continue;
            }

            let entry_metadata = if use_metadata_match {
                match fs::symlink_metadata(entry.path()) {
                    Ok(metadata) => metadata,
                    Err(_) => continue,
                }
            } else {
                match entry.metadata() {
                    Ok(metadata) => metadata,
                    Err(_) => continue,
                }
            };

            if SameInode::check(&entry_metadata, current_metadata) {
                found_name = Some(name);
                break;
            }
        }

        match found_name {
            Some(name) => {
                Self::file_name_prepend(file_name, Path::new(&name));
                env::set_current_dir(&parent_ref)?;
                Ok(parent_metadata)
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "couldn't find directory entry in {} with matching i-node",
                    parent_ref.display()
                ),
            )),
        }
    }

    pub fn robust_getcwd(file_name: &mut FileName) -> io::Result<()> {
        let root = RootDevIno::get().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "failed to get attributes of root directory",
            )
        })?;

        let original_dir = env::current_dir()?;
        let mut current_metadata = fs::metadata(".")?;
        let mut height = 1usize;

        loop {
            let current = Self::dev_ino(&current_metadata);
            if current == root {
                break;
            }

            let parent_ref = Self::nth_parent(height);
            let child_dir = env::current_dir()?;
            let entries = fs::read_dir(&parent_ref)?;
            let parent_metadata = fs::metadata(&parent_ref)?;

            let mut found_name: Option<OsString> = None;
            for entry_result in entries {
                let entry = match entry_result {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                let name = entry.file_name();
                if name == "." || name == ".." {
                    continue;
                }

                let candidate_path = parent_ref.join(&name);
                let entry_metadata = match fs::symlink_metadata(&candidate_path) {
                    Ok(metadata) => metadata,
                    Err(_) => continue,
                };

                let candidate_canonical = match fs::canonicalize(&candidate_path) {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                if candidate_canonical == child_dir || SameInode::check(&entry_metadata, &current_metadata) {
                    found_name = Some(name);
                    break;
                }
            }

            match found_name {
                Some(name) => {
                    Self::file_name_prepend(file_name, Path::new(&name));
                    current_metadata = parent_metadata;
                    height += 1;
                }
                None => {
                    let _ = env::set_current_dir(&original_dir);
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "couldn't find directory entry in {} with matching i-node",
                            parent_ref.display()
                        ),
                    ));
                }
            }
        }

        if file_name.as_path().as_os_str().is_empty() {
            Self::file_name_prepend(file_name, Path::new(""));
        }

        env::set_current_dir(original_dir)?;
        Ok(())
    }

    pub fn logical_getcwd() -> Option<PathBuf> {
        let wd = env::var_os("PWD")?;
        let path = PathBuf::from(&wd);

        if !path.is_absolute() {
            return None;
        }

        let text = path.to_string_lossy();
        if text.contains("/./")
            || text.ends_with("/.")
            || text.contains("/../")
            || text.ends_with("/..")
        {
            return None;
        }

        let stated = fs::metadata(&path).ok()?;
        let actual = fs::metadata(".").ok()?;
        if SameInode::check(&stated, &actual) {
            Some(path)
        } else {
            None
        }
    }

    pub fn main(args: &[String]) -> ExitCode {
        if let Some(argv0) = args.first() {
            Progname::set_program_name(Some(argv0));
        }

        let mut logical = env::var_os("POSIXLY_CORRECT").is_some();
        let mut saw_non_option = false;

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-L" | "--logical" => logical = true,
                "-P" | "--physical" => logical = false,
                "--help" => Self::print_usage_and_exit(0),
                "--version" => {
                    println!("pwd");
                    return ExitCode::SUCCESS;
                }
                _ if arg.starts_with('-') => Self::print_usage_and_exit(1),
                _ => saw_non_option = true,
            }
        }

        if saw_non_option {
            let _ = writeln!(io::stderr(), "ignoring non-option arguments");
        }

        if logical {
            if let Some(wd) = Self::logical_getcwd() {
                println!("{}", wd.display());
                return ExitCode::SUCCESS;
            }
        }

        if let Ok(wd) = Xgetcwd::getcwd() {
            println!("{}", wd.display());
            return ExitCode::SUCCESS;
        }

        let mut file_name = Self::file_name();
        match Self::robust_getcwd(&mut file_name) {
            Ok(()) => {
                println!("{}", file_name.as_path().display());
                ExitCode::SUCCESS
            }
            Err(err) => {
                let _ = writeln!(io::stderr(), "pwd: {}", err);
                ExitCode::from(1)
            }
        }
    }

    pub fn file_name() -> FileName {
        FileName::new()
    }

    pub fn dev_ino(metadata: &fs::Metadata) -> DevIno {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
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

    pub fn main_root_pwd(args: &[String]) -> ExitCode {
        Self::main(args)
    }

    pub fn current_dir() -> io::Result<CurrentDir> {
        env::current_dir().map(|path| CurrentDir { path })
    }

    pub fn main_root_file_name_03() -> FileName {
        Self::file_name()
    }
}
