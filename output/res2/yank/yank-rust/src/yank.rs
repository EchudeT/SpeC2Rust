use std::cmp::{max, min};
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

const VERSION: &str = "0.0.0";
const DEFAULT_YANK_COMMAND: &str = "xsel";
const T_ENTER_CA_MODE: &str = "\u{1b}[?1049h";
const T_EXIT_CA_MODE: &str = "\u{1b}[?1049l";
const T_SAVE_CURSOR: &str = "\u{1b}7";
const T_RESTORE_CURSOR: &str = "\u{1b}8";
const T_CURSOR_VISIBLE: &str = "\u{1b}[?25h";
const T_CURSOR_INVISIBLE: &str = "\u{1b}[?25l";
const T_ENTER_STANDOUT_MODE: &str = "\u{1b}[7m";
const T_EXIT_STANDOUT_MODE: &str = "\u{1b}[27m";
const T_CLR_EOS: &str = "\u{1b}[J";

#[derive(Clone, Debug, Default)]
struct Field {
    lo: usize,
    so: usize,
    eo: usize,
}

#[derive(Debug)]
struct TerminalState {
    alt_screen: bool,
    rows: usize,
    cols: usize,
    tty_in: std::fs::File,
    tty_out: std::fs::File,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Key {
    Enter,
    Terminate,
    Home,
    End,
    Left,
    Right,
    Up,
    Down,
    Other,
}

pub struct Yank {
    input_data: Vec<u8>,
    fields: Vec<Field>,
    output_limit: usize,
    terminal: Option<TerminalState>,
    yank_argv: Vec<String>,
    one: bool,
    ignore_case: bool,
    use_alt_screen: bool,
    regex_newline: bool,
    pattern: String,
}

impl Default for Yank {
    fn default() -> Self {
        Self {
            input_data: Vec::new(),
            fields: Vec::new(),
            output_limit: 0,
            terminal: None,
            yank_argv: vec![DEFAULT_YANK_COMMAND.to_string()],
            one: false,
            ignore_case: false,
            use_alt_screen: false,
            regex_newline: false,
            pattern: Self::string_to_pattern(" "),
        }
    }
}

impl Yank {
    pub fn input(&mut self) -> io::Result<()> {
        self.input_data.clear();
        io::stdin().read_to_end(&mut self.input_data)?;
        Ok(())
    }

    pub fn string_to_pattern(s: &str) -> String {
        format!("[^{}\u{000C}\n\r\t]+", s)
    }

    pub fn field_cmp(f1: &Field, f2: &Field) -> i32 {
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

    pub fn xwrite<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<usize> {
        writer.write_all(bytes)?;
        Ok(bytes.len())
    }

    pub fn yank(&self, bytes: &[u8]) -> io::Result<()> {
        if !Self::stdout_is_tty() {
            let mut stdout = io::stdout().lock();
            Self::xwrite(&mut stdout, bytes)?;
            stdout.flush()?;
            return Ok(());
        }

        let program = self
            .yank_argv
            .first()
            .cloned()
            .unwrap_or_else(|| DEFAULT_YANK_COMMAND.to_string());
        let args = if self.yank_argv.len() > 1 {
            self.yank_argv[1..].to_vec()
        } else {
            Vec::new()
        };

        let mut child = Command::new(&program)
            .args(args)
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            Self::xwrite(&mut stdin, bytes)?;
        }

        let status = child.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("child process exited with status {status}"),
            ))
        }
    }

    pub fn twrite(&mut self, bytes: &[u8]) -> io::Result<()> {
        let tty = self
            .terminal
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "terminal not initialized"))?;
        Self::xwrite(&mut tty.tty_out, bytes)?;
        tty.tty_out.flush()
    }

    pub fn tputs(&mut self, s: &str) -> io::Result<()> {
        self.twrite(s.as_bytes())
    }

    pub fn tsetup(&mut self) -> io::Result<()> {
        let tty_in = OpenOptions::new().read(true).open("/dev/tty")?;
        let tty_out = OpenOptions::new().write(true).open("/dev/tty")?;

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

        self.fields.clear();

        if self.pattern == Self::string_to_pattern("") {
            let mut start = 0usize;
            let len = self.input_data.len();
            while start < len {
                let mut end = start;
                while end < len && self.input_data[end] != b'\n' {
                    end += 1;
                }
                if end > start {
                    self.fields.push(Field {
                        lo: start,
                        so: start,
                        eo: end.saturating_sub(1),
                    });
                }
                if end >= len {
                    break;
                }
                start = end + 1;
            }
            self.output_limit = self.input_data.len();
        } else {
            let total_visible = min(cols.saturating_mul(rows), self.input_data.len());
            let delim_bytes = Self::pattern_delimiters(&self.pattern);
            let is_delim = |b: u8| match &delim_bytes {
                Some(bytes) => bytes.contains(&b),
                None => matches!(b, b' ' | b'\n' | b'\r' | b'\t' | 0x0c),
            };
            let mut cursor = 0usize;

            while cursor < total_visible {
                while cursor < total_visible && is_delim(self.input_data[cursor]) {
                    cursor += 1;
                }
                if cursor >= total_visible {
                    break;
                }

                let start = cursor;
                while cursor < total_visible && !is_delim(self.input_data[cursor]) {
                    cursor += 1;
                }
                let end = cursor.saturating_sub(1);
                self.fields.push(Field {
                    lo: 0,
                    so: start,
                    eo: end,
                });
            }

            let mut s = 0usize;
            let mut e = 0usize;
            let mut n = total_visible;
            let mut i = 0usize;
            let mut j = 0usize;

            while n > 0 && i < rows {
                if s == e {
                    let search_start = min(s.saturating_add(1), self.input_data.len());
                    let search_end = min(search_start.saturating_add(n), self.input_data.len());
                    e = self.input_data[search_start..search_end]
                        .iter()
                        .position(|&b| b == b'\n')
                        .map(|p| search_start + p)
                        .unwrap_or(self.input_data.len());
                }

                let w = min(e.saturating_sub(s), cols);
                while j < self.fields.len() && self.fields[j].so < s + w {
                    self.fields[j].lo = s;
                    j += 1;
                }
                s = s.saturating_add(w);
                n = n.saturating_sub(w);
                i += 1;
            }

            self.fields.truncate(j);
            if n > 0 && !self.fields.is_empty() {
                let last = self.fields.len() - 1;
                if self.fields[last].eo.saturating_sub(self.fields[last].lo) >= cols {
                    self.fields[last].eo = self.fields[last].lo + cols - 1;
                }
            }
            self.output_limit = s.saturating_sub(1);
        }

        let screen_rows = self
            .fields
            .iter()
            .map(|f| f.lo)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            .max(1);

        self.terminal = Some(TerminalState {
            alt_screen: self.use_alt_screen,
            rows,
            cols,
            tty_in,
            tty_out,
        });

        if self.use_alt_screen {
            self.tputs(T_ENTER_CA_MODE)?;
        }
        self.tputs(T_CURSOR_INVISIBLE)?;
        for _ in 0..screen_rows {
            self.tputs("\n")?;
        }
        for _ in 0..screen_rows {
            self.tputs("\u{1b}M")?;
        }
        self.tputs(T_SAVE_CURSOR)
    }

    pub fn tend(&mut self) -> io::Result<()> {
        if self.terminal.is_none() {
            return Ok(());
        }
        self.tputs(T_RESTORE_CURSOR)?;
        self.tputs(T_CLR_EOS)?;
        self.tputs(T_CURSOR_VISIBLE)?;
        if self
            .terminal
            .as_ref()
            .map(|t| t.alt_screen)
            .unwrap_or(false)
        {
            self.tputs(T_EXIT_CA_MODE)?;
        }
        self.terminal = None;
        Ok(())
    }

    pub fn tgetc(&mut self) -> io::Result<Key> {
        let tty = self
            .terminal
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "terminal not initialized"))?;

        let mut first = [0u8; 1];
        let n = tty.tty_in.read(&mut first)?;
        if n == 0 {
            return Ok(Key::Terminate);
        }

        let key = match first[0] {
            b'\n' | b'\r' => Key::Enter,
            0x01 => Key::Home,
            0x03 | 0x04 => Key::Terminate,
            0x05 | b'G' => Key::End,
            0x0e | b'l' => Key::Right,
            0x10 | b'h' => Key::Left,
            b'j' => Key::Down,
            b'k' => Key::Up,
            b'g' => Key::Home,
            0x1b => {
                let mut seq = [0u8; 2];
                let n2 = tty.tty_in.read(&mut seq)?;
                match &seq[..n2] {
                    b"[A" => Key::Up,
                    b"[B" => Key::Down,
                    b"[C" => Key::Right,
                    b"[D" => Key::Left,
                    _ => Key::Other,
                }
            }
            _ => Key::Other,
        };

        Ok(key)
    }

    pub fn tmain(&mut self) -> io::Result<Option<usize>> {
        let mut i: isize = 0;
        let mut j: isize = 0;
        let n = self.output_limit;

        loop {
            self.tputs(T_RESTORE_CURSOR)?;
            if !self.fields.is_empty() {
                let field = &self.fields[i as usize];
                let so = field.so;
                let eo = field.eo;
                let head = self.input_data[..so].to_vec();
                let selected = self.input_data[so..=eo].to_vec();
                self.twrite(&head)?;
                self.tputs(T_ENTER_STANDOUT_MODE)?;
                self.twrite(&selected)?;
                self.tputs(T_EXIT_STANDOUT_MODE)?;
                let tail_start = eo.saturating_add(1);
                let tail_end = min(n, self.input_data.len());
                if tail_start < tail_end {
                    let tail = self.input_data[tail_start..tail_end].to_vec();
                    self.twrite(&tail)?;
                }
            } else {
                let end = min(n, self.input_data.len());
                let visible = self.input_data[..end].to_vec();
                self.twrite(&visible)?;
            }

            match self.tgetc()? {
                Key::Enter => {
                    if !self.fields.is_empty() {
                        return Ok(Some(i as usize));
                    }
                }
                Key::Terminate => return Ok(None),
                Key::Home => j = 0,
                Key::Right => j = i + 1,
                Key::End => j = self.fields.len() as isize - 1,
                Key::Left => j = i - 1,
                Key::Down | Key::Up => {
                    if self.fields.is_empty() {
                        continue;
                    }
                    if self.tgetc_key_is_down_last() {
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
                        while j > 0 && self.fields[(j - 1) as usize].lo == self.fields[k as usize].lo
                        {
                            j -= 1;
                        }
                    }
                    while j + 1 < self.fields.len() as isize
                        && Self::field_cmp(&self.fields[i as usize], &self.fields[j as usize]) < 0
                        && self.fields[j as usize].lo == self.fields[(j + 1) as usize].lo
                    {
                        j += 1;
                    }
                }
                Key::Other => {}
            }

            if j >= 0 && (j as usize) < self.fields.len() {
                i = j;
            }
        }
    }

    fn tgetc_key_is_down_last(&self) -> bool {
        false
    }

    fn pattern_delimiters(pattern: &str) -> Option<Vec<u8>> {
        let prefix = "[^";
        let suffix = "\u{000C}\n\r\t]+";
        if let Some(rest) = pattern.strip_prefix(prefix) {
            if let Some(delims) = rest.strip_suffix(suffix) {
                return Some(delims.as_bytes().to_vec());
            }
        }
        None
    }

    pub fn usage_text() -> String {
        "usage: yank [-1ilxv] [-d delim] [-g pattern] [-- command [args]]\n".to_string()
    }

    pub fn main(&mut self, args: &[String]) -> i32 {
        match self.main_root(args) {
            Ok(code) => code,
            Err(err) => {
                let _ = writeln!(io::stderr().lock(), "{err}");
                1
            }
        }
    }

    pub fn main_root(&mut self, args: &[String]) -> io::Result<i32> {
        self.one = false;
        self.ignore_case = false;
        self.use_alt_screen = false;
        self.regex_newline = false;
        self.pattern = Self::string_to_pattern(" ");
        self.yank_argv = vec![DEFAULT_YANK_COMMAND.to_string()];

        let mut iter = args.iter().skip(1).peekable();
        let mut command_args: Vec<String> = Vec::new();

        while let Some(arg) = iter.next() {
            if !command_args.is_empty() {
                command_args.push(arg.clone());
                continue;
            }

            if arg == "--" {
                command_args.extend(iter.cloned());
                break;
            }

            match arg.as_str() {
                "-1" => self.one = true,
                "-i" => self.ignore_case = true,
                "-l" => self.pattern = Self::string_to_pattern(""),
                "-x" => self.use_alt_screen = true,
                "-v" => {
                    let mut stdout = io::stdout().lock();
                    writeln!(stdout, "yank {VERSION}")?;
                    return Ok(0);
                }
                "-d" => {
                    let Some(next) = iter.next() else {
                        let _ = write!(io::stderr().lock(), "{}", Self::usage_text());
                        return Ok(2);
                    };
                    self.pattern = Self::string_to_pattern(next);
                }
                "-g" => {
                    let Some(next) = iter.next() else {
                        let _ = write!(io::stderr().lock(), "{}", Self::usage_text());
                        return Ok(2);
                    };
                    self.pattern = next.clone();
                    self.regex_newline = true;
                }
                _ if arg.starts_with('-') => {
                    let _ = write!(io::stderr().lock(), "{}", Self::usage_text());
                    return Ok(2);
                }
                _ => {
                    command_args.push(arg.clone());
                    command_args.extend(iter.cloned());
                    break;
                }
            }
        }

        if !command_args.is_empty() {
            self.yank_argv = command_args;
        }

        self.input()?;
        self.tsetup()?;

        let selected = if self.one && self.fields.len() == 1 {
            Some(0usize)
        } else {
            self.tmain()?
        };

        self.tend()?;

        let Some(index) = selected else {
            return Ok(1);
        };

        if let Some(field) = self.fields.get(index) {
            let end = min(field.eo.saturating_add(1), self.input_data.len());
            if field.so <= end {
                self.yank(&self.input_data[field.so..end])?;
            }
        }

        Ok(0)
    }

    fn stdout_is_tty() -> bool {
        std::env::var_os("TERM").is_some()
    }
}
