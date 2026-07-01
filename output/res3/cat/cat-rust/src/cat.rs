#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
use std::fs::File;
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};

pub struct Cat;

#[derive(Clone, Debug)]
struct LineNumberState {
    digits: Vec<u8>,
    print_start: usize,
}

impl Default for LineNumberState {
    fn default() -> Self {
        let mut digits = b"         0\t".to_vec();
        let end = digits.len().saturating_sub(2);
        digits[end] = b'1';
        Self {
            digits,
            print_start: end,
        }
    }
}

impl LineNumberState {
    fn current_prefix(&self) -> &[u8] {
        &self.digits[self.print_start..]
    }

    fn increment_like_coreutils(&mut self) {
        if self.digits.len() < 3 {
            self.digits = b"1\t".to_vec();
            self.print_start = 0;
            return;
        }

        let mut endp = self.digits.len() - 3;
        loop {
            if self.digits[endp] < b'9' {
                self.digits[endp] += 1;
                return;
            }
            self.digits[endp] = b'0';
            if endp == self.print_start {
                break;
            }
            endp -= 1;
        }

        if self.print_start > 0 {
            self.print_start -= 1;
            self.digits[self.print_start] = b'1';
        } else {
            self.digits[0] = b'>';
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct FormatOptions {
    show_nonprinting: bool,
    show_tabs: bool,
    number: bool,
    number_nonblank: bool,
    show_ends: bool,
    squeeze_blank: bool,
}

impl Cat {
    pub fn print_usage_and_exit(status: i32) -> ! {
        let program = "cat";
        if status == 0 {
            let mut out = io::stdout().lock();
            let _ = writeln!(out, "Usage: {program} [OPTION]... [FILE]...");
            let _ = writeln!(out, "Concatenate FILE(s) to standard output.");
            let _ = writeln!(out);
            let _ = writeln!(out, "  -A, --show-all           equivalent to -vET");
            let _ = writeln!(out, "  -b, --number-nonblank    number nonempty output lines, overrides -n");
            let _ = writeln!(out, "  -e                       equivalent to -vE");
            let _ = writeln!(out, "  -E, --show-ends          display $ at end of each line");
            let _ = writeln!(out, "  -n, --number             number all output lines");
            let _ = writeln!(out, "  -s, --squeeze-blank      suppress repeated empty output lines");
            let _ = writeln!(out, "  -t                       equivalent to -vT");
            let _ = writeln!(out, "  -T, --show-tabs          display TAB characters as ^I");
            let _ = writeln!(out, "  -u                       (ignored)");
            let _ = writeln!(out, "  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB");
            let _ = writeln!(out, "      --help     display this help and exit");
            let _ = writeln!(out, "      --version  output version information and exit");
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "Examples:\n  {program} f - g  Output f's contents, then standard input, then g's contents.\n  {program}        Copy standard input to standard output."
            );
        } else {
            let _ = writeln!(
                io::stderr().lock(),
                "Try 'cat --help' for more information."
            );
        }
        std::process::exit(status);
    }

    pub fn next_line_num(state: &mut String) {
        let mut inner = if state.is_empty() {
            LineNumberState::default()
        } else {
            let bytes = state.as_bytes().to_vec();
            let print_start = bytes.iter().position(|&b| b != b' ').unwrap_or(0);
            LineNumberState {
                digits: bytes,
                print_start,
            }
        };
        inner.increment_like_coreutils();
        *state = String::from_utf8(inner.digits).unwrap_or_else(|_| "1\t".to_string());
    }

    pub fn simple_cat<R: Read, W: Write>(
        input: &mut R,
        output: &mut W,
        bufsize: usize,
    ) -> io::Result<bool> {
        let mut buf = vec![0_u8; bufsize.max(1)];
        loop {
            let n = input.read(&mut buf)?;
            if n == 0 {
                return Ok(true);
            }
            output.write_all(&buf[..n])?;
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
        input: &mut R,
        output: &mut W,
        options: CatOptions,
        line_state: &mut LineNumberState,
        line_start: &mut bool,
        blank_run: &mut usize,
        pending_cr: &mut bool,
    ) -> io::Result<bool> {
        let fmt = FormatOptions {
            show_nonprinting: options.show_nonprinting,
            show_tabs: options.show_tabs,
            number: options.number,
            number_nonblank: options.number_nonblank,
            show_ends: options.show_ends,
            squeeze_blank: options.squeeze_blank,
        };

        let mut pending = Vec::with_capacity(8192);
        let mut buf = vec![0u8; 8192];

        loop {
            let n = input.read(&mut buf)?;
            if n == 0 {
                break;
            }

            let data = &buf[..n];
            let mut i = 0usize;

            while i < data.len() {
                let byte = data[i];

                if *line_start {
                    let is_blank_line = byte == b'\n';
                    if is_blank_line {
                        *blank_run += 1;
                    } else {
                        *blank_run = 0;
                    }

                    if fmt.squeeze_blank && is_blank_line && *blank_run >= 2 {
                        i += 1;
                        *line_start = true;
                        continue;
                    }

                    if fmt.number && (!fmt.number_nonblank || !is_blank_line) {
                        pending.extend_from_slice(line_state.current_prefix());
                        line_state.increment_like_coreutils();
                    }

                    *line_start = false;
                }

                match byte {
                    b'\n' => {
                        if fmt.show_ends {
                            if *pending_cr {
                                pending.extend_from_slice(b"^M");
                                *pending_cr = false;
                            }
                            pending.push(b'$');
                        }
                        pending.push(b'\n');
                        *line_start = true;
                    }
                    b'\t' if fmt.show_tabs => {
                        if *pending_cr {
                            pending.push(b'\r');
                            *pending_cr = false;
                        }
                        pending.extend_from_slice(b"^I");
                    }
                    _ => {
                        if !fmt.show_nonprinting {
                            if byte == b'\r' && fmt.show_ends {
                                let next_is_nl = if i + 1 < data.len() {
                                    data[i + 1] == b'\n'
                                } else {
                                    false
                                };
                                if next_is_nl {
                                    pending.extend_from_slice(b"^M");
                                } else if i + 1 == data.len() {
                                    *pending_cr = true;
                                } else {
                                    pending.push(b'\r');
                                }
                            } else {
                                if *pending_cr {
                                    pending.push(b'\r');
                                    *pending_cr = false;
                                }
                                pending.push(byte);
                            }
                        } else {
                            if *pending_cr {
                                pending.push(b'\r');
                                *pending_cr = false;
                            }
                            Self::push_visible_byte(
                                &mut pending,
                                byte,
                                fmt.show_nonprinting,
                                fmt.show_tabs,
                            );
                        }
                    }
                }

                i += 1;
            }

            Self::write_pending(output, &mut pending)?;
            output.flush()?;
        }

        Self::write_pending(output, &mut pending)?;
        Ok(true)
    }

    pub fn copy_cat<R: Read, W: Write>(
        input: &mut R,
        output: &mut W,
        options: CatOptions,
        line_state: &mut LineNumberState,
        line_start: &mut bool,
        blank_run: &mut usize,
        pending_cr: &mut bool,
    ) -> io::Result<bool> {
        if options.number
            || options.number_nonblank
            || options.squeeze_blank
            || options.show_ends
            || options.show_nonprinting
            || options.show_tabs
        {
            Self::cat(input, output, options, line_state, line_start, blank_run, pending_cr)
        } else {
            Self::simple_cat(input, output, 128 * 1024)
        }
    }

    pub fn main(args: &[String]) -> i32 {
        let mut options = CatOptions::default();
        let mut files: Vec<String> = Vec::new();

        let mut iter = args.iter().skip(1).peekable();
        while let Some(arg) = iter.next() {
            if arg == "--" {
                files.extend(iter.cloned());
                break;
            } else if arg == "--help" {
                Self::print_usage_and_exit(0);
            } else if arg == "--version" {
                let _ = writeln!(io::stdout().lock(), "{} 1.0.0", Self::proper_name());
                return 0;
            } else if let Some(flag) = arg.strip_prefix("--") {
                match flag {
                    "number-nonblank" => {
                        options.number = true;
                        options.number_nonblank = true;
                    }
                    "number" => options.number = true,
                    "squeeze-blank" => options.squeeze_blank = true,
                    "show-nonprinting" => options.show_nonprinting = true,
                    "show-ends" => options.show_ends = true,
                    "show-tabs" => options.show_tabs = true,
                    "show-all" => {
                        options.show_nonprinting = true;
                        options.show_ends = true;
                        options.show_tabs = true;
                    }
                    _ => Self::print_usage_and_exit(1),
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                for ch in arg[1..].chars() {
                    match ch {
                        'b' => {
                            options.number = true;
                            options.number_nonblank = true;
                        }
                        'e' => {
                            options.show_ends = true;
                            options.show_nonprinting = true;
                        }
                        'n' => options.number = true,
                        's' => options.squeeze_blank = true,
                        't' => {
                            options.show_tabs = true;
                            options.show_nonprinting = true;
                        }
                        'u' => {}
                        'v' => options.show_nonprinting = true,
                        'A' => {
                            options.show_nonprinting = true;
                            options.show_ends = true;
                            options.show_tabs = true;
                        }
                        'E' => options.show_ends = true,
                        'T' => options.show_tabs = true,
                        '-' => files.push(arg.clone()),
                        _ => Self::print_usage_and_exit(1),
                    }
                }
            } else {
                files.push(arg.clone());
            }
        }

        if files.is_empty() {
            files.push("-".to_string());
        }

        let stdout = io::stdout();
        let mut out = stdout.lock();
        let mut ok = true;
        let mut line_state = LineNumberState::default();
        let mut line_start = true;
        let mut blank_run = 0usize;
        let mut pending_cr = false;
        for file in files {
            if file == "-" {
                let stdin_is_stdout = match Self::stdin_same_as_stdout() {
                    Ok(v) => v,
                    Err(err) => {
                        let _ = writeln!(io::stderr().lock(), "cat: -: {err}");
                        ok = false;
                        continue;
                    }
                };

                if stdin_is_stdout && Self::stdout_would_exhaust_input() {
                    let _ = writeln!(io::stderr().lock(), "cat: -: input file is output file");
                    ok = false;
                    continue;
                }

                let stdin = io::stdin();
                let mut input = stdin.lock();
                match Self::copy_cat(
                    &mut input,
                    &mut out,
                    options.clone(),
                    &mut line_state,
                    &mut line_start,
                    &mut blank_run,
                    &mut pending_cr,
                ) {
                    Ok(result) => {
                        ok &= result;
                    }
                    Err(err) => {
                        let _ = writeln!(io::stderr().lock(), "cat: -: {err}");
                        ok = false;
                    }
                }
            } else {
                let path = PathBuf::from(&file);
                match File::open(&path) {
                    Ok(mut input) => {
                        let same_as_stdout = match Self::same_file_as_stdout(&path) {
                            Ok(v) => v,
                            Err(err) => {
                                let _ = writeln!(io::stderr().lock(), "cat: {}: {err}", path.display());
                                ok = false;
                                continue;
                            }
                        };

                        if same_as_stdout && Self::stdout_would_exhaust_input_file(&input) {
                            let _ = writeln!(
                                io::stderr().lock(),
                                "cat: {}: input file is output file",
                                path.display()
                            );
                            ok = false;
                            continue;
                        }

                        match Self::copy_cat(
                            &mut input,
                            &mut out,
                            options.clone(),
                            &mut line_state,
                            &mut line_start,
                            &mut blank_run,
                            &mut pending_cr,
                        ) {
                            Ok(result) => {
                                ok &= result;
                            }
                            Err(err) => {
                                let _ = writeln!(io::stderr().lock(), "cat: {}: {err}", path.display());
                                ok = false;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = writeln!(io::stderr().lock(), "cat: {}: {err}", path.display());
                        ok = false;
                    }
                }
            }
        }

        if pending_cr {
            ok &= out.write_all(b"\r").is_ok();
        }

        if out.flush().is_err() {
            ok = false;
        }

        if ok { 0 } else { 1 }
    }

    pub fn proper_name() -> &'static str {
        "cat"
    }

    #[cfg(unix)]
    fn same_file_as_stdout(path: &Path) -> io::Result<bool> {
        let in_meta = std::fs::metadata(path)?;
        let out_meta = std::fs::metadata("/proc/self/fd/1")?;
        Ok(in_meta.dev() == out_meta.dev() && in_meta.ino() == out_meta.ino())
    }

    #[cfg(not(unix))]
    fn same_file_as_stdout(_path: &Path) -> io::Result<bool> {
        Ok(false)
    }

    #[cfg(unix)]
    fn stdin_same_as_stdout() -> io::Result<bool> {
        let in_meta = std::fs::metadata("/proc/self/fd/0")?;
        let out_meta = std::fs::metadata("/proc/self/fd/1")?;
        Ok(in_meta.dev() == out_meta.dev() && in_meta.ino() == out_meta.ino())
    }

    #[cfg(unix)]
    fn stdout_would_exhaust_input() -> bool {
        match Self::fd_would_exhaust_input(0) {
            Ok(v) => v,
            Err(_) => false,
        }
    }

    #[cfg(unix)]
    fn stdout_would_exhaust_input_file(input: &File) -> bool {
        match Self::fd_would_exhaust_input(input.as_raw_fd()) {
            Ok(v) => v,
            Err(_) => false,
        }
    }

    #[cfg(unix)]
    fn fd_would_exhaust_input(input_fd: i32) -> io::Result<bool> {
        let out_flags = crate::fcntl::fcntl(1, crate::fcntl::F_GETFL, 0)?;
        if out_flags & crate::fcntl::O_APPEND != 0 {
            return Ok(true);
        }

        let in_pos = crate::fcntl::lseek(input_fd, 0, crate::fcntl::SEEK_CUR)?;
        let out_pos = crate::fcntl::lseek(1, 0, crate::fcntl::SEEK_CUR)?;
        Ok(in_pos < out_pos)
    }

    #[cfg(not(unix))]
    fn stdout_would_exhaust_input_file(_input: &File) -> bool {
        false
    }

    #[cfg(not(unix))]
    fn stdout_would_exhaust_input() -> bool {
        false
    }

    #[cfg(not(unix))]
    fn stdout_would_exhaust_input_file(_input: &File) -> bool {
        false
    }

    fn push_visible_byte(out: &mut Vec<u8>, ch: u8, show_nonprinting: bool, show_tabs: bool) {
        if !show_nonprinting {
            out.push(ch);
            return;
        }

        if ch == b'\t' && !show_tabs {
            out.push(b'\t');
        } else if ch >= 32 {
            if ch < 127 {
                out.push(ch);
            } else if ch == 127 {
                out.extend_from_slice(b"^?");
            } else {
                out.extend_from_slice(b"M-");
                if ch >= 160 {
                    if ch < 255 {
                        out.push(ch - 128);
                    } else {
                        out.extend_from_slice(b"^?");
                    }
                } else {
                    out.push(b'^');
                    out.push(ch - 128 + 64);
                }
            }
        } else {
            out.push(b'^');
            out.push(ch + 64);
        }
    }
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
