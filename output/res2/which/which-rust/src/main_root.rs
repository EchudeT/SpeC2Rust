use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const VERSION: &str = "2.21";

fn print_usage(program: &str, to_stderr: bool) {
    let mut out = String::new();
    out.push_str(&format!("Usage: {} [options] [--] COMMAND [...]\n", program));
    out.push_str("Write the full path of COMMAND(s) to standard output.\n\n");
    out.push_str("  --version, -[vV] Print version and exit successfully.\n");
    out.push_str("  --help,          Print this help and exit successfully.\n");
    out.push_str("  --skip-dot       Skip directories in PATH that start with a dot.\n");
    out.push_str("  --skip-tilde     Skip directories in PATH that start with a tilde.\n");
    out.push_str("  --show-dot       Don't expand a dot to current directory in output.\n");
    out.push_str("  --show-tilde     Output a tilde for HOME directory for non-root.\n");
    out.push_str("  --tty-only       Stop processing options on the right if not on tty.\n");
    out.push_str("  --all, -a        Print all matches in PATH, not just the first\n");
    out.push_str("  --read-alias, -i Read list of aliases from stdin.\n");
    out.push_str("  --skip-alias     Ignore option --read-alias; don't read stdin.\n");
    out.push_str("  --read-functions Read shell functions from stdin.\n");
    out.push_str("  --skip-functions Ignore option --read-functions; don't read stdin.\n\n");
    out.push_str("Recommended use is to write the output of (alias; declare -f) to standard\n");
    out.push_str("input, so that which can show aliases and shell functions. See which(1) for\n");
    out.push_str("examples.\n\n");
    out.push_str("If the options --read-alias and/or --read-functions are specified then the\n");
    out.push_str("output can be a full alias or function definition, optionally followed by\n");
    out.push_str("the full path of each command used inside of those.\n\n");
    out.push_str("Report bugs to <which-bugs@gnu.org>.\n");

    if to_stderr {
        eprint!("{}", out);
    } else {
        print!("{}", out);
    }
}

fn print_version() {
    println!(
        "GNU which v{}, Copyright (C) 1999 - 2015 Carlo Wood.",
        VERSION
    );
    println!("GNU which comes with ABSOLUTELY NO WARRANTY;");
    println!("This program is free software; your freedom to use, change");
    println!("and distribute this program is protected by the GPL.");
}

fn print_fail(program: &str, name: &str, path_list: &str) {
    eprintln!("{}: no {} in ({})", program, name, path_list);
}

fn is_absolute_program(name: &str) -> bool {
    name.contains('/') || name.starts_with('~')
}

fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0),
        Err(_) => false,
    }
}

fn path_clean_up(path: &Path) -> String {
    fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn make_full_pathname(dir: &str, name: &str) -> PathBuf {
    if dir.is_empty() {
        PathBuf::from(name)
    } else {
        Path::new(dir).join(name)
    }
}

fn search_path(name: &str, path_list: &str, show_all: bool) -> Vec<String> {
    let mut results = Vec::new();

    if is_absolute_program(name) {
        let path = PathBuf::from(name);
        if is_executable(&path) {
            results.push(path_clean_up(&path));
        }
        return results;
    }

    for dir in path_list.split(':') {
        let candidate = make_full_pathname(dir, name);
        if is_executable(&candidate) {
            results.push(path_clean_up(&candidate));
            if !show_all {
                break;
            }
        }
    }

    results
}

pub fn main_entry(args: &[String]) -> i32 {
    let program = args
        .first()
        .cloned()
        .unwrap_or_else(|| String::from("which"));

    let mut show_all = false;
    let mut operands: Vec<String> = Vec::new();
    let mut parse_options = true;

    for arg in args.iter().skip(1) {
        if parse_options && arg == "--" {
            parse_options = false;
            continue;
        }

        if parse_options && arg.starts_with('-') && arg.len() > 1 {
            match arg.as_str() {
                "--help" => {
                    print_usage(&program, false);
                    return 0;
                }
                "--version" | "-v" | "-V" => {
                    print_version();
                    return 0;
                }
                "--all" | "-a" => {
                    show_all = true;
                }
                "--skip-dot" | "--skip-tilde" | "--show-dot" | "--show-tilde" | "--tty-only"
                | "--read-alias" | "-i" | "--skip-alias" | "--read-functions"
                | "--skip-functions" => {}
                _ if arg.starts_with("--") => {
                    eprintln!("{}: unrecognized option '{}'", program, arg);
                    print_usage(&program, true);
                    return 1;
                }
                _ => operands.push(arg.clone()),
            }
        } else {
            operands.push(arg.clone());
        }
    }

    if operands.is_empty() {
        print_usage(&program, true);
        return -1;
    }

    let path_list = env::var("PATH").unwrap_or_default();
    let mut fail_count = 0;

    for operand in operands {
        let matches = search_path(&operand, &path_list, show_all);
        if matches.is_empty() {
            let (fail_name, fail_path) = if is_absolute_program(&operand) {
                let path = Path::new(&operand);
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&operand)
                    .to_string();
                let parent = path
                    .parent()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                (name, parent)
            } else {
                (operand.clone(), path_list.clone())
            };
            print_fail(&program, &fail_name, &fail_path);
            fail_count += 1;
        } else {
            for found in matches {
                println!("{}", found);
            }
        }
    }

    fail_count
}
