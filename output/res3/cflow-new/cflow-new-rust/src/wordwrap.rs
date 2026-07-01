use std::fmt::{self, Write as _};
use std::io::{self, Write};

const DEFAULT_RIGHT_MARGIN: usize = 79;
const UNSET: usize = usize::MAX;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModuleSrcParseoptPosition06 {
    pub off: usize,
    pub col: usize,
}

impl ModuleSrcParseoptPosition06 {
    pub fn new(n: usize) -> Self {
        Self { off: n, col: n }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Word10 {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub struct WordwrapFile {
    pub fd: Option<i32>,
    pub left_margin: usize,
    pub right_margin: usize,
    pub next_left_margin: usize,
    pub word_start: usize,
    pub err: Option<i32>,
    pub indent: bool,
    pub unibyte: bool,
    pub cur: ModuleSrcParseoptPosition06,
    pub last_ws: ModuleSrcParseoptPosition06,
    pub ws_run: ModuleSrcParseoptPosition06,
    pub buffer: Vec<u8>,
    pub words: Vec<Word10>,
    pub closed: bool,
}

impl Default for WordwrapFile {
    fn default() -> Self {
        let left_margin = 0usize;
        Self {
            fd: None,
            left_margin,
            right_margin: DEFAULT_RIGHT_MARGIN,
            next_left_margin: UNSET,
            word_start: UNSET,
            err: None,
            indent: false,
            unibyte: false,
            cur: ModuleSrcParseoptPosition06 {
                off: left_margin,
                col: left_margin,
            },
            last_ws: ModuleSrcParseoptPosition06::default(),
            ws_run: ModuleSrcParseoptPosition06::default(),
            buffer: Vec::new(),
            words: Vec::new(),
            closed: false,
        }
    }
}

pub struct Wordwrap {
    sink: Box<dyn Write>,
    file: WordwrapFile,
}

impl Wordwrap {
    pub fn open<W: Write + 'static>(writer: W) -> Self {
        let mut this = Self {
            sink: Box::new(writer),
            file: Self::file(),
        };
        let _ = this.set_right_margin(0);
        this
    }

    pub fn fdopen<W: Write + 'static>(writer: W) -> Self {
        Self::open(writer)
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }

    pub fn position_incr(pos: &mut ModuleSrcParseoptPosition06, nbytes: usize) {
        pos.off += nbytes;
        pos.col += 1;
    }


    pub fn position_add(
        a: &mut ModuleSrcParseoptPosition06,
        b: &ModuleSrcParseoptPosition06,
    ) {
        a.off += b.off;
        a.col += b.col;
    }

    pub fn position_eq(
        a: &ModuleSrcParseoptPosition06,
        b: &ModuleSrcParseoptPosition06,
    ) -> bool {
        a.col == b.col
    }

    pub fn line(&mut self, clrws: bool) {
        self.file.cur = ModuleSrcParseoptPosition06::new(self.file.left_margin);
        self.file.unibyte = false;
        if clrws {
            self.file.ws_run = ModuleSrcParseoptPosition06::new(0);
        }
    }

    pub fn detect_right_margin(&self) -> usize {
        std::env::var("COLUMNS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|&n| n > 0)
            .unwrap_or(DEFAULT_RIGHT_MARGIN)
    }

    pub fn ww_fd_writer(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.sink.write(bytes)
    }

    pub fn at_bol(&self) -> bool {
        self.file.cur.col == self.file.left_margin
    }

    pub fn at_eol(&self) -> bool {
        self.file.cur.col == self.file.right_margin
    }

    pub fn full_write(&mut self, size: usize) -> io::Result<usize> {
        let mut total = 0usize;
        while total < size {
            let chunk = self.file.buffer[total..size].to_vec();
            match self.ww_fd_writer(&chunk) {
                Ok(0) => {
                    self.file.err = Some(28);
                    return Err(io::Error::new(io::ErrorKind::WriteZero, "short write"));
                }
                Ok(n) => total += n,
                Err(e) => {
                    self.file.err = e.raw_os_error();
                    return Err(e);
                }
            }
        }
        Ok(total)
    }

    pub fn safe_mbrtowc(&mut self, s: &[u8]) -> (char, usize) {
        if !self.file.unibyte {
            if let Ok(text) = std::str::from_utf8(s) {
                if let Some(ch) = text.chars().next() {
                    return (ch, ch.len_utf8());
                }
            }
            self.file.unibyte = true;
        }
        (s.first().copied().unwrap_or_default() as char, usize::from(!s.is_empty()))
    }

    pub fn wsprefix(&mut self, strv: &[u8]) -> usize {
        let mut i = 0usize;
        while i < strv.len() {
            let (wc, n) = self.safe_mbrtowc(&strv[i..]);
            if !matches!(wc, ' ' | '\t') {
                break;
            }
            if n == 0 {
                break;
            }
            i += n;
        }
        i
    }

    pub fn rescan(&mut self, size: usize) {
        self.line(false);
        let limit = size.min(self.file.buffer.len());
        while self.file.cur.off < limit {
            let off = self.file.cur.off;
            let chunk = self.file.buffer[off..limit].to_vec();
            let (wc, n) = self.safe_mbrtowc(&chunk);
            if matches!(wc, ' ' | '\t') {
                if !(self.file.ws_run.col > 0
                    && self.file.last_ws.col + self.file.ws_run.col == self.file.cur.col)
                {
                    self.file.last_ws = self.file.cur;
                    self.file.ws_run = ModuleSrcParseoptPosition06::new(0);
                }
                Self::position_incr(&mut self.file.ws_run, n);
            }
            Self::position_incr(&mut self.file.cur, n);
        }
    }

    pub fn last_ws(
        &mut self,
        size: usize,
        last_ws: &mut ModuleSrcParseoptPosition06,
    ) -> ModuleSrcParseoptPosition06 {
        let mut cur = Self::module_src_parseopt_position_06();
        let mut ws_run = Self::module_src_parseopt_position_06();
        *last_ws = ModuleSrcParseoptPosition06::new(UNSET);

        let limit = size.min(self.file.buffer.len());
        while cur.off < limit {
            let chunk = self.file.buffer[cur.off..limit].to_vec();
            let (wc, n) = self.safe_mbrtowc(&chunk);
            if matches!(wc, ' ' | '\t') {
                if !(ws_run.col > 0 && last_ws.col.saturating_add(ws_run.col) == cur.col) {
                    *last_ws = cur;
                    ws_run = ModuleSrcParseoptPosition06::new(0);
                }
                Self::position_incr(&mut ws_run, n);
            } else {
                *last_ws = ModuleSrcParseoptPosition06::new(UNSET);
                ws_run = ModuleSrcParseoptPosition06::new(0);
            }
            Self::position_incr(&mut cur, n);
        }
        cur
    }

    pub fn flush_line(&mut self, size: usize) -> io::Result<()> {
        let mut last_ws = Self::module_src_parseopt_position_06();
        let mut pos;

        if self.file.ws_run.off > 0 && size == self.file.last_ws.off.saturating_add(self.file.ws_run.off)
        {
            pos = self.file.last_ws;
            last_ws = self.file.last_ws;
        } else {
            pos = self.last_ws(size, &mut last_ws);
        }

        if (pos.col >= self.file.left_margin && self.file.cur.col > self.file.left_margin)
            || size == self.file.cur.off
        {
            if last_ws.off != UNSET {
                pos = last_ws;
            }

            let n = self.full_write(pos.off)?;
            if n < pos.off {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "partial wrapped write",
                ));
            }
        }

        self.sink.write_all(b"\n")?;

        if self.file.next_left_margin != UNSET {
            self.file.left_margin = self.file.next_left_margin;
            self.file.next_left_margin = UNSET;
        }

        let mut remain = self.file.cur.off.saturating_sub(size);
        let mut start = size;
        if remain > 0 {
            let chunk = self.file.buffer[size..self.file.cur.off].to_vec();
            let wsn = self.wsprefix(&chunk);
            start += wsn;
            remain -= wsn;

            if remain > 0 {
                let dst = self.file.left_margin;
                self.file.buffer.copy_within(start..start + remain, dst);
                self.file.cur.off = self.file.left_margin + remain;
            } else {
                self.file.cur.off = self.file.left_margin;
            }
            self.file.ws_run = ModuleSrcParseoptPosition06::new(0);
        } else {
            self.file.cur.off = self.file.left_margin;
        }

        if self.file.indent {
            if self.file.buffer.len() < self.file.left_margin {
                self.file.buffer.resize(self.file.left_margin, b' ');
            } else {
                self.file.buffer[..self.file.left_margin].fill(b' ');
            }
            self.file.indent = false;
            self.file.last_ws = ModuleSrcParseoptPosition06::new(0);
            self.file.ws_run = ModuleSrcParseoptPosition06::new(self.file.left_margin);
        }

        self.rescan(self.file.left_margin + remain);
        if self.file.buffer.len() > self.file.cur.off {
            self.file.buffer.truncate(self.file.cur.off);
        }
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.file.cur.col > self.file.left_margin {
            self.flush_line(self.file.cur.off)?;
        }
        self.sink.flush()
    }

    pub fn error(&self) -> Option<i32> {
        self.file.err
    }

    pub fn set_left_margin(&mut self, left: usize) -> io::Result<()> {
        if left == self.file.left_margin {
            return Ok(());
        } else if left >= self.file.right_margin {
            self.file.err = Some(22);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "left margin must be smaller than right margin",
            ));
        }

        let bol = self.at_bol();
        self.file.left_margin = left;
        self.file.indent = true;

        if left < self.file.cur.col
            || (left == self.file.cur.col
                && (self.file.ws_run.col == 0
                    || self.file.cur.col > self.file.last_ws.col + self.file.ws_run.col))
        {
            if !bol {
                self.flush_line(self.file.cur.off)?;
            } else {
                self.line(true);
            }
        } else if left > self.file.cur.col {
            let n = self.file.left_margin - self.file.cur.col;
            if n > 0 {
                if self.file.buffer.len() < self.file.cur.off + n {
                    self.file.buffer.resize(self.file.cur.off + n, 0);
                }
                self.file.buffer[self.file.cur.off..self.file.cur.off + n].fill(b' ');
                self.file.last_ws = self.file.cur;
                self.file.ws_run = ModuleSrcParseoptPosition06::new(n);
                Self::position_add(&mut self.file.cur, &self.file.ws_run);
                self.file.unibyte = false;
            } else {
                self.line(true);
            }
        }
        Ok(())
    }

    pub fn next_left_margin(&mut self, left: usize) -> io::Result<()> {
        if left == self.file.left_margin {
            return Ok(());
        } else if left >= self.file.right_margin {
            self.file.err = Some(22);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "next left margin must be smaller than right margin",
            ));
        }
        self.file.next_left_margin = left;
        self.file.indent = true;
        Ok(())
    }

    pub fn set_right_margin(&mut self, right: usize) -> io::Result<()> {
        let right = if right == 0 {
            self.detect_right_margin()
        } else {
            right
        };

        if right <= self.file.left_margin {
            self.file.err = Some(22);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "right margin must be greater than left margin",
            ));
        }

        self.file.right_margin = right;

        if self.file.cur.col > self.file.right_margin {
            self.flush()?;
        }
        Ok(())
    }

    pub fn word_start(&mut self) {
        self.file.word_start = self.file.cur.off;
    }

    pub fn word_end(&mut self) {
        self.file.word_start = UNSET;
    }

    pub fn write(&mut self, strv: &str) -> io::Result<()> {
        let mut i = 0usize;
        let bytes = strv.as_bytes();

        while i < bytes.len() {
            let (wc, n) = self.safe_mbrtowc(&bytes[i..]);
            if n == 0 {
                break;
            }

            if wc == '\n' {
                if self.file.buffer.len() < self.file.cur.off {
                    self.file.buffer.resize(self.file.cur.off, b' ');
                }
                self.flush_line(self.file.cur.off)?;
                i += n;
                continue;
            }

            if self.file.cur.col == self.file.right_margin
                && !matches!(wc, ' ' | '\t')
                && self.file.cur.col > self.file.left_margin
            {
                self.flush_line(i)?;
            }

            if self.file.buffer.len() < self.file.cur.off + n {
                self.file.buffer.resize(self.file.cur.off + n, 0);
            }
            self.file.buffer[self.file.cur.off..self.file.cur.off + n]
                .copy_from_slice(&bytes[i..i + n]);

            if matches!(wc, ' ' | '\t') {
                if !(self.file.ws_run.col > 0
                    && self.file.last_ws.col + self.file.ws_run.col == self.file.cur.col)
                {
                    self.file.last_ws = self.file.cur;
                    self.file.ws_run = ModuleSrcParseoptPosition06::new(0);
                }
                Self::position_incr(&mut self.file.ws_run, n);
            } else {
                self.file.ws_run = ModuleSrcParseoptPosition06::new(0);
            }

            Self::position_incr(&mut self.file.cur, n);
            i += n;

            if self.file.cur.col >= self.file.right_margin && self.file.cur.col > self.file.left_margin
            {
                self.flush_line(i)?;
            }
        }

        Ok(())
    }

    pub fn puts(&mut self, strv: &str) -> io::Result<()> {
        self.write(strv)
    }

    pub fn putc(&mut self, c: char) -> io::Result<()> {
        let mut buf = [0u8; 4];
        self.write(c.encode_utf8(&mut buf))
    }

    pub fn para(&mut self) -> io::Result<()> {
        if self.at_bol() {
            self.sink.write_all(b"\n")?;
            self.sink.flush()
        } else {
            self.write("\n\n")
        }
    }

    pub fn vprintf(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        let mut rendered = String::new();
        rendered
            .write_fmt(args)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "formatting failed"))?;
        self.write(&rendered)
    }

    pub fn printf(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        self.vprintf(args)
    }

    pub fn module_cluster() -> &'static str {
        "module_src_parseopt_wordwrap.c_14"
    }

    pub fn file() -> WordwrapFile {
        WordwrapFile::default()
    }

    pub fn module_src_parseopt_position_06() -> ModuleSrcParseoptPosition06 {
        ModuleSrcParseoptPosition06::default()
    }

    pub fn word_10() -> Word10 {
        Word10::default()
    }
}
