use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, IsTerminal, Read, Write};
use std::process::{self, Command, Stdio};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_YANK_CMD: &str = "xsel";
const T_ENTER_STANDOUT_MODE: &str = "\x1b[7m";
const T_EXIT_STANDOUT_MODE: &str = "\x1b[0m";
const T_CURSOR_INVISIBLE: &str = "\x1b[?25l";
const T_CURSOR_VISIBLE: &str = "\x1b[?25h";
const T_SAVE_CURSOR: &str = "\x1b7";
const T_RESTORE_CURSOR: &str = "\x1b8";
const T_CLR_EOS: &str = "\x1b[J";
const T_ENTER_CA_MODE: &str = "\x1b[?1049h";
const T_EXIT_CA_MODE: &str = "\x1b[?1049l";

#[derive(Clone, Debug)]
struct Field {
    lo: usize,
    so: usize,
    eo: usize,
}

#[derive(Clone, Debug, Default)]
struct InputBuffer {
    data: Vec<u8>,
}

#[derive(Debug)]
struct TerminalState {
    reader: BufReader<File>,
    writer: File,
    alternate_screen: bool,
    active: bool,
}

#[derive(Clone, Debug)]
enum Key {
    Enter,
    Home,
    Terminate,
    End,
    Right,
    Left,
    Down,
    Up,
    Other,
}

#[derive(Clone, Debug)]
struct Config {
    one_if_single: bool,
    ignore_case: bool,
    alternate_screen: bool,
    pattern: String,
    regex_like_pattern: bool,
    yank_argv: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            one_if_single: false,
            ignore_case: false,
            alternate_screen: false,
            pattern: Yank::string_to_pattern(" "),
            regex_like_pattern: false,
            yank_argv: vec![DEFAULT_YANK_CMD.to_string()],
        }
    }
}

pub struct Yank;

impl Yank {
    pub fn read_input() -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        io::stdin().read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn string_to_pattern(s: &str) -> String {
        format!("[^{}\u{000c}\n\r\t]+", s)
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

    pub fn xwrite<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<()> {
        writer.write_all(bytes)
    }

    pub fn yank_selection(s: &[u8], command: &[String]) -> io::Result<()> {
        if command.is_empty() {
            let mut stdout = io::stdout().lock();
            Self::xwrite(&mut stdout, s)?;
            process::exit(0);
        }

        let program = command
            .first()
            .cloned()
            .unwrap_or_else(|| DEFAULT_YANK_CMD.to_string());
        let mut child = Command::new(program)
            .args(command.iter().skip(1))
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            Self::xwrite(&mut stdin, s)?;
        }

        let status = child.wait()?;
        if let Some(code) = status.code() {
            process::exit(code);
        }
        process::exit(1);
    }

    pub fn terminal_write(state: &mut TerminalState, s: &[u8]) -> io::Result<()> {
        Self::xwrite(&mut state.writer, s)
    }

    pub fn terminal_puts(state: &mut TerminalState, s: &str) -> io::Result<()> {
        Self::terminal_write(state, s.as_bytes())
    }

    pub fn terminal_setup(
        input: &InputBuffer,
        pattern: &str,
        ignore_case: bool,
        alternate_screen: bool,
    ) -> io::Result<(TerminalState, Vec<Field>, usize)> {
        let reader_file = File::open("/dev/tty")?;
        let writer_file = File::options().write(true).open("/dev/tty")?;

        let mut state = TerminalState {
            reader: BufReader::new(reader_file),
            writer: writer_file,
            alternate_screen,
            active: true,
        };

        let width = env::var("COLUMNS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(80);
        let height = env::var("LINES")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(24);

        let limit = min(width.saturating_mul(height), input.data.len());
        let mut fields = Self::find_fields(&input.data, pattern, ignore_case, limit);

        let mut n = limit;
        let mut s = 0usize;
        let mut e = 0usize;
        let mut i = 0usize;
        let mut j = 0usize;

        while n > 0 && i < height {
            if s == e {
                let search_end = min(s + n, input.data.len());
                if let Some(rel) = input.data[s.saturating_add(1)..search_end]
                    .iter()
                    .position(|b| *b == b'\n')
                {
                    e = s + 1 + rel;
                } else {
                    e = input.data.len();
                }
            }

            let w = min(e.saturating_sub(s), width);
            while j < fields.len() && fields[j].so < s + w {
                fields[j].lo = s;
                j += 1;
            }
            s = s.saturating_add(w);
            n = n.saturating_sub(w);
            i += 1;
        }

        fields.truncate(j);
        if n > 0 && !fields.is_empty() {
            let last = fields.len() - 1;
            if fields[last].eo.saturating_sub(fields[last].lo) >= width {
                fields[last].eo = fields[last].lo + width - 1;
            }
        }
        let output_len = s.saturating_sub(1);

        if state.alternate_screen {
            Self::terminal_puts(&mut state, T_ENTER_CA_MODE)?;
        }
        Self::terminal_puts(&mut state, T_CURSOR_INVISIBLE)?;
        for _ in 0..i {
            Self::terminal_puts(&mut state, "\n")?;
        }
        for _ in 0..i {
            Self::terminal_puts(&mut state, "\x1bM")?;
        }
        Self::terminal_puts(&mut state, T_SAVE_CURSOR)?;

        Ok((state, fields, output_len))
    }

    pub fn terminal_end(state: &mut TerminalState) -> io::Result<()> {
        if !state.active {
            return Ok(());
        }

        Self::terminal_puts(state, T_RESTORE_CURSOR)?;
        Self::terminal_puts(state, T_CLR_EOS)?;
        Self::terminal_puts(state, T_CURSOR_VISIBLE)?;
        if state.alternate_screen {
            Self::terminal_puts(state, T_EXIT_CA_MODE)?;
        }
        state.active = false;
        Ok(())
    }

    pub fn terminal_getc(state: &mut TerminalState) -> io::Result<Key> {
        let mut first = [0u8; 1];
        let n = state.reader.read(&mut first)?;
        if n == 0 {
            return Ok(Key::Terminate);
        }

        let key = match first[0] {
            b'\n' => Key::Enter,
            0x01 | b'g' => Key::Home,
            0x03 | 0x04 => Key::Terminate,
            0x05 | b'G' => Key::End,
            0x0e | b'l' => Key::Right,
            0x10 | b'h' => Key::Left,
            b'j' => Key::Down,
            b'k' => Key::Up,
            0x1b => {
                let mut rest = [0u8; 2];
                let m = state.reader.read(&mut rest)?;
                match &rest[..m] {
                    [b'[', b'A'] => Key::Up,
                    [b'[', b'B'] => Key::Down,
                    [b'[', b'C'] => Key::Right,
                    [b'[', b'D'] => Key::Left,
                    _ => Key::Other,
                }
            }
            _ => Key::Other,
        };

        Ok(key)
    }

    pub fn terminal_main(
        state: &mut TerminalState,
        input: &InputBuffer,
        fields: &[Field],
        output_len: usize,
    ) -> io::Result<Option<usize>> {
        let mut i: isize = 0;
        let mut j: isize = 0;
        let field_count = fields.len() as isize;

        loop {
            Self::terminal_puts(state, T_RESTORE_CURSOR)?;
            if !fields.is_empty() {
                let current = &fields[i as usize];
                Self::terminal_write(state, &input.data[..current.so])?;
                Self::terminal_puts(state, T_ENTER_STANDOUT_MODE)?;
                Self::terminal_write(state, &input.data[current.so..=current.eo])?;
                Self::terminal_puts(state, T_EXIT_STANDOUT_MODE)?;
                let tail_start = current.eo + 1;
                let tail_end = output_len.min(input.data.len());
                if tail_start <= tail_end {
                    Self::terminal_write(state, &input.data[tail_start..tail_end])?;
                }
            } else {
                Self::terminal_write(state, &input.data[..output_len.min(input.data.len())])?;
            }

            let key = Self::terminal_getc(state)?;
            match key {
                Key::Enter => {
                    if !fields.is_empty() {
                        return Ok(Some(i as usize));
                    }
                }
                Key::Terminate => return Ok(None),
                Key::Home => j = 0,
                Key::Right => j = i + 1,
                Key::End => j = field_count - 1,
                Key::Left => j = i - 1,
                Key::Down | Key::Up => {
                    if fields.is_empty() {
                        continue;
                    }
                    if matches!(key, Key::Down) {
                        j = i;
                        while j < field_count && fields[i as usize].lo == fields[j as usize].lo {
                            j += 1;
                        }
                        if j == field_count {
                            continue;
                        }
                    } else {
                        let mut k = i;
                        while k > 0 && fields[i as usize].lo == fields[k as usize].lo {
                            k -= 1;
                        }
                        j = k;
                        while j > 0 && fields[(j - 1) as usize].lo == fields[k as usize].lo {
                            j -= 1;
                        }
                    }
                    while (j as usize) + 1 < fields.len()
                        && Self::field_cmp(&fields[i as usize], &fields[j as usize]) < 0
                        && fields[j as usize].lo == fields[(j + 1) as usize].lo
                    {
                        j += 1;
                    }
                }
                Key::Other => {}
            }

            if j >= 0 && j < field_count {
                i = j;
            }
        }
    }

    pub fn print_usage() -> ! {
        eprintln!("usage: yank [-1ilxv] [-d delim] [-g pattern] [-- command [args]]");
        process::exit(2);
    }

    pub fn main() -> i32 {
        match Self::main_root() {
            Ok(code) => code,
            Err(err) => {
                eprintln!("yank: {err}");
                1
            }
        }
    }

    pub fn main_root() -> io::Result<i32> {
        let mut config = Config::default();
        let mut args = env::args().skip(1).peekable();
        let mut command_args = Vec::new();

        while let Some(arg) = args.next() {
            if arg == "--" {
                command_args.extend(args);
                break;
            } else if arg == "-1" {
                config.one_if_single = true;
            } else if arg == "-i" {
                config.ignore_case = true;
            } else if arg == "-l" {
                config.pattern = Self::string_to_pattern("");
                config.regex_like_pattern = false;
            } else if arg == "-x" {
                config.alternate_screen = true;
            } else if arg == "-v" {
                println!("yank {VERSION}");
                return Ok(0);
            } else if arg == "-d" {
                let value = args.next().unwrap_or_else(|| Self::print_usage());
                config.pattern = Self::string_to_pattern(&value);
                config.regex_like_pattern = false;
            } else if arg == "-g" {
                let value = args.next().unwrap_or_else(|| Self::print_usage());
                config.pattern = value;
                config.regex_like_pattern = true;
            } else if arg.starts_with('-') {
                Self::print_usage();
            } else {
                command_args.push(arg);
                command_args.extend(args);
                break;
            }
        }

        if !command_args.is_empty() {
            config.yank_argv = command_args;
        }

        let input = InputBuffer {
            data: Self::read_input()?,
        };

        let fields = Self::find_fields(&input.data, &config.pattern, config.ignore_case, input.data.len());

        let selected = if config.one_if_single && fields.len() == 1 {
            Some(0)
        } else {
            let (mut tty, screen_fields, output_len) = Self::terminal_setup(
                &input,
                &config.pattern,
                config.ignore_case,
                config.alternate_screen,
            )?;

            let selected = Self::terminal_main(&mut tty, &input, &screen_fields, output_len)?;
            Self::terminal_end(&mut tty)?;

            match selected {
                Some(screen_index) => {
                    let chosen = &screen_fields[screen_index];
                    fields
                        .iter()
                        .position(|f| f.so == chosen.so && f.eo == chosen.eo)
                }
                None => None,
            }
        };

        let Some(index) = selected else {
            return Ok(1);
        };

        let field = &fields[index];
        Self::yank_selection(&input.data[field.so..=field.eo], &config.yank_argv)?;
        Ok(0)
    }

    fn find_fields(data: &[u8], pattern: &str, ignore_case: bool, limit: usize) -> Vec<Field> {
        if let Some(excluded) = Self::parse_strtopat_pattern(pattern) {
            return Self::scan_delimited(data, &excluded, limit);
        }

        let haystack = String::from_utf8_lossy(&data[..limit]);
        let source = if ignore_case {
            haystack.to_lowercase()
        } else {
            haystack.into_owned()
        };
        let needle = if ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        if needle.is_empty() {
            return Vec::new();
        }

        let mut fields = Vec::new();
        let mut start = 0usize;
        while let Some(pos) = source[start..].find(&needle) {
            let so = start + pos;
            let eo = so + needle.len() - 1;
            fields.push(Field { lo: 0, so, eo });
            start = so + needle.len();
            if start >= source.len() {
                break;
            }
        }
        fields
    }

    fn scan_delimited(data: &[u8], excluded: &[char], limit: usize) -> Vec<Field> {
        let mut fields = Vec::new();
        let mut idx = 0usize;

        while idx < limit {
            while idx < limit {
                let ch = data[idx] as char;
                if excluded.contains(&ch) {
                    idx += 1;
                } else {
                    break;
                }
            }
            if idx >= limit {
                break;
            }
            let start = idx;
            while idx < limit {
                let ch = data[idx] as char;
                if excluded.contains(&ch) {
                    break;
                }
                idx += 1;
            }
            let end = idx.saturating_sub(1);
            if start <= end {
                fields.push(Field {
                    lo: 0,
                    so: start,
                    eo: end,
                });
            }
        }

        fields
    }

    fn parse_strtopat_pattern(pattern: &str) -> Option<Vec<char>> {
        let prefix = "[^";
        let suffix = "]+";
        if !pattern.starts_with(prefix) || !pattern.ends_with(suffix) {
            return None;
        }

        let inner = &pattern[prefix.len()..pattern.len() - suffix.len()];
        let mut excluded = Vec::new();
        let mut chars = inner.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                let escaped = match chars.next() {
                    Some('f') => '\u{000c}',
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some(other) => other,
                    None => '\\',
                };
                excluded.push(escaped);
            } else {
                excluded.push(ch);
            }
        }

        Some(excluded)
    }

    fn terminal_getc_key_kind(_state: &mut TerminalState) -> io::Result<Key> {
        Ok(Key::Other)
    }
}
