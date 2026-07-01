use crate::bash::Bash;
use crate::shell::Shell;
use std::env;
use std::io::{self, Write};

#[derive(Clone, Debug, Default)]
pub struct FunctionSt {
    pub name: String,
    pub lines: Vec<String>,
}

pub struct Which {
    bash: Bash,
    shell: Shell,
    program_name: String,
    show_all: bool,
    skip_dot: bool,
    skip_tilde: bool,
    show_dot: bool,
    show_tilde: bool,
    tty_only: bool,
    read_alias: bool,
    skip_alias: bool,
    read_functions: bool,
    skip_functions: bool,
    absolute_path_given: bool,
    abs_path: Option<String>,
    cwd: Option<String>,
    home: Option<String>,
    found_path_starts_with_dot: bool,
    functions: Vec<FunctionSt>,
    aliases: Vec<String>,
}

impl Default for Which {
    fn default() -> Self {
        let mut bash = Bash::new();
        bash.get_current_user_info();
        let home = bash.sh_get_home_dir();
        Self {
            bash,
            shell: Shell,
            program_name: "./which".to_string(),
            show_all: false,
            skip_dot: false,
            skip_tilde: false,
            show_dot: false,
            show_tilde: false,
            tty_only: false,
            read_alias: false,
            skip_alias: false,
            read_functions: false,
            skip_functions: false,
            absolute_path_given: false,
            abs_path: None,
            cwd: None,
            home,
            found_path_starts_with_dot: false,
            functions: Vec::new(),
            aliases: Vec::new(),
        }
    }
}

impl Which {
    pub fn function_st(name: impl Into<String>, lines: Vec<String>) -> FunctionSt {
        FunctionSt {
            name: name.into(),
            lines,
        }
    }

    pub fn alloc_buffer(size: usize) -> Vec<u8> {
        vec![0_u8; size]
    }

    pub fn resize_buffer(mut buf: Vec<u8>, size: usize) -> Vec<u8> {
        buf.resize(size, 0);
        buf
    }

    pub fn print_usage(&self, mut out: impl Write) {
        let _ = writeln!(
            out,
            "Usage: {} [options] [--] COMMAND [...]",
            self.program_name
        );
        let _ = writeln!(out, "Write the full path of COMMAND(s) to standard output.\n");
        let _ = writeln!(out, "  --version, -[vV] Print version and exit successfully.");
        let _ = writeln!(out, "  --help,          Print this help and exit successfully.");
        let _ = writeln!(out, "  --skip-dot       Skip directories in PATH that start with a dot.");
        let _ = writeln!(
            out,
            "  --skip-tilde     Skip directories in PATH that start with a tilde."
        );
        let _ = writeln!(
            out,
            "  --show-dot       Don't expand a dot to current directory in output."
        );
        let _ = writeln!(
            out,
            "  --show-tilde     Output a tilde for HOME directory for non-root."
        );
        let _ = writeln!(
            out,
            "  --tty-only       Stop processing options on the right if not on tty."
        );
        let _ = writeln!(
            out,
            "  --all, -a        Print all matches in PATH, not just the first"
        );
        let _ = writeln!(out, "  --read-alias, -i Read list of aliases from stdin.");
        let _ = writeln!(
            out,
            "  --skip-alias     Ignore option --read-alias; don't read stdin."
        );
        let _ = writeln!(out, "  --read-functions Read shell functions from stdin.");
        let _ = writeln!(
            out,
            "  --skip-functions Ignore option --read-functions; don't read stdin.\n"
        );
        let _ = writeln!(
            out,
            "Recommended use is to write the output of (alias; declare -f) to standard"
        );
        let _ = writeln!(
            out,
            "input, so that which can show aliases and shell functions. See which(1) for"
        );
        let _ = writeln!(out, "examples.\n");
        let _ = writeln!(
            out,
            "If the options --read-alias and/or --read-functions are specified then the"
        );
        let _ = writeln!(
            out,
            "output can be a full alias or function definition, optionally followed by"
        );
        let _ = writeln!(out, "the full path of each command used inside of those.\n");
        let _ = writeln!(out, "Report bugs to <which-bugs@gnu.org>.");
    }

    pub fn print_version(&self) {
        println!("GNU which v2.21, Copyright (C) 1999 - 2015 Carlo Wood.");
        println!("GNU which comes with ABSOLUTELY NO WARRANTY;");
        println!("This program is free software; your freedom to use, change");
        println!("and distribute this program is protected by the GPL.");
    }

    pub fn print_fail(&self, name: &str, path_list: &str) {
        eprintln!("{}: no {} in ({})", self.program_name, name, path_list);
    }

    pub fn print_unrecognized_option_and_usage(&self, option: &str) {
        eprintln!("{}: unrecognized option '{}'", self.program_name, option);
        eprintln!("Usage: {} [options] [--] COMMAND [...]", self.program_name);
        eprintln!("Write the full path of COMMAND(s) to standard output.");
    }

    pub fn get_current_working_directory(&mut self) -> String {
        if let Some(cwd) = &self.cwd {
            return cwd.clone();
        }

        let mut cwd = env::current_dir()
            .ok()
            .map(|p| p.to_string_lossy().into_owned())
            .or_else(|| env::var("PWD").ok())
            .unwrap_or_default();

        if !cwd.starts_with('/') {
            eprintln!("Can't get current working directory");
            std::process::exit(255);
        }

        if !cwd.ends_with('/') {
            cwd.push('/');
        }

        self.cwd = Some(cwd.clone());
        cwd
    }

    pub fn path_clean_up(&mut self, path: &str) -> String {
        let mut raw = String::new();
        if path.starts_with('/') {
            raw.push_str(path);
        } else {
            raw.push_str(&self.get_current_working_directory());
            raw.push_str(path);
        }

        let preserve_double_leading = raw.starts_with("//") && !raw.starts_with("///");
        let mut parts: Vec<&str> = Vec::new();

        for part in raw.split('/') {
            match part {
                "" => {
                    if parts.is_empty() {
                        continue;
                    }
                }
                "." => {}
                ".." => {
                    if parts.pop().is_none() {
                        return raw;
                    }
                }
                _ => parts.push(part),
            }
        }

        let mut result = String::new();
        if preserve_double_leading {
            result.push_str("//");
        } else {
            result.push('/');
        }
        result.push_str(&parts.join("/"));

        if raw.ends_with('/') && !result.ends_with('/') {
            result.push('/');
        }

        if result.is_empty() {
            "/".to_string()
        } else {
            result
        }
    }

    pub fn find_command_in_path(
        &mut self,
        name: &str,
        path_list: &str,
        path_index: &mut usize,
    ) -> Option<String> {
        let mut search_path_list = path_list.to_string();
        let mut command_name = name.to_string();

        if !self.bash.absolute_program(name) {
            self.absolute_path_given = false;
        } else {
            self.absolute_path_given = true;

            let abs_path = if !name.starts_with('.') && !name.starts_with('/') && !name.starts_with('~')
            {
                format!("./{name}")
            } else {
                name.to_string()
            };

            if let Some(pos) = abs_path.rfind('/') {
                search_path_list = abs_path[..pos].to_string();
                command_name = abs_path[pos + 1..].to_string();
                self.abs_path = Some(search_path_list.clone());
            } else {
                search_path_list.clear();
                command_name = abs_path;
                self.abs_path = Some(search_path_list.clone());
            }
        }

        while *path_index < search_path_list.len() && search_path_list.as_bytes()[*path_index] != 0 {
            let mut path = if self.absolute_path_given {
                *path_index = search_path_list.len();
                search_path_list.clone()
            } else {
                match self
                    .bash
                    .get_next_path_element(Some(&search_path_list), path_index)
                {
                    Some(p) => p,
                    None => break,
                }
            };

            if path.is_empty() {
                path = ".".to_string();
            }

            if path.starts_with('~') {
                path = self
                    .shell
                    .module_tilde(&path)
                    .unwrap_or(path);

                if self.skip_tilde {
                    continue;
                }
            }

            if self.skip_dot && !path.starts_with('/') {
                continue;
            }

            self.found_path_starts_with_dot = path.starts_with('.');

            let full_path =
                self.bash
                    .make_full_pathname(&path, &command_name, command_name.len());
            let status = self.bash.file_status(&full_path);

            if status.exists && status.executable {
                return Some(full_path);
            }
        }

        None
    }

    pub fn func_search(
        &self,
        indent: bool,
        cmd: &str,
        func_list: &[FunctionSt],
        function_start_type: i32,
    ) -> bool {
        for function in func_list {
            if function.name == cmd {
                if indent {
                    print!("\t");
                }
                if function_start_type == 1 {
                    println!("{cmd} () {{");
                } else {
                    println!("{cmd} ()");
                }
                for line in &function.lines {
                    if indent {
                        print!("\t");
                    }
                    print!("{line}");
                }
                return true;
            }
        }
        false
    }

    pub fn path_search(&mut self, indent: bool, cmd: &str, path_list: &str) -> bool {
        let mut found_something = false;

        if !path_list.is_empty() {
            let mut path_index = 0usize;
            loop {
                let mut next = self.show_all;
                let result = self.find_command_in_path(cmd, path_list, &mut path_index);
                let Some(result) = result else {
                    break;
                };

                let cleaned = self.path_clean_up(&result);
                let home = self.home.clone().unwrap_or_default();
                let in_home = (self.show_tilde || self.skip_tilde)
                    && !home.is_empty()
                    && cleaned.starts_with(&home);

                if indent {
                    print!("\t");
                }

                let mut full_path = cleaned.as_str();

                let cwd = self.get_current_working_directory();
                if !(self.skip_tilde && in_home)
                    && self.show_dot
                    && self.found_path_starts_with_dot
                    && full_path.starts_with(&cwd)
                {
                    full_path = &full_path[cwd.len()..];
                    print!("./");
                } else if in_home {
                    if self.skip_tilde {
                        next = true;
                        continue;
                    }
                    if self.show_tilde {
                        full_path = &full_path[home.len()..];
                        print!("~/");
                    }
                }

                println!("{full_path}");
                found_something = true;

                if !next {
                    break;
                }
            }
        }

        found_something
    }

    pub fn process_alias(
        &mut self,
        input: &str,
        argv: &mut [Option<String>],
        path_list: &str,
        function_start_type: i32,
    ) {
        let mut p = input.trim_start();

        if let Some(rest) = p.strip_prefix("alias") {
            p = rest;
        }
        p = p.trim_start();

        let alias_name_len = p
            .chars()
            .take_while(|c| *c != ' ' && *c != '\t' && *c != '=')
            .count();
        let alias_name = &p[..alias_name_len];

        for arg in argv.iter_mut() {
            let Some(current_arg) = arg.clone() else {
                continue;
            };
            if current_arg != alias_name {
                continue;
            }

            print!("{input}");

            if !self.show_all {
                *arg = None;
            }

            let mut rhs = p[alias_name_len..].trim_start();
            if let Some(rest) = rhs.strip_prefix('=') {
                rhs = rest;
            }
            rhs = rhs.trim_start();

            let mut chars = rhs.chars();
            let quote = match chars.next() {
                Some('"') => {
                    rhs = &rhs[1..];
                    Some('"')
                }
                Some('\'') => {
                    rhs = &rhs[1..];
                    Some('\'')
                }
                _ => None,
            };

            loop {
                rhs = rhs.trim_start();
                let len = rhs
                    .chars()
                    .take_while(|c| {
                        !c.is_whitespace()
                            && Some(*c) != quote
                            && *c != '|'
                            && *c != '&'
                    })
                    .count();

                let cmd = rhs[..len].to_string();

                if let Some(existing) = arg.as_ref() {
                    if *existing == cmd {
                        *arg = None;
                    }
                }

                let mut found = false;
                if self.read_functions && !cmd.contains('/') {
                    found = self.func_search(true, &cmd, &self.functions, function_start_type);
                }
                if self.show_all || !found {
                    self.path_search(true, &cmd, path_list);
                }

                let bytes = rhs.as_bytes();
                let mut idx = len;
                while idx < bytes.len() {
                    let ch = bytes[idx] as char;
                    let next = bytes.get(idx + 1).copied().map(char::from);
                    if (ch == '|' && next != Some('|')) || (ch == '&' && next != Some('&')) {
                        break;
                    }
                    idx += 1;
                }

                if idx >= bytes.len() {
                    break;
                }

                idx += 1;
                rhs = &rhs[idx..];
            }

            break;
        }
    }

    pub fn run(&mut self, args: &[String]) -> i32 {
        if let Some(arg0) = args.first() {
            self.program_name = arg0.clone();
        }
        let mut operands: Vec<Option<String>> = Vec::new();
        let mut parse_options = true;

        for arg in args.iter().skip(1) {
            if parse_options && arg == "--" {
                parse_options = false;
                continue;
            }

            if parse_options && arg.starts_with("--") {
                match arg.as_str() {
                    "--help" => {
                        self.print_usage(io::stdout());
                        return 0;
                    }
                    "--version" => {
                        self.print_version();
                        return 0;
                    }
                    "--skip-dot" => self.skip_dot = true,
                    "--skip-tilde" => self.skip_tilde = true,
                    "--show-dot" => self.show_dot = true,
                    "--show-tilde" => self.show_tilde = true,
                    "--tty-only" => self.tty_only = true,
                    "--all" => self.show_all = true,
                    "--read-alias" => self.read_alias = true,
                    "--skip-alias" => self.skip_alias = true,
                    "--read-functions" => self.read_functions = true,
                    "--skip-functions" => self.skip_functions = true,
                    _ => {
                        self.print_unrecognized_option_and_usage(arg);
                        return 1;
                    }
                }
                continue;
            }

            if parse_options && arg.starts_with('-') && arg.len() > 1 {
                let shorts = &arg[1..];
                let mut recognized = true;
                for ch in shorts.chars() {
                    match ch {
                        'a' => self.show_all = true,
                        'i' => self.read_alias = true,
                        'v' | 'V' => {
                            self.print_version();
                            return 0;
                        }
                        _ => {
                            recognized = false;
                            break;
                        }
                    }
                }
                if !recognized {
                    self.print_unrecognized_option_and_usage(arg);
                    return 1;
                }
                continue;
            }

            operands.push(Some(arg.clone()));
        }

        if self.skip_alias {
            self.read_alias = false;
        }
        if self.skip_functions {
            self.read_functions = false;
        }

        if self.show_tilde && self.home.is_none() {
            self.home = self.shell.get_home_dir().or_else(|| self.bash.sh_get_home_dir());
        }

        if operands.is_empty() {
            self.print_usage(io::stderr());
            return 1;
        }

        let path_list = self
            .bash
            .sh_get_env_value("PATH")
            .unwrap_or_default();

        let mut found_any = false;
        let mut missing_any = false;

        for operand in &mut operands {
            let Some(cmd) = operand.clone() else {
                continue;
            };

            let mut found_something = false;

            if self.read_functions {
                found_something = self.func_search(false, &cmd, &self.functions, 1);
            }

            if (self.show_all || !found_something)
                && !self.path_search(false, &cmd, &path_list)
                && !found_something
            {
                let fail_name = if self.absolute_path_given {
                    cmd.rsplit('/').next().unwrap_or(&cmd).to_string()
                } else {
                    cmd.clone()
                };
                let fail_path = if self.absolute_path_given {
                    self.abs_path.clone().unwrap_or_else(|| path_list.clone())
                } else {
                    path_list.clone()
                };
                self.print_fail(&fail_name, &fail_path);
                missing_any = true;
            } else {
                found_any = true;
            }
        }

        if self.read_alias && !self.skip_alias {
            let mut alias_operands = operands.clone();
            let aliases = self.aliases.clone();
            for alias in aliases {
                self.process_alias(&alias, &mut alias_operands, &path_list, 1);
            }
        }

        if missing_any {
            1
        } else if found_any {
            0
        } else {
            1
        }
    }
}
