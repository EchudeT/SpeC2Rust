use crate::bash::{absolute_program, get_next_path_element, make_full_pathname, sh_get_home_dir, Bash, FileStatus};
use std::env;
use std::fs::Metadata;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct FunctionSt {
    pub name: String,
    pub len: usize,
    pub lines: Vec<String>,
}

pub struct Which {
    progname: String,
    skip_dot: bool,
    skip_tilde: bool,
    show_dot: bool,
    show_tilde: bool,
    tty_only: bool,
    show_all: bool,
    read_alias: bool,
    skip_alias: bool,
    read_functions: bool,
    skip_functions: bool,
    absolute_path_given: bool,
    abs_path: Option<String>,
    found_path_starts_with_dot: bool,
    cwd: Option<String>,
    home: Option<String>,
    aliases: Vec<String>,
    functions: Vec<FunctionSt>,
}

impl Default for Which {
    fn default() -> Self {
        Self {
            progname: String::from("which"),
            skip_dot: false,
            skip_tilde: false,
            show_dot: false,
            show_tilde: false,
            tty_only: false,
            show_all: false,
            read_alias: false,
            skip_alias: false,
            read_functions: false,
            skip_functions: false,
            absolute_path_given: false,
            abs_path: None,
            found_path_starts_with_dot: false,
            cwd: None,
            home: None,
            aliases: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl Which {
    pub fn function_st(name: String, lines: Vec<String>) -> FunctionSt {
        FunctionSt {
            len: name.len(),
            name,
            lines,
        }
    }

    pub fn alloc_buffer(size: usize) -> Vec<u8> {
        vec![0_u8; size]
    }

    pub fn resize_buffer(mut buffer: Vec<u8>, size: usize) -> Vec<u8> {
        buffer.resize(size, 0_u8);
        buffer
    }

    pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
        std::fs::symlink_metadata(path)
    }

    pub fn var_os<K: AsRef<std::ffi::OsStr>>(key: K) -> Option<std::ffi::OsString> {
        env::var_os(key)
    }

    pub fn print_usage(&self, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "Usage: {} [options] [--] COMMAND [...]", self.progname)?;
        writeln!(out, "Write the full path of COMMAND(s) to standard output.\n")?;
        writeln!(out, "  --version, -[vV] Print version and exit successfully.")?;
        writeln!(out, "  --help,          Print this help and exit successfully.")?;
        writeln!(out, "  --skip-dot       Skip directories in PATH that start with a dot.")?;
        writeln!(out, "  --skip-tilde     Skip directories in PATH that start with a tilde.")?;
        writeln!(out, "  --show-dot       Don't expand a dot to current directory in output.")?;
        writeln!(out, "  --show-tilde     Output a tilde for HOME directory for non-root.")?;
        writeln!(out, "  --tty-only       Stop processing options on the right if not on tty.")?;
        writeln!(out, "  --all, -a        Print all matches in PATH, not just the first")?;
        writeln!(out, "  --read-alias, -i Read list of aliases from stdin.")?;
        writeln!(out, "  --skip-alias     Ignore option --read-alias; don't read stdin.")?;
        writeln!(out, "  --read-functions Read shell functions from stdin.")?;
        writeln!(out, "  --skip-functions Ignore option --read-functions; don't read stdin.\n")?;
        writeln!(
            out,
            "Recommended use is to write the output of (alias; declare -f) to standard"
        )?;
        writeln!(
            out,
            "input, so that which can show aliases and shell functions. See which(1) for"
        )?;
        writeln!(out, "examples.\n")?;
        writeln!(
            out,
            "If the options --read-alias and/or --read-functions are specified then the"
        )?;
        writeln!(
            out,
            "output can be a full alias or function definition, optionally followed by"
        )?;
        writeln!(out, "the full path of each command used inside of those.\n")?;
        writeln!(out, "Report bugs to <which-bugs@gnu.org>.")?;
        Ok(())
    }

    pub fn print_version(&self) -> io::Result<()> {
        let mut out = io::stdout().lock();
        writeln!(
            out,
            "GNU which v2.21, Copyright (C) 1999 - 2015 Carlo Wood."
        )?;
        writeln!(out, "GNU which comes with ABSOLUTELY NO WARRANTY;")?;
        writeln!(
            out,
            "This program is free software; your freedom to use, change"
        )?;
        writeln!(
            out,
            "and distribute this program is protected by the GPL."
        )?;
        Ok(())
    }

    pub fn print_fail(&self, name: &str, path_list: &str) -> io::Result<()> {
        let mut err = io::stderr().lock();
        writeln!(err, "{}: no {} in ({})", self.progname, name, path_list)
    }

    pub fn get_current_working_directory(&mut self) -> io::Result<String> {
        if let Some(cwd) = &self.cwd {
            return Ok(cwd.clone());
        }

        let mut cwd = env::current_dir()
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .or_else(|| env::var("PWD").ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Can't get current working directory"))?;

        if !cwd.starts_with('/') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Can't get current working directory",
            ));
        }

        if !cwd.ends_with('/') {
            cwd.push('/');
        }

        self.cwd = Some(cwd.clone());
        Ok(cwd)
    }

    pub fn path_clean_up(&mut self, path: &str) -> String {
        let mut input = if path.starts_with('/') {
            path.to_string()
        } else {
            match self.get_current_working_directory() {
                Ok(cwd) => format!("{cwd}{path}"),
                Err(_) => path.to_string(),
            }
        };

        let preserve_double_slash = input.starts_with("//") && !input.starts_with("///");
        let absolute = input.starts_with('/');

        let mut parts: Vec<String> = Vec::new();
        for part in input.split('/') {
            if part.is_empty() || part == "." {
                continue;
            }
            if part == ".." {
                if parts.pop().is_none() {
                    return path.to_string();
                }
            } else {
                parts.push(part.to_string());
            }
        }

        input.clear();
        if absolute {
            if preserve_double_slash {
                input.push_str("//");
            } else {
                input.push('/');
            }
        }
        input.push_str(&parts.join("/"));
        if input.is_empty() {
            if absolute {
                "/".to_string()
            } else {
                ".".to_string()
            }
        } else {
            input
        }
    }

    fn expand_tilde(path: &str) -> String {
        if path == "~" || path.starts_with("~/") {
            if let Some(home) = env::var("HOME").ok().or_else(sh_get_home_dir) {
                if path == "~" {
                    return home;
                }
                return format!("{home}/{}", &path[2..]);
            }
        }
        path.to_string()
    }

    fn is_executable_candidate(path: &str) -> bool {
        matches!(
            Bash::file_status(path),
            FileStatus::Executable | FileStatus::ReadableExecutable
        )
    }

    pub fn find_command_in_path(
        &mut self,
        name: &str,
        path_list: &str,
        path_index: &mut usize,
    ) -> Option<String> {
        let mut effective_path_list = path_list.to_string();
        let mut effective_name = name.to_string();

        if !absolute_program(name) {
            self.absolute_path_given = false;
            self.abs_path = None;
        } else {
            self.absolute_path_given = true;
            let abs_path = if !name.starts_with('.') && !name.starts_with('/') && !name.starts_with('~') {
                format!("./{name}")
            } else {
                name.to_string()
            };

            if let Some((dir, file)) = abs_path.rsplit_once('/') {
                self.abs_path = Some(dir.to_string());
                effective_path_list = dir.to_string();
                effective_name = file.to_string();
            } else {
                self.abs_path = Some(abs_path.clone());
                effective_path_list.clear();
                effective_name = abs_path;
            }
        }

        while !effective_path_list.is_empty() && *path_index < effective_path_list.len() {
            let mut path = if self.absolute_path_given {
                *path_index = effective_path_list.len();
                effective_path_list.clone()
            } else {
                match get_next_path_element(&effective_path_list, path_index) {
                    Some(p) => p,
                    None => break,
                }
            };

            if path.starts_with('~') {
                path = Self::expand_tilde(&path);
                if self.skip_tilde {
                    continue;
                }
            }

            if self.skip_dot && !path.starts_with('/') {
                continue;
            }

            self.found_path_starts_with_dot = path.starts_with('.');

            let full_path = make_full_pathname(&path, &effective_name);

            if Self::is_executable_candidate(&full_path) {
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
        let mut out = io::stdout().lock();
        for function in func_list {
            if function.name == cmd {
                if indent {
                    let _ = write!(out, "\t");
                }
                if function_start_type == 1 {
                    let _ = writeln!(out, "{} () {{", cmd);
                } else {
                    let _ = writeln!(out, "{} ()", cmd);
                }
                for line in &function.lines {
                    if indent {
                        let _ = write!(out, "\t");
                    }
                    let _ = write!(out, "{line}");
                }
                return true;
            }
        }
        false
    }

    pub fn path_search(&mut self, indent: bool, cmd: &str, path_list: &str) -> bool {
        let mut found_something = false;

        if path_list.is_empty() {
            return false;
        }

        let mut path_index = 0usize;
        loop {
            let mut next = self.show_all;
            let result = self.find_command_in_path(cmd, path_list, &mut path_index);
            let Some(result) = result else {
                break;
            };

            let cleaned = self.path_clean_up(&result);
            let home = self.home.clone().unwrap_or_default();
            let homelen = home.len();
            let in_home = (self.show_tilde || self.skip_tilde)
                && !home.is_empty()
                && cleaned.starts_with(&home);

            {
                let mut out = io::stdout().lock();
                if indent {
                    let _ = write!(out, "\t");
                }

                if !(self.skip_tilde && in_home)
                    && self.show_dot
                    && self.found_path_starts_with_dot
                {
                    if let Ok(cwd) = self.get_current_working_directory() {
                        if cleaned.starts_with(&cwd) {
                            let suffix = &cleaned[cwd.len()..];
                            let _ = writeln!(out, "./{suffix}");
                            found_something = true;
                            if !next {
                                break;
                            }
                            continue;
                        }
                    }
                } else if in_home {
                    if self.skip_tilde {
                        next = true;
                        if !next {
                            break;
                        }
                        continue;
                    }
                    if self.show_tilde {
                        let suffix = &cleaned[homelen..];
                        let _ = writeln!(out, "~/{suffix}");
                        found_something = true;
                        if !next {
                            break;
                        }
                        continue;
                    }
                }

                let _ = writeln!(out, "{cleaned}");
            }

            found_something = true;
            if !next {
                break;
            }
        }

        found_something
    }

    pub fn process_alias(
        &mut self,
        text: &str,
        argv: &mut [Option<String>],
        path_list: &str,
        function_start_type: i32,
    ) {
        let mut p = text.trim_start();

        if let Some(rest) = p.strip_prefix("alias") {
            p = rest.trim_start();
        }

        let name_len = p
            .chars()
            .take_while(|c| *c != ' ' && *c != '\t' && *c != '=')
            .count();
        if name_len == 0 || p.len() < name_len {
            return;
        }
        let alias_name = &p[..name_len];

        let matching_index = argv.iter().position(|arg| arg.as_deref() == Some(alias_name));
        let Some(index) = matching_index else {
            return;
        };

        {
            let mut out = io::stdout().lock();
            let _ = write!(out, "{text}");
        }

        if !self.show_all {
            argv[index] = None;
        }

        p = &p[name_len..];
        p = p.trim_start();
        if let Some(rest) = p.strip_prefix('=') {
            p = rest.trim_start();
        }

        let mut quote = '\0';
        if let Some(first) = p.chars().next() {
            if first == '"' || first == '\'' {
                quote = first;
                p = &p[first.len_utf8()..];
            }
        }

        loop {
            p = p.trim_start();
            if p.is_empty() {
                break;
            }

            let end = p
                .char_indices()
                .find(|(_, ch)| {
                    *ch == ' '
                        || *ch == '\t'
                        || (quote != '\0' && *ch == quote)
                        || *ch == '|'
                        || *ch == '&'
                })
                .map(|(idx, _)| idx)
                .unwrap_or(p.len());

            let cmd = p[..end].to_string();
            if cmd.is_empty() {
                break;
            }

            if argv[index].as_deref() == Some(cmd.as_str()) {
                argv[index] = None;
            }

            let mut found = false;
            if self.read_functions && !cmd.contains('/') {
                found = self.func_search(true, &cmd, &self.functions, function_start_type);
            }
            if self.show_all || !found {
                self.path_search(true, &cmd, path_list);
            }

            p = &p[end..];
            while p.chars().next().is_some() {
                let mut iter = p.chars();
                let first = iter.next().unwrap_or_default();
                let second = iter.next();
                if first != '|' && first != '&' {
                    p = &p[first.len_utf8()..];
                    continue;
                }
                if second == Some(first) {
                    p = &p[first.len_utf8()..];
                    continue;
                }
                break;
            }

            if p.is_empty() {
                break;
            }

            if let Some(ch) = p.chars().next() {
                p = &p[ch.len_utf8()..];
            } else {
                break;
            }
        }
    }

    pub fn run(&mut self, args: &[String]) -> i32 {
        if let Some(arg0) = args.first() {
            self.progname = arg0.clone();
        }

        let path_list = env::var("PATH").unwrap_or_default();

        let mut positionals: Vec<String> = Vec::new();
        let mut iter = args.iter().skip(1).peekable();

        while let Some(arg) = iter.next() {
            if arg == "--" {
                positionals.extend(iter.cloned());
                break;
            } else if arg == "--help" {
                let mut out = io::stdout().lock();
                let _ = self.print_usage(&mut out);
                return 0;
            } else if arg == "--version" || arg == "-v" || arg == "-V" {
                let _ = self.print_version();
                return 0;
            } else if arg == "--skip-dot" {
                self.skip_dot = !self.tty_only;
            } else if arg == "--skip-tilde" {
                self.skip_tilde = !self.tty_only;
            } else if arg == "--show-dot" {
                self.show_dot = !self.tty_only;
            } else if arg == "--show-tilde" {
                self.show_tilde = !self.tty_only;
            } else if arg == "--tty-only" {
                self.tty_only = false;
            } else if arg == "--all" || arg == "-a" {
                self.show_all = true;
            } else if arg == "--read-alias" || arg == "-i" {
                self.read_alias = true;
            } else if arg == "--skip-alias" {
                self.skip_alias = true;
            } else if arg == "--read-functions" {
                self.read_functions = true;
            } else if arg == "--skip-functions" {
                self.skip_functions = true;
            } else if arg.starts_with("--") {
                let mut err = io::stderr().lock();
                let _ = writeln!(err, "{}: unrecognized option '{}'", self.progname, arg);
                let _ = self.print_usage(&mut err);
                return 1;
            } else if arg.starts_with('-') && arg.len() > 1 {
                for ch in arg[1..].chars() {
                    match ch {
                        'a' => self.show_all = true,
                        'i' => self.read_alias = true,
                        'v' | 'V' => {
                            let _ = self.print_version();
                            return 0;
                        }
                        _ => {
                            let mut err = io::stderr().lock();
                            let _ = writeln!(err, "{}: unrecognized option '-{}'", self.progname, ch);
                            let _ = self.print_usage(&mut err);
                            return 1;
                        }
                    }
                }
            } else {
                positionals.push(arg.clone());
            }
        }

        let _ = Bash::update_user_ids();

        if self.show_dot {
            let _ = self.get_current_working_directory();
        }

        if self.show_tilde || self.skip_tilde {
            if let Some(mut home) = env::var("HOME").ok().or_else(sh_get_home_dir) {
                if !home.ends_with('/') {
                    home.push('/');
                }
                self.home = Some(home);
            }
        }

        if self.skip_alias {
            self.read_alias = false;
        }
        if self.skip_functions {
            self.read_functions = false;
        }

        if positionals.is_empty() {
            let mut err = io::stderr().lock();
            let _ = self.print_usage(&mut err);
            return -1;
        }

        let mut function_start_type = 0;

        if self.read_alias || self.read_functions {
            let stdin = io::stdin();
            let mut lines = stdin.lock().lines();
            let mut processing_aliases = self.read_alias;

            while let Some(Ok(buf)) = lines.next() {
                let trimmed = buf.trim_end();
                let mut looks_like_function_start = false;
                let mut function_start_has_declare = false;

                if self.read_functions {
                    if trimmed.ends_with(" ()") {
                        looks_like_function_start = true;
                        function_start_has_declare = buf.starts_with("declare -");
                    }
                    if trimmed.ends_with(" () {") {
                        looks_like_function_start = true;
                        function_start_type = 1;
                        function_start_has_declare = false;
                    }
                }

                if processing_aliases && !looks_like_function_start {
                    if buf.starts_with("declare -") {
                        continue;
                    }
                    self.aliases.push(format!("{buf}\n"));
                } else if self.read_functions && looks_like_function_start {
                    processing_aliases = false;

                    let mut header = buf.clone();
                    if function_start_has_declare {
                        header = header[9..].to_string();
                        if let Some(pos) = header.find(' ') {
                            header = header[pos + 1..].to_string();
                        }
                    }

                    let name = header
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .to_string();

                    let mut body = Vec::new();
                    while let Some(Ok(line)) = lines.next() {
                        let stored = format!("{line}\n");
                        let is_end = line == "}";
                        body.push(stored);
                        if is_end {
                            break;
                        }
                    }

                    self.functions.push(Self::function_st(name, body));
                }
            }

            if self.read_alias {
                let mut argv_opts: Vec<Option<String>> =
                    positionals.iter().cloned().map(Some).collect();
                let aliases = self.aliases.clone();
                for alias in aliases {
                    self.process_alias(&alias, &mut argv_opts, &path_list, function_start_type);
                }
                positionals = argv_opts.into_iter().flatten().collect();
            }
        }

        let mut fail_count = 0;

        for arg in &positionals {
            let mut found_something = false;

            if self.read_functions && !arg.contains('/') {
                found_something = self.func_search(false, arg, &self.functions, function_start_type);
            }

            if (self.show_all || !found_something)
                && !self.path_search(false, arg, &path_list)
                && !found_something
            {
                let failure_name = if self.absolute_path_given {
                    arg.rsplit('/').next().unwrap_or(arg).to_string()
                } else {
                    arg.clone()
                };
                let failure_path = if self.absolute_path_given {
                    self.abs_path.clone().unwrap_or_else(|| path_list.clone())
                } else {
                    path_list.clone()
                };
                let _ = self.print_fail(&failure_name, &failure_path);
                fail_count += 1;
            }
        }

        fail_count
    }
}
