use std::fmt;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WordwrapPosition {
    pub off: usize,
    pub col: usize,
}

pub struct Wordwrap {
    writer: Box<dyn Write>,
    buffer: Vec<u8>,
    cur: WordwrapPosition,
    last_ws: WordwrapPosition,
    ws_run: WordwrapPosition,
    left_margin: usize,
    right_margin: usize,
    next_left_margin: Option<usize>,
    word_start: Option<usize>,
    err: Option<io::ErrorKind>,
    unibyte: bool,
    indent: bool,
    closed: bool,
}

impl Wordwrap {
    const DEFAULT_RIGHT_MARGIN: usize = 79;
    const WORD_10: &str = "word";

    pub fn new_position(n: usize) -> WordwrapPosition {
        WordwrapPosition { off: n, col: n }
    }

    pub fn position_incr(pos: &mut WordwrapPosition, nbytes: usize) {
        pos.off += nbytes;
        pos.col += 1;
    }

    pub fn position_add(a: &mut WordwrapPosition, b: &WordwrapPosition) {
        a.off += b.off;
        a.col += b.col;
    }

    pub fn position_eq(a: &WordwrapPosition, b: &WordwrapPosition) -> bool {
        a.col == b.col
    }

    pub fn file(writer: Box<dyn Write>) -> Self {
        let mut wf = Self {
            writer,
            buffer: Vec::new(),
            cur: WordwrapPosition::default(),
            last_ws: WordwrapPosition::default(),
            ws_run: WordwrapPosition::default(),
            left_margin: 0,
            right_margin: Self::DEFAULT_RIGHT_MARGIN,
            next_left_margin: None,
            word_start: None,
            err: None,
            unibyte: false,
            indent: false,
            closed: false,
        };
        wf.right_margin = wf.detect_right_margin().max(1);
        wf.reset_line(true);
        wf
    }

    pub fn open(writer: impl Write + 'static) -> Self {
        Self::file(Box::new(writer))
    }

    pub fn fdopen(writer: impl Write + 'static) -> Self {
        Self::open(writer)
    }

    pub fn reset_line(&mut self, clrws: bool) {
        self.cur = Self::new_position(self.left_margin);
        self.unibyte = false;
        if clrws {
            self.ws_run = Self::new_position(0);
        }
    }

    pub fn detect_right_margin(&mut self) -> usize {
        std::env::var("COLUMNS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|&n| n > 0)
            .unwrap_or(Self::DEFAULT_RIGHT_MARGIN)
    }

    pub fn ww_fd_writer(writer: &mut dyn Write, bytes: &[u8]) -> io::Result<usize> {
        writer.write(bytes)
    }

    pub fn close(&mut self) -> io::Result<()> {
        if self.closed {
            return self.error();
        }
        let flush_result = self.flush();
        self.closed = true;
        flush_result
    }

    pub fn at_bol(&self) -> bool {
        self.cur.col == self.left_margin
    }

    pub fn at_eol(&self) -> bool {
        self.cur.col == self.right_margin
    }

    pub fn full_write(&mut self, size: usize) -> io::Result<usize> {
        let mut total = 0usize;
        while total < size {
            match self.writer.write(&self.buffer[total..size]) {
                Ok(0) => {
                    self.err = Some(io::ErrorKind::WriteZero);
                    return Err(io::Error::new(io::ErrorKind::WriteZero, "write returned zero"));
                }
                Ok(n) => total += n,
                Err(e) => {
                    self.err = Some(e.kind());
                    return Err(e);
                }
            }
        }
        Ok(total)
    }

    pub fn safe_mbrtowc(&mut self, s: &[u8]) -> Option<(char, usize)> {
        if s.is_empty() {
            return None;
        }
        if !self.unibyte {
            if let Ok(text) = std::str::from_utf8(s) {
                if let Some(ch) = text.chars().next() {
                    return Some((ch, ch.len_utf8()));
                }
            }
            self.unibyte = true;
        }
        Some((s[0] as char, 1))
    }

    pub fn wsprefix(&mut self, bytes: &[u8], size: usize) -> usize {
        let mut i = 0usize;
        let end = size.min(bytes.len());
        while i < end {
            let Some((wc, n)) = self.safe_mbrtowc(&bytes[i..end]) else {
                break;
            };
            if wc != ' ' && wc != '\t' {
                break;
            }
            i += n;
        }
        i
    }

    pub fn rescan(&mut self, size: usize) {
        self.reset_line(false);
        let limit = size.min(self.buffer.len());
        while self.cur.off < limit {
            let off = self.cur.off;
            let chunk = self.buffer[off..limit].to_vec();
            let Some((wc, n)) = self.safe_mbrtowc(&chunk) else {
                break;
            };

            if wc == ' ' || wc == '\t' {
                if !(self.ws_run.col > 0 && self.last_ws.col + self.ws_run.col == self.cur.col) {
                    self.last_ws = self.cur;
                    self.ws_run = Self::new_position(0);
                }
                Self::position_incr(&mut self.ws_run, n);
            }

            Self::position_incr(&mut self.cur, n);
        }
    }

    pub fn last_ws(
        &mut self,
        size: usize,
        last_ws: &mut WordwrapPosition,
    ) -> WordwrapPosition {
        let mut cur = WordwrapPosition::default();
        let mut ws_run = WordwrapPosition::default();
        *last_ws = Self::new_position(usize::MAX);

        let limit = size.min(self.buffer.len());
        while cur.off < limit {
            let chunk = self.buffer[cur.off..limit].to_vec();
            let Some((wc, n)) = self.safe_mbrtowc(&chunk) else {
                break;
            };
            if wc == ' ' || wc == '\t' {
                if !(ws_run.col > 0 && last_ws.col.saturating_add(ws_run.col) == cur.col) {
                    *last_ws = cur;
                    ws_run = Self::new_position(0);
                }
                Self::position_incr(&mut ws_run, n);
            } else {
                *last_ws = Self::new_position(usize::MAX);
                ws_run = Self::new_position(0);
            }
            Self::position_incr(&mut cur, n);
        }
        cur
    }

    pub fn flush_line(&mut self, size: usize) -> io::Result<()> {
        let unset = usize::MAX;
        let size = size.min(self.cur.off).min(self.buffer.len());

        let (mut pos, mut last_ws) =
            if self.ws_run.off > 0 && size == self.last_ws.off.saturating_add(self.ws_run.off) {
                (self.last_ws, self.last_ws)
            } else {
                let mut lw = WordwrapPosition::default();
                let p = self.last_ws(size, &mut lw);
                (p, lw)
            };

        if (pos.col >= self.left_margin && self.cur.col > self.left_margin) || size == self.cur.off {
            if last_ws.off != unset {
                pos = last_ws;
            }

            self.full_write(pos.off)?;
        }

        if let Err(e) = self.writer.write_all(b"\n") {
            self.err = Some(e.kind());
            return Err(e);
        }

        if let Some(next) = self.next_left_margin.take() {
            self.left_margin = next;
        }

        let mut n = self.cur.off.saturating_sub(size);
        if n > 0 {
            let tail = self.buffer[size..self.cur.off].to_vec();
            let wsn = self.wsprefix(&tail, n);
            let start = size + wsn;
            n -= wsn;

            if n > 0 {
                let new_len = self.left_margin + n;
                if self.buffer.len() < new_len {
                    self.buffer.resize(new_len, b' ');
                }
                self.buffer
                    .copy_within(start..start + n, self.left_margin);
                self.cur.off = self.left_margin + n;
                self.ws_run = Self::new_position(0);
            } else {
                self.cur.off = self.left_margin;
            }
        } else {
            self.cur.off = self.left_margin;
        }

        if self.indent {
            if self.buffer.len() < self.left_margin {
                self.buffer.resize(self.left_margin, b' ');
            }
            self.buffer[..self.left_margin].fill(b' ');
            self.indent = false;
            self.last_ws = Self::new_position(0);
            self.ws_run = Self::new_position(self.left_margin);
        }

        if self.buffer.len() < self.cur.off {
            self.buffer.resize(self.cur.off, b' ');
        }
        self.rescan(self.left_margin + n);
        self.buffer.truncate(self.cur.off);
        last_ws = self.last_ws;
        let _ = last_ws;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        if self.cur.col > self.left_margin {
            self.flush_line(self.cur.off)
        } else {
            Ok(())
        }
    }

    pub fn error(&self) -> io::Result<()> {
        match self.err {
            Some(kind) => Err(io::Error::new(kind, "wordwrap error")),
            None => Ok(()),
        }
    }

    pub fn set_left_margin(&mut self, left: usize) -> io::Result<()> {
        if left == self.left_margin {
            return Ok(());
        } else if left >= self.right_margin {
            self.err = Some(io::ErrorKind::InvalidInput);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "left margin must be less than right margin",
            ));
        }

        let bol = self.at_bol();
        self.left_margin = left;
        self.indent = true;

        if left < self.cur.col
            || (left == self.cur.col
                && (self.ws_run.col == 0
                    || self.cur.col > self.last_ws.col.saturating_add(self.ws_run.col)))
        {
            if !bol {
                self.flush_line(self.cur.off)?;
            } else {
                self.reset_line(true);
            }
        } else if left > self.cur.col {
            let n = self.left_margin - self.cur.col;
            if n > 0 {
                if self.buffer.len() < self.cur.off + n {
                    self.buffer.resize(self.cur.off + n, b' ');
                }
                self.buffer[self.cur.off..self.cur.off + n].fill(b' ');
                self.last_ws = self.cur;
                self.ws_run = Self::new_position(n);
                Self::position_add(&mut self.cur, &self.ws_run);
                self.unibyte = false;
            } else {
                self.reset_line(true);
            }
        }
        Ok(())
    }

    pub fn next_left_margin(&mut self, left: usize) -> io::Result<()> {
        if left == self.left_margin {
            return Ok(());
        } else if left >= self.right_margin {
            self.err = Some(io::ErrorKind::InvalidInput);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "next left margin must be less than right margin",
            ));
        }
        self.next_left_margin = Some(left);
        self.indent = true;
        Ok(())
    }

    pub fn set_right_margin(&mut self, right: usize) -> io::Result<()> {
        let right = if right == 0 {
            self.detect_right_margin()
        } else {
            right
        };

        if right == 0 || right <= self.left_margin {
            self.err = Some(io::ErrorKind::InvalidInput);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "right margin must be greater than left margin",
            ));
        }

        self.right_margin = right;
        if self.cur.col > self.right_margin {
            self.flush()?;
        }
        Ok(())
    }

    pub fn word_start(&mut self) {
        self.word_start = Some(self.cur.off);
    }

    pub fn word_end(&mut self) {
        self.word_start = None;
    }

    pub fn write(&mut self, text: &str) -> io::Result<()> {
        let bytes = text.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            let Some((wc, n)) = self.safe_mbrtowc(&bytes[i..]) else {
                break;
            };

            if wc == '\n' {
                let line_end = self.cur.off;
                self.flush_line(line_end)?;
                i += n;
                continue;
            }

            if self.cur.col >= self.right_margin && self.cur.col > self.left_margin {
                let line_end = self.cur.off;
                self.flush_line(line_end)?;
            }

            if self.buffer.len() < self.cur.off + n {
                self.buffer.resize(self.cur.off + n, 0);
            }
            self.buffer[self.cur.off..self.cur.off + n].copy_from_slice(&bytes[i..i + n]);

            if wc == ' ' || wc == '\t' {
                if !(self.ws_run.col > 0 && self.last_ws.col + self.ws_run.col == self.cur.col) {
                    self.last_ws = self.cur;
                    self.ws_run = Self::new_position(0);
                }
                Self::position_incr(&mut self.ws_run, n);
            } else {
                self.ws_run = Self::new_position(0);
            }

            Self::position_incr(&mut self.cur, n);
            i += n;

            if self.cur.col >= self.right_margin && self.cur.col > self.left_margin {
                let wrap_at = if self.last_ws.off != usize::MAX && self.last_ws.off < self.cur.off {
                    self.last_ws.off + self.ws_run.off
                } else {
                    self.cur.off
                };
                self.flush_line(wrap_at)?;
            }
        }

        Ok(())
    }

    pub fn puts(&mut self, text: &str) -> io::Result<()> {
        self.write(text)
    }

    pub fn putc(&mut self, c: char) -> io::Result<()> {
        let mut buf = [0u8; 4];
        self.write(c.encode_utf8(&mut buf))
    }

    pub fn para(&mut self) -> io::Result<()> {
        if self.at_bol() {
            self.putc('\n')
        } else {
            self.write("\n\n")
        }
    }

    pub fn vprintf(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        let rendered = fmt::format(args);
        self.write(&rendered)
    }

    pub fn printf(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
        self.vprintf(args)
    }

    pub fn module_cluster() -> &'static str {
        "module_src_parseopt_wordwrap.c_14"
    }

    pub fn word_10() -> &'static str {
        Self::WORD_10
    }
}
