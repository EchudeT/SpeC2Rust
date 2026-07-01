mod progname;
mod pwd;
mod xgetcwd;
mod root_dev_ino;
mod same_inode;
mod xalloc_die;
use std::env;
use std::process::ExitCode;

use crate::progname::Progname;
use crate::pwd::Pwd;
use crate::xgetcwd::Xgetcwd;

pub struct Main;

impl Main {
    pub fn run() -> ExitCode {
        let args: Vec<String> = env::args().collect();

        Progname::set_program_name(args.first().map(String::as_str));

        let mut logical = env::var_os("POSIXLY_CORRECT").is_some();
        let mut saw_non_option = false;

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-L" | "--logical" => logical = true,
                "-P" | "--physical" => logical = false,
                "--help" => Pwd::print_usage_and_exit(0),
                "--version" => {
                    return Pwd::main(&args);
                }
                _ if arg.starts_with('-') => Pwd::print_usage_and_exit(1),
                _ => saw_non_option = true,
            }
        }

        if saw_non_option {
            eprintln!("ignoring non-option arguments");
        }

        if logical {
            if let Some(path) = Pwd::logical_getcwd() {
                println!("{}", path.display());
                return ExitCode::SUCCESS;
            }
        }

        match Xgetcwd::getcwd() {
            Ok(path) => {
                println!("{}", path.display());
                ExitCode::SUCCESS
            }
            Err(_) => {
                let mut file_name = Pwd::file_name();
                match Pwd::robust_getcwd(&mut file_name) {
                    Ok(()) => {
                        println!("{}", file_name.as_path().display());
                        ExitCode::SUCCESS
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        ExitCode::from(1)
                    }
                }
            }
        }
    }
}

fn main() {
    std::process::exit(match Main::run() {
        ExitCode::SUCCESS => 0,
        _ => 1,
    });
}
