use std::cmp::{max, min};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, IsTerminal, Read, Write};
use std::os::fd::AsRawFd;
use std::process::{Command, Stdio};
const VERSION: &str = env!("CARGO_PKG_VERSION");
const YANKCMD: &str = "xsel";

const T_ENTER_CA_MODE: &str = "\u{1b}[?1049h";
const T_EXIT_CA_MODE: &str = "\u{1b}[?1049l";
const T_CURSOR_INVISIBLE: &str = "\u{1b}[?25l";
const T_CURSOR_VISIBLE: &str = "\u{1b}[?25h";
const T_SAVE_CURSOR: &str = "\u{1b}7";
const T_RESTORE_CURSOR: &str = "\u{1b}8";
const T_CLR_EOS: &str = "\u{1b}[J";
const T_ENTER_STANDOUT_MODE: &str = "\u{1b}[7m";
const T_EXIT_STANDOUT_MODE: &str = "\u{1b}[0m";

const KEY_ENTER: i32 = 1;
const KEY_HOME: i32 = 2;
const KEY_END: i32 = 3;
const KEY_TERM: i32 = 4;
const KEY_UP: i32 = 5;
const KEY_RIGHT: i32 = 6;
const KEY_DOWN: i32 = 7;
const KEY_LEFT: i32 = 8;

#[derive(Clone, Debug)]
struct Field {
    lo: usize,
    so: usize,
    eo: usize,
}

#[derive(Debug)]
struct TerminalState {
    reader: Option<File>,
    writer: Option<File>,
    alternate_screen: bool,
    saved_mode: bool,
    orig_stty: Option<String>,
}

impl TerminalState {
    fn new() -> Self {
        Self {
            reader: None,
            writer: None,
            alternate_screen: false,
            saved_mode: false,
            orig_stty: None,
        }
    }
}

pub struct Yank {
    input_buffer: Vec<u8>,
    fields: Vec<Field>,
    output_len: usize,
    tty: TerminalState,
    yank_command: Vec<String>,
}

impl Default for Yank {
    fn default() -> Self {
        Self {
            input_buffer: Vec::new(),
            fields: Vec::new(),
            output_len: 0,
            tty: TerminalState::new(),
            yank_command: vec![YANKCMD.to_string()],
        }
    }
}

#[derive(Clone, Copy)]
struct WinSize {
    rows: usize,
    cols: usize,
}

impl Yank {
    fn tty_winsize(&self) -> WinSize {
        let cols = env::var("COLUMNS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&v| v > 0)
            .unwrap_or(80);
        let rows = env::var("LINES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&v| v > 0)
            .unwrap_or(24);
        WinSize { rows, cols }
    }

    fn capture_stty_state(fd: &File) -> Option<String> {
        Command::new("stty")
            .arg("-g")
            .stdin(Stdio::from(fd.try_clone().ok()?))
            .output()
            .ok()
            .and_then(|out| {
                if out.status.success() {
                    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
                } else {
                    None
                }
            })
    }

    fn set_rawish_mode(fd: &File) {
        let _ = Command::new("stty")
            .args(["-echo", "-icanon", "-isig", "icrnl", "min", "1", "time", "0"])
            .stdin(Stdio::from(fd.try_clone().unwrap_or_else(|_| fd.try_clone().expect("tty clone"))))
            .status();
    }

    fn restore_stty_state(fd: &File, state: &str) {
        let _ = Command::new("stty")
            .arg(state)
            .stdin(Stdio::from(fd.try_clone().unwrap_or_else(|_| fd.try_clone().expect("tty clone"))))
            .status();
    }

}
impl Yank {
    pub fn input(&mut self) -> io::Result<()> {
        let mut stdin = io::stdin().lock();
        self.input_buffer.clear();
        stdin.read_to_end(&mut self.input_buffer)?;
        self.input_buffer.push(0);
        Ok(())
    }


    fn byte_matches_pattern_class(b: u8, pattern: &str) -> bool {
        if !pattern.starts_with("[^") || !pattern.ends_with("]+") {
            return b != 0 && b != b'\n' && b != b'\r';
        }

        let inner = &pattern[2..pattern.len() - 2];
        let mut chars = inner.chars().peekable();
        while let Some(ch) = chars.next() {
            let matched = if ch == '\\' {
                match chars.next() {
                    Some('n') => b == b'\n',
                    Some('r') => b == b'\r',
                    Some('t') => b == b'\t',
                    Some('f') => b == 0x0c,
                    Some(other) => b == other as u8,
                    None => b == b'\\',
                }
            } else {
                b == ch as u8
            };

            if matched {
                return false;
            }
        }

        b != 0
    }
    pub fn delimiters_to_pattern(delimiters: &str) -> Result<String, String> {
        let mut escaped = String::new();
        for ch in delimiters.chars() {
            match ch {
                '\\' | ']' | '^' | '-' => {
                    escaped.push('\\');
                    escaped.push(ch);
                }
                _ => escaped.push(ch),
            }
        }
        Ok(format!("[^{}\u{000c}\n\r\t]+", escaped))
    }

    pub fn compare_fields(f1: &Field, f2: &Field) -> i32 {
        let s1 = f1.so.saturating_sub(f1.lo);
        let e1 = f1.eo.saturating_sub(f1.lo);
        let s2 = f2.so.saturating_sub(f2.lo);
        let e2 = f2.eo.saturating_sub(f2.lo);

        if max(s1, s2) <= min(e1, e2) {
            0
        } else if e1 < s2 {
            1
        } else {
            -1
        }
    }

    pub fn xwrite<W: Write>(writer: &mut W, mut bytes: &[u8]) -> io::Result<()> {
        while !bytes.is_empty() {
            let written = writer.write(bytes)?;
            if written == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write complete buffer",
                ));
            }
            bytes = &bytes[written..];
        }
        Ok(())
    }

    pub fn yank(&self, bytes: &[u8]) -> io::Result<i32> {
        if !io::stdout().is_terminal() {
            let mut stdout = io::stdout().lock();
            Self::xwrite(&mut stdout, bytes)?;
            stdout.flush()?;
            return Ok(0);
        }

        let mut child = Command::new(
            self.yank_command
                .first()
                .map(String::as_str)
                .unwrap_or(YANKCMD),
        )
        .args(self.yank_command.iter().skip(1))
        .stdin(Stdio::piped())
        .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            Self::xwrite(&mut stdin, bytes)?;
        }

        let status = child.wait()?;
        if let Some(code) = status.code() {
            Ok(code)
        } else {
            Ok(1)
        }
    }

    pub fn twrite(&mut self, bytes: &[u8]) -> io::Result<()> {
        let writer = self
            .tty
            .writer
            .as_mut()
            .ok_or_else(|| io::Error::other("terminal writer not initialized"))?;
        Self::xwrite(writer, bytes)?;
        writer.flush()
    }

    pub fn tputs(&mut self, text: &str) -> io::Result<()> {
        self.twrite(text.as_bytes())
    }

    pub fn tsetup(&mut self, pattern: &str, _ignore_case: bool) -> io::Result<()> {
        self.tty.reader = Some(File::open("/dev/tty")?);
        self.tty.writer = Some(OpenOptions::new().write(true).open("/dev/tty")?);
        self.fields.clear();

        let ws = self.tty_winsize();
        let visible_len = self.input_buffer.len().saturating_sub(1);
        let mut m = min(ws.cols.saturating_mul(ws.rows), visible_len);
        let mut n = m;
        let mut i = 0usize;

        while m > 0 && i < visible_len {
            if !Self::byte_matches_pattern_class(self.input_buffer[i], pattern) {
                i += 1;
                continue;
            }

            let start = i;
            i += 1;
            while i < visible_len && Self::byte_matches_pattern_class(self.input_buffer[i], pattern) {
                i += 1;
            }
            let rm_eo = i - start;
            if rm_eo == 0 {
                continue;
            }

            let eo = start + max(min(rm_eo, m), 1) - 1;
            self.fields.push(Field { lo: 0, so: start, eo });
            m = m.saturating_sub(min(rm_eo, m));
        }

        let mut row = 0usize;
        let mut j = 0usize;
        let mut s = 0usize;
        let mut e = 0usize;
        while n > 0 && row < ws.rows {
            if s == e {
                e = self.input_buffer[s.saturating_add(1)..visible_len]
                    .iter()
                    .position(|&c| c == b'\n')
                    .map(|p| s + 1 + p)
                    .unwrap_or(visible_len);
            }

            let w = min(e.saturating_sub(s), ws.cols);
            while j < self.fields.len() && self.fields[j].so < s + w {
                self.fields[j].lo = s;
                j += 1;
            }
            s = s.saturating_add(w);
            n = n.saturating_sub(w);
            row += 1;
        }

        self.fields.truncate(j);
        if n > 0 && !self.fields.is_empty() {
            let last = self.fields.len() - 1;
            if self.fields[last].eo.saturating_sub(self.fields[last].lo) >= ws.cols {
                self.fields[last].eo = self.fields[last].lo + ws.cols - 1;
            }
        }
        self.output_len = max(s.saturating_sub(1), 0);

        if let Some(reader) = self.tty.reader.as_ref() {
            self.tty.orig_stty = Self::capture_stty_state(reader);
            Self::set_rawish_mode(reader);
        }

        if self.tty.alternate_screen {
            self.tputs(T_ENTER_CA_MODE)?;
        }
        self.tputs(T_CURSOR_INVISIBLE)?;
        for _ in 0..row {
            self.tputs("\n")?;
        }
        for _ in 0..row {
            self.tputs("\x1bM")?;
        }
        self.tputs(T_SAVE_CURSOR)?;
        self.tty.saved_mode = true;
        Ok(())
    }

    pub fn tend(&mut self) -> io::Result<()> {
        if self.tty.saved_mode {
            self.tputs(T_RESTORE_CURSOR)?;
            self.tputs(T_CLR_EOS)?;
            self.tputs(T_CURSOR_VISIBLE)?;
            if self.tty.alternate_screen {
                self.tputs(T_EXIT_CA_MODE)?;
            }
        }
        if let (Some(reader), Some(state)) = (self.tty.reader.as_ref(), self.tty.orig_stty.as_deref()) {
            Self::restore_stty_state(reader, state);
        }
        self.tty.orig_stty = None;
        self.tty.reader = None;
        self.tty.writer = None;
        self.tty.saved_mode = false;
        Ok(())
    }

    pub fn tgetc(&mut self) -> io::Result<i32> {
        let reader = self
            .tty
            .reader
            .as_mut()
            .ok_or_else(|| io::Error::other("terminal reader not initialized"))?;

        let mut buf = [0u8; 3];
        let n = reader.read(&mut buf)?;
        if n == 0 {
            return Ok(KEY_TERM);
        }
        let s = &buf[..n];

        let keys: [(&[u8], i32); 18] = [
            (b"\n", KEY_ENTER),
            (b"\r", KEY_ENTER),
            (b"\x01", KEY_HOME),
            (b"\x03", KEY_TERM),
            (b"\x04", KEY_TERM),
            (b"\x05", KEY_END),
            (b"\x0e", KEY_RIGHT),
            (b"\x10", KEY_LEFT),
            (b"G", KEY_END),
            (b"g", KEY_HOME),
            (b"h", KEY_LEFT),
            (b"j", KEY_DOWN),
            (b"k", KEY_UP),
            (b"l", KEY_RIGHT),
            (b"\x1b[A", KEY_UP),
            (b"\x1b[C", KEY_RIGHT),
            (b"\x1b[B", KEY_DOWN),
            (b"\x1b[D", KEY_LEFT),
        ];

        for (pat, key) in keys {
            if s.starts_with(pat) {
                return Ok(key);
            }
        }

        Ok(0)
    }

    pub fn tmain(&mut self) -> io::Result<Option<usize>> {
        let mut i: isize = 0;
        let mut j: isize = 0;
        let n = self.output_len;

        loop {
            self.tputs(T_RESTORE_CURSOR)?;
            if !self.fields.is_empty() {
                let current = self.fields[i as usize].clone();
                let before = self.input_buffer[..current.so].to_vec();
                let selected = self.input_buffer[current.so..=current.eo].to_vec();
                let after_len = n.saturating_sub(current.eo);
                let after = if after_len > 0 && current.eo + 1 <= self.input_buffer.len() {
                    Some(self.input_buffer[current.eo + 1..min(current.eo + 1 + after_len, self.input_buffer.len())].to_vec())
                } else {
                    None
                };

                self.twrite(&before)?;
                self.tputs(T_ENTER_STANDOUT_MODE)?;
                self.twrite(&selected)?;
                self.tputs(T_EXIT_STANDOUT_MODE)?;
                if let Some(after) = after.as_deref() {
                    self.twrite(after)?;
                }
            } else {
                let all = self.input_buffer[..n].to_vec();
                self.twrite(&all)?;
            }

            let c = self.tgetc()?;
            match c {
                KEY_ENTER => {
                    if !self.fields.is_empty() {
                        return Ok(Some(i as usize));
                    }
                }
                KEY_TERM => return Ok(None),
                KEY_HOME => j = 0,
                KEY_RIGHT => j = i + 1,
                KEY_END => j = self.fields.len() as isize - 1,
                KEY_LEFT => j = i - 1,
                KEY_DOWN | KEY_UP => {
                    if self.fields.is_empty() {
                        continue;
                    }

                    if c == KEY_DOWN {
                        j = i;
                        while j < self.fields.len() as isize
                            && self.fields[i as usize].lo == self.fields[j as usize].lo
                        {
                            j += 1;
                        }
                        if j == self.fields.len() as isize {
                            continue;
                        }
                    } else {
                        let mut k = i;
                        while k > 0 && self.fields[i as usize].lo == self.fields[k as usize].lo {
                            k -= 1;
                        }
                        j = k;
                        while j > 0
                            && self.fields[(j - 1) as usize].lo == self.fields[k as usize].lo
                        {
                            j -= 1;
                        }
                    }

                    while j >= 0
                        && (j as usize) + 1 < self.fields.len()
                        && Self::compare_fields(&self.fields[i as usize], &self.fields[j as usize]) < 0
                        && self.fields[j as usize].lo == self.fields[(j + 1) as usize].lo
                    {
                        j += 1;
                    }
                }
                _ => {}
            }

            if j >= 0 && j < self.fields.len() as isize {
                i = j;
            }
        }
    }

    pub fn usage_text() -> String {
        "usage: yank [-1ilxv] [-d delim] [-g pattern] [-- command [args]]\n".to_string()
    }

    pub fn main(args: &[String]) -> i32 {
        match Self::main_root(args) {
            Ok(code) => code,
            Err(err) => {
                let _ = writeln!(io::stderr().lock(), "{err}");
                1
            }
        }
    }

    pub fn main_root(args: &[String]) -> Result<i32, String> {
        let mut app = Self::default();
        let mut one = false;
        let mut ignore_case = false;
        let mut pattern = Self::delimiters_to_pattern(" ")?;
        let mut command_args_start = None;

        let mut i = 1usize;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--" {
                command_args_start = Some(i + 1);
                break;
            }
            if !arg.starts_with('-') || arg == "-" {
                command_args_start = Some(i);
                break;
            }

            match arg.as_str() {
                "-1" => one = true,
                "-i" => ignore_case = true,
                "-l" => pattern = Self::delimiters_to_pattern("")?,
                "-v" => {
                    println!("yank {VERSION}");
                    return Ok(0);
                }
                "-x" => app.tty.alternate_screen = true,
                "-d" => {
                    i += 1;
                    let value = args.get(i).ok_or_else(Self::usage_text)?;
                    pattern = Self::delimiters_to_pattern(value)?;
                }
                "-g" => {
                    i += 1;
                    let value = args.get(i).ok_or_else(Self::usage_text)?;
                    pattern = value.clone();
                }
                _ => return Err(Self::usage_text()),
            }
            i += 1;
        }

        if let Some(start) = command_args_start {
            app.yank_command = if start < args.len() {
                args[start..].to_vec()
            } else {
                vec![YANKCMD.to_string()]
            };
        }

        if app.yank_command.is_empty() {
            app.yank_command.push(YANKCMD.to_string());
        }

        app.input().map_err(|e| e.to_string())?;
        app.tsetup(&pattern, ignore_case).map_err(|e| e.to_string())?;

        let selected = if one && app.fields.len() == 1 {
            Some(0)
        } else {
            app.tmain().map_err(|e| {
                let _ = app.tend();
                e.to_string()
            })?
        };

        app.tend().map_err(|e| e.to_string())?;

        let Some(index) = selected else {
            return Ok(1);
        };

        let field = &app.fields[index];
        let code = app
            .yank(&app.input_buffer[field.so..=field.eo])
            .map_err(|e| e.to_string())?;
        Ok(code)
    }
}
