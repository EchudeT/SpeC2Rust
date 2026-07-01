use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::{Command, ExitCode, Stdio};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const YANKCMD: &str = "xsel";

const T_ENTER_CA_MODE: &str = "\u{1b}[?1049h";
const T_EXIT_CA_MODE: &str = "\u{1b}[?1049l";
const T_SAVE_CURSOR: &str = "\u{1b}7";
const T_RESTORE_CURSOR: &str = "\u{1b}8";
const T_CLR_EOS: &str = "\u{1b}[J";
const T_CURSOR_VISIBLE: &str = "\u{1b}[?25h";
const T_CURSOR_INVISIBLE: &str = "\u{1b}[?25l";
const T_ENTER_STANDOUT_MODE: &str = "\u{1b}[7m";
const T_EXIT_STANDOUT_MODE: &str = "\u{1b}[27m";

const KEY_ENTER: i32 = 1;
const KEY_HOME: i32 = 2;
const KEY_TERM: i32 = 3;
const KEY_END: i32 = 4;
const KEY_RIGHT: i32 = 5;
const KEY_LEFT: i32 = 6;
const KEY_DOWN: i32 = 7;
const KEY_UP: i32 = 8;

#[derive(Clone, Debug)]
struct Field {
    lo: usize,
    so: usize,
    eo: usize,
}

#[derive(Clone, Debug)]
struct InputBuffer {
    v: Vec<u8>,
}

#[derive(Clone, Debug)]
struct FieldBuffer {
    v: Vec<Field>,
    output_len: usize,
}

#[derive(Clone, Debug)]
struct Options {
    one: bool,
    ignore_case: bool,
    use_ca_mode: bool,
    pattern: String,
    yank_argv: Vec<String>,
}

#[derive(Debug)]
struct TerminalState {
    use_ca_mode: bool,
    rfile: File,
    wfile: File,
}

pub struct Yank {
    input: InputBuffer,
    fields: FieldBuffer,
    terminal: Option<TerminalState>,
    options: Options,
}

impl Yank {
    pub fn main() -> ExitCode {
        match Self::real_main() {
            Ok(code) => ExitCode::from(code as u8),
            Err(err) => {
                let _ = writeln!(io::stderr(), "{err}");
                ExitCode::from(1)
            }
        }
    }

    pub fn main_root() -> ExitCode {
        Self::main()
    }

    pub fn real_main() -> Result<i32, String> {
        let mut args = env::args().collect::<Vec<_>>();
        let program = args
            .first()
            .cloned()
            .unwrap_or_else(|| "yank".to_string());

        let mut one = false;
        let mut ignore_case = false;
        let mut use_ca_mode = false;
        let mut pattern = Self::pattern_from_delimiters(" ");
        let mut command_start = None;
        let mut i = 1usize;

        while i < args.len() {
            let arg = &args[i];
            if arg == "--" {
                command_start = Some(i + 1);
                break;
            }
            if !arg.starts_with('-') || arg == "-" {
                command_start = Some(i);
                break;
            }

            match arg.as_str() {
                "-1" => one = true,
                "-i" => ignore_case = true,
                "-l" => pattern = Self::pattern_from_delimiters(""),
                "-x" => use_ca_mode = true,
                "-v" => {
                    println!("yank {VERSION}");
                    return Ok(0);
                }
                "-d" => {
                    i += 1;
                    let value = args.get(i).ok_or_else(|| Self::usage_text(&program))?;
                    pattern = Self::pattern_from_delimiters(value);
                }
                "-g" => {
                    i += 1;
                    let value = args.get(i).ok_or_else(|| Self::usage_text(&program))?;
                    pattern = value.clone();
                }
                _ => return Err(Self::usage_text(&program)),
            }
            i += 1;
        }

        let yank_argv = match command_start {
            Some(start) if start < args.len() => args.split_off(start),
            _ => vec![YANKCMD.to_string()],
        };

        let mut yank = Yank {
            input: InputBuffer { v: Vec::new() },
            fields: FieldBuffer {
                v: Vec::new(),
                output_len: 0,
            },
            terminal: None,
            options: Options {
                one,
                ignore_case,
                use_ca_mode,
                pattern,
                yank_argv,
            },
        };

        yank.input().map_err(|e| e.to_string())?;
        yank.tsetup().map_err(|e| e.to_string())?;

        let selected = if yank.options.one && yank.fields.v.len() == 1 {
            Some(0usize)
        } else {
            yank.tmain().map_err(|e| e.to_string())?
        };

        let tend_result = yank.tend().map_err(|e| e.to_string());
        tend_result?;

        let Some(index) = selected else {
            return Ok(1);
        };

        let field = yank.fields.v[index].clone();
        let bytes = yank.input.v[field.so..=field.eo].to_vec();
        yank.yank(&bytes).map_err(|e| e.to_string())?;
        Ok(0)
    }

    pub fn input(&mut self) -> io::Result<()> {
        let mut stdin = io::stdin().lock();
        stdin.read_to_end(&mut self.input.v)?;
        Ok(())
    }

    pub fn pattern_from_delimiters(s: &str) -> String {
        format!("[^{}\u{000C}\n\r\t]+", s)
    }

    pub fn field_cmp(f1: &Field, f2: &Field) -> i32 {
        let s1 = f1.so - f1.lo;
        let e1 = f1.eo - f1.lo;
        let s2 = f2.so - f2.lo;
        let e2 = f2.eo - f2.lo;

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

    pub fn yank(&self, bytes: &[u8]) -> io::Result<()> {
        use std::io::IsTerminal;

        if !io::stdout().is_terminal() && self.options.yank_argv.len() == 1 && self.options.yank_argv[0] == YANKCMD {
            let mut stdout = io::stdout().lock();
            return Self::xwrite(&mut stdout, bytes);
        }

        let mut command = Command::new(&self.options.yank_argv[0]);
        if self.options.yank_argv.len() > 1 {
            command.args(&self.options.yank_argv[1..]);
        }

        let mut child = command.stdin(Stdio::piped()).spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            Self::xwrite(stdin, bytes)?;
        }

        let status = child.wait()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("command exited with status {status}"),
            ))
        }
    }

    pub fn twrite(&mut self, bytes: &[u8]) -> io::Result<()> {
        if let Some(terminal) = self.terminal.as_mut() {
            Self::xwrite(&mut terminal.wfile, bytes)
        } else {
            let mut stderr = io::stderr().lock();
            Self::xwrite(&mut stderr, bytes)
        }
    }

    pub fn tputs(&mut self, s: &str) -> io::Result<()> {
        self.twrite(s.as_bytes())
    }

    pub fn tsetup(&mut self) -> io::Result<()> {
        self.fields.v.clear();
        self.fields.output_len = self.input.v.len();

        let matches = self.compute_matches();
        self.fields.v = matches;
        self.assign_line_offsets(80, 24);

        let rfile = File::open("/dev/tty")?;
        let wfile = File::options().write(true).open("/dev/tty")?;
        self.terminal = Some(TerminalState {
            use_ca_mode: self.options.use_ca_mode,
            rfile,
            wfile,
        });

        if self.options.use_ca_mode {
            self.tputs(T_ENTER_CA_MODE)?;
        }
        self.tputs(T_CURSOR_INVISIBLE)?;
        self.tputs(T_SAVE_CURSOR)?;
        Ok(())
    }

    pub fn tend(&mut self) -> io::Result<()> {
        self.tputs(T_RESTORE_CURSOR)?;
        self.tputs(T_CLR_EOS)?;
        self.tputs(T_CURSOR_VISIBLE)?;
        if let Some(terminal) = &self.terminal {
            if terminal.use_ca_mode {
                self.tputs(T_EXIT_CA_MODE)?;
            }
        }
        self.terminal = None;
        Ok(())
    }

    pub fn tgetc(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 3];
        let n = if let Some(terminal) = self.terminal.as_mut() {
            terminal.rfile.read(&mut buf)?
        } else {
            io::stdin().lock().read(&mut buf)?
        };
        if n == 0 {
            return Ok(KEY_TERM);
        }
        let slice = &buf[..n];

        let keys: &[(&[u8], i32)] = &[
            (b"\n", KEY_ENTER),
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

        for (pattern, code) in keys {
            if slice.starts_with(pattern) {
                return Ok(*code);
            }
        }

        Ok(0)
    }

    pub fn tmain(&mut self) -> io::Result<Option<usize>> {
        let mut i: isize = 0;
        let mut j: isize = 0;
        let n = self.fields.output_len;

        loop {
            self.tputs(T_RESTORE_CURSOR)?;
            if !self.fields.v.is_empty() {
                let current_index = i as usize;
                let current_so = self.fields.v[current_index].so;
                let current_eo = self.fields.v[current_index].eo;

                let before = self.input.v[..current_so].to_vec();
                let current = self.input.v[current_so..=current_eo].to_vec();
                let after = if current_eo + 1 < n {
                    self.input.v[current_eo + 1..n].to_vec()
                } else {
                    Vec::new()
                };

                self.twrite(&before)?;
                self.tputs(T_ENTER_STANDOUT_MODE)?;
                self.twrite(&current)?;
                self.tputs(T_EXIT_STANDOUT_MODE)?;
                if !after.is_empty() {
                    self.twrite(&after)?;
                }
            } else {
                let all = self.input.v[..n].to_vec();
                self.twrite(&all)?;
            }

            let c = self.tgetc()?;
            match c {
                KEY_ENTER => {
                    if !self.fields.v.is_empty() {
                        return Ok(Some(i as usize));
                    }
                }
                KEY_TERM => return Ok(None),
                KEY_HOME => j = 0,
                KEY_RIGHT => j = i + 1,
                KEY_END => j = self.fields.v.len() as isize - 1,
                KEY_LEFT => j = i - 1,
                KEY_DOWN | KEY_UP => {
                    if self.fields.v.is_empty() {
                        continue;
                    }

                    if c == KEY_DOWN {
                        j = i;
                        while j < self.fields.v.len() as isize
                            && self.fields.v[i as usize].lo == self.fields.v[j as usize].lo
                        {
                            j += 1;
                        }
                        if j == self.fields.v.len() as isize {
                            continue;
                        }
                    } else {
                        let mut k = i;
                        while k > 0 && self.fields.v[i as usize].lo == self.fields.v[k as usize].lo {
                            k -= 1;
                        }
                        j = k;
                        while j > 0 && self.fields.v[(j - 1) as usize].lo == self.fields.v[k as usize].lo
                        {
                            j -= 1;
                        }
                    }

                    while j >= 0
                        && (j as usize) + 1 < self.fields.v.len()
                        && Self::field_cmp(&self.fields.v[i as usize], &self.fields.v[j as usize]) < 0
                        && self.fields.v[j as usize].lo == self.fields.v[j as usize + 1].lo
                    {
                        j += 1;
                    }
                }
                _ => {}
            }

            if j >= 0 && j < self.fields.v.len() as isize {
                i = j;
            }
        }
    }

    pub fn usage_message() -> String {
        Self::usage_text("yank")
    }

    fn usage_text(program: &str) -> String {
        format!("usage: {program} [-1ilxv] [-d delim] [-g pattern] [-- command [args]]")
    }

    fn compute_matches(&self) -> Vec<Field> {
        let mut result = Vec::new();
        let bytes = &self.input.v;
        let delims = Self::parse_delimiters(&self.options.pattern);
        let mut start = 0usize;

        while start < bytes.len() {
            while start < bytes.len() && Self::is_delim(bytes[start], &delims) {
                start += 1;
            }
            if start >= bytes.len() {
                break;
            }

            let mut end = start;
            while end < bytes.len() && !Self::is_delim(bytes[end], &delims) {
                end += 1;
            }

            if end > start {
                result.push(Field {
                    lo: 0,
                    so: start,
                    eo: end - 1,
                });
            }
            start = end;
        }

        if self.options.ignore_case {
            result.sort_by(|a, b| {
                let sa = String::from_utf8_lossy(&bytes[a.so..=a.eo]).to_lowercase();
                let sb = String::from_utf8_lossy(&bytes[b.so..=b.eo]).to_lowercase();
                sa.cmp(&sb)
            });
        }

        result
    }

    fn parse_delimiters(pattern: &str) -> Vec<u8> {
        if let Some(inner) = pattern
            .strip_prefix("[^")
            .and_then(|s| s.strip_suffix("]+"))
        {
            inner.as_bytes().to_vec()
        } else {
            b" \x0C\n\r\t".to_vec()
        }
    }

    fn is_delim(byte: u8, delims: &[u8]) -> bool {
        delims.contains(&byte)
    }

    fn assign_line_offsets(&mut self, cols: usize, rows: usize) {
        let mut s = 0usize;
        let mut e = 0usize;
        let mut n = min(cols.saturating_mul(rows), self.input.v.len());
        let mut i = 0usize;
        let mut j = 0usize;

        while n > 0 && i < rows {
            if s == e {
                if let Some(pos) = self.input.v[s.saturating_add(1)..s + n]
                    .iter()
                    .position(|&b| b == b'\n')
                {
                    e = s + 1 + pos;
                } else {
                    e = self.input.v.len();
                }
            }

            let w = min(e.saturating_sub(s), cols);
            while j < self.fields.v.len() && self.fields.v[j].so < s + w {
                self.fields.v[j].lo = s;
                j += 1;
            }
            s += w;
            n = n.saturating_sub(w);
            i += 1;
        }

        self.fields.v.truncate(j);
        if n > 0 && !self.fields.v.is_empty() {
            let last = self.fields.v.last_mut().expect("non-empty");
            if last.eo.saturating_sub(last.lo) >= cols {
                last.eo = last.lo + cols - 1;
            }
        }
        self.fields.output_len = s.saturating_sub(1);
    }
}
