use std::fs::{self, File};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub struct Cat {
    line_number: Vec<u8>,
    line_num_start: usize,
    line_num_print: usize,
    line_num_end: usize,
    newlines2: i32,
    pending_cr: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CatOptions {
    pub show_nonprinting: bool,
    pub show_tabs: bool,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
}

impl Default for Cat {
    fn default() -> Self {
        let mut line_number = b"     0\t".to_vec();
        let line_num_end = 5;
        line_number[line_num_end] = b'0';
        Self {
            line_number,
            line_num_start: 5,
            line_num_print: 0,
            line_num_end,
            newlines2: 0,
            pending_cr: false,
        }
    }
}

impl Cat {
    pub fn usage_text(success: bool) -> String {
        if !success {
            return "Try 'cat --help' for more information.\n".to_string();
        }

        concat!(
            "Usage: cat [OPTION]... [FILE]...\n",
            "Concatenate FILE(s) to standard output.\n",
            "\n",
            "  -A, --show-all           equivalent to -vET\n",
            "  -b, --number-nonblank    number nonempty output lines, overrides -n\n",
            "  -e                       equivalent to -vE\n",
            "  -E, --show-ends          display $ at end of each line\n",
            "  -n, --number             number all output lines\n",
            "  -s, --squeeze-blank      suppress repeated empty output lines\n",
            "  -t                       equivalent to -vT\n",
            "  -T, --show-tabs          display TAB characters as ^I\n",
            "  -u                       (ignored)\n",
            "  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n",
            "      --help     display this help and exit\n",
            "      --version  output version information and exit\n",
            "\n",
            "Examples:\n",
            "  cat f - g  Output f's contents, then standard input, then g's contents.\n",
            "  cat        Copy standard input to standard output.\n",
        )
        .to_string()
    }

    pub fn proper_name() -> &'static str {
        "cat"
    }

    pub fn show_nonprinting() -> CatOptions {
        CatOptions {
            show_nonprinting: true,
            ..CatOptions::default()
        }
    }

    pub fn show_tabs() -> CatOptions {
        CatOptions {
            show_tabs: true,
            ..CatOptions::default()
        }
    }

    pub fn number_nonblank() -> CatOptions {
        CatOptions {
            number: true,
            number_nonblank: true,
            ..CatOptions::default()
        }
    }

    pub fn show_ends() -> CatOptions {
        CatOptions {
            show_ends: true,
            ..CatOptions::default()
        }
    }

    pub fn squeeze_blank() -> CatOptions {
        CatOptions {
            squeeze_blank: true,
            ..CatOptions::default()
        }
    }

    pub fn next_line_num(&mut self) {
        let mut endp = self.line_num_end;
        loop {
            if self.line_number[endp] < b'9' {
                self.line_number[endp] += 1;
                return;
            }
            self.line_number[endp] = b'0';
            if endp == self.line_num_start {
                break;
            }
            endp -= 1;
        }

        if self.line_num_start > 0 {
            self.line_num_start -= 1;
            self.line_number[self.line_num_start] = b'1';
        } else {
            self.line_number[0] = b'>';
        }

        if self.line_num_start < self.line_num_print {
            self.line_num_print -= 1;
        }
    }

    fn line_num_slice(&self) -> &[u8] {
        &self.line_number[self.line_num_print..]
    }

    pub fn simple_cat<R: Read, W: Write>(
        input: &mut R,
        output: &mut W,
        buffer_size: usize,
    ) -> io::Result<()> {
        let mut buf = vec![0_u8; buffer_size.max(1)];
        loop {
            let n_read = input.read(&mut buf)?;
            if n_read == 0 {
                return Ok(());
            }
            output.write_all(&buf[..n_read])?;
        }
    }

    pub fn write_pending<W: Write>(output: &mut W, pending: &mut Vec<u8>) -> io::Result<()> {
        if !pending.is_empty() {
            output.write_all(pending)?;
            pending.clear();
        }
        Ok(())
    }

    pub fn cat<R: Read, W: Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
        options: CatOptions,
    ) -> io::Result<()> {
        let mut inbuf = [0_u8; 8192];
        let mut pending = Vec::with_capacity(8192 * 4 + 32);
        let mut at_line_start = self.newlines2 == 0;
        let mut consecutive_blank_lines = if self.newlines2 > 0 {
            self.newlines2 as usize
        } else {
            0
        };

        loop {
            let n_read = input.read(&mut inbuf)?;
            if n_read == 0 {
                break;
            }

            for i in 0..n_read {
                let b = inbuf[i];
                let next_is_nl = i + 1 < n_read && inbuf[i + 1] == b'\n';

                if at_line_start {
                    let is_blank_line = b == b'\n';

                    if is_blank_line {
                        consecutive_blank_lines = consecutive_blank_lines.saturating_add(1);
                        if options.squeeze_blank && consecutive_blank_lines > 1 {
                            at_line_start = true;
                            continue;
                        }
                    } else {
                        consecutive_blank_lines = 0;
                    }

                    if options.number && (!options.number_nonblank || !is_blank_line) {
                        self.next_line_num();
                        pending.extend_from_slice(self.line_num_slice());
                    }

                    at_line_start = false;
                }

                if options.show_nonprinting {
                    match b {
                        b'\t' if !options.show_tabs => pending.push(b'\t'),
                        b'\n' => {
                            if options.show_ends {
                                pending.push(b'$');
                            }
                            pending.push(b'\n');
                            at_line_start = true;
                        }
                        0..=31 => {
                            pending.push(b'^');
                            pending.push(b + 64);
                        }
                        127 => pending.extend_from_slice(b"^?"),
                        128..=255 => {
                            pending.extend_from_slice(b"M-");
                            let c = b - 128;
                            if c < 32 {
                                pending.push(b'^');
                                pending.push(c + 64);
                            } else if c == 127 {
                                pending.extend_from_slice(b"^?");
                            } else {
                                pending.push(c);
                            }
                        }
                        _ => pending.push(b),
                    }
                    continue;
                }

                match b {
                    b'\n' => {
                        if options.show_ends {
                            if self.pending_cr {
                                pending.extend_from_slice(b"^M");
                                self.pending_cr = false;
                            }
                            pending.push(b'$');
                        }
                        pending.push(b'\n');
                        at_line_start = true;
                    }
                    b'\t' if options.show_tabs => {
                        if self.pending_cr {
                            pending.push(b'\r');
                            self.pending_cr = false;
                        }
                        pending.extend_from_slice(b"^I");
                    }
                    b'\r' if options.show_ends => {
                        if self.pending_cr {
                            pending.push(b'\r');
                        }
                        if next_is_nl {
                            pending.extend_from_slice(b"^M");
                        } else if i + 1 == n_read {
                            self.pending_cr = true;
                        } else {
                            pending.push(b'\r');
                            self.pending_cr = false;
                        }
                    }
                    _ => {
                        if self.pending_cr {
                            pending.push(b'\r');
                            self.pending_cr = false;
                        }
                        pending.push(b);
                    }
                }
            }

            Self::write_pending(output, &mut pending)?;
            output.flush()?;
        }

        Self::write_pending(output, &mut pending)?;
        self.newlines2 = consecutive_blank_lines.min(2) as i32;
        Ok(())
    }

    pub fn copy_cat<R: Read, W: Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
        options: CatOptions,
    ) -> io::Result<()> {
        if options.number
            || options.number_nonblank
            || options.show_ends
            || options.show_nonprinting
            || options.show_tabs
            || options.squeeze_blank
        {
            self.cat(input, output, options)
        } else {
            Self::simple_cat(input, output, 8192)
        }
    }

    fn fd_target_path(fd: i32) -> Option<PathBuf> {
        fs::canonicalize(format!("/proc/self/fd/{fd}")).ok()
    }

    fn fd_pos(fd: i32) -> Option<u64> {
        let fdinfo = fs::read_to_string(format!("/proc/self/fdinfo/{fd}")).ok()?;
        for line in fdinfo.lines() {
            if let Some(rest) = line.strip_prefix("pos:\t") {
                if let Ok(v) = rest.trim().parse::<u64>() {
                    return Some(v);
                }
            }
        }
        None
    }

    fn fd_is_append(fd: i32) -> bool {
        let fdinfo = match fs::read_to_string(format!("/proc/self/fdinfo/{fd}")) {
            Ok(s) => s,
            Err(_) => return false,
        };
        for line in fdinfo.lines() {
            if let Some(rest) = line.strip_prefix("flags:\t") {
                if let Ok(flags) = u32::from_str_radix(rest.trim(), 8) {
                    return (flags & 0o2000) != 0;
                }
            }
        }
        false
    }

    fn same_underlying_file(file: &File) -> Option<bool> {
        let stdout_path = Self::fd_target_path(1)?;
        let out_meta = fs::metadata(stdout_path).ok()?;
        let in_meta = file.metadata().ok()?;
        Some(out_meta.dev() == in_meta.dev() && out_meta.ino() == in_meta.ino())
    }

    fn stdout_position() -> Option<u64> {
        Self::fd_pos(1)
    }

    fn file_exhausts_output(file: &mut File) -> bool {
        if !matches!(Self::same_underlying_file(file), Some(true)) {
            return false;
        }

        if Self::fd_is_append(1) {
            return true;
        }

        let in_pos = match file.stream_position() {
            Ok(pos) => pos,
            Err(_) => return false,
        };
        let out_pos = match Self::stdout_position() {
            Some(pos) => pos,
            None => return false,
        };
        in_pos < out_pos
    }

    fn stdin_exhausts_output() -> bool {
        let stdin_path = match Self::fd_target_path(0) {
            Some(p) => p,
            None => return false,
        };
        let stdout_path = match Self::fd_target_path(1) {
            Some(p) => p,
            None => return false,
        };
        let in_meta = match fs::metadata(stdin_path) {
            Ok(meta) => meta,
            Err(_) => return false,
        };
        let out_meta = match fs::metadata(stdout_path) {
            Ok(meta) => meta,
            Err(_) => return false,
        };
        if in_meta.dev() != out_meta.dev() || in_meta.ino() != out_meta.ino() {
            return false;
        }

        if Self::fd_is_append(1) {
            return true;
        }

        let in_pos = match Self::fd_pos(0) {
            Some(pos) => pos,
            None => return false,
        };
        let out_pos = match Self::stdout_position() {
            Some(pos) => pos,
            None => return false,
        };
        in_pos < out_pos
    }

    fn emit_error(path: &str, err: &str) {
        let _ = writeln!(io::stderr(), "cat: {}: {}", path, err);
    }

    pub fn main(args: &[String]) -> ExitCode {
        let mut options = CatOptions::default();
        let mut files = Vec::new();

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "--help" => {
                    let _ = io::stdout().write_all(Self::usage_text(true).as_bytes());
                    return ExitCode::SUCCESS;
                }
                "--version" => {
                    let _ = writeln!(io::stdout(), "{}", Self::proper_name());
                    return ExitCode::SUCCESS;
                }
                "-A" | "--show-all" => {
                    options.show_nonprinting = true;
                    options.show_ends = true;
                    options.show_tabs = true;
                }
                "-b" | "--number-nonblank" => {
                    options.number = true;
                    options.number_nonblank = true;
                }
                "-e" => {
                    options.show_ends = true;
                    options.show_nonprinting = true;
                }
                "-E" | "--show-ends" => options.show_ends = true,
                "-n" | "--number" => options.number = true,
                "-s" | "--squeeze-blank" => options.squeeze_blank = true,
                "-t" => {
                    options.show_tabs = true;
                    options.show_nonprinting = true;
                }
                "-T" | "--show-tabs" => options.show_tabs = true,
                "-u" => {}
                "-v" | "--show-nonprinting" => options.show_nonprinting = true,
                "-" => files.push(arg.clone()),
                _ if arg.starts_with('-') => {
                    let _ = io::stderr().write_all(Self::usage_text(false).as_bytes());
                    return ExitCode::from(1);
                }
                _ => files.push(arg.clone()),
            }
        }

        if files.is_empty() {
            files.push("-".to_string());
        }

        let mut cat = Self::default();
        let stdout = io::stdout();
        let mut output = stdout.lock();
        let mut ok = true;

        for file_name in files {
            if file_name == "-" {
                if Self::stdin_exhausts_output() {
                    Self::emit_error("-", "input file is output file");
                    ok = false;
                    continue;
                }
                let stdin = io::stdin();
                let mut input = stdin.lock();
                if cat.copy_cat(&mut input, &mut output, options).is_err() {
                    ok = false;
                }
            } else {
                match File::open(Path::new(&file_name)) {
                    Ok(mut file) => {
                        if Self::file_exhausts_output(&mut file) {
                            Self::emit_error(&file_name, "input file is output file");
                            ok = false;
                            continue;
                        }
                        let _ = file.seek(SeekFrom::Start(0));
                        let mut input = BufReader::new(file);
                        if cat.copy_cat(&mut input, &mut output, options).is_err() {
                            Self::emit_error(&file_name, "I/O error");
                            ok = false;
                        }
                    }
                    Err(err) => {
                        Self::emit_error(&file_name, &err.to_string());
                        ok = false;
                    }
                }
            }
        }

        if cat.pending_cr && output.write_all(b"\r").is_err() {
            ok = false;
        }

        if output.flush().is_err() {
            ok = false;
        }

        if ok {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }
}
