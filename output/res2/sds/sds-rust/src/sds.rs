use std::cmp::Ordering;
use std::fmt::{self, Write};

const SDS_MAX_PREALLOC: usize = 1024 * 1024;
const SDS_TYPE_5: u8 = 0;
const SDS_TYPE_8: u8 = 1;
const SDS_TYPE_16: u8 = 2;
const SDS_TYPE_32: u8 = 3;
const SDS_TYPE_64: u8 = 4;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Sds {
    buf: Vec<u8>,
}

impl Sds {
    pub fn hdr_size(type_code: u8) -> usize {
        match type_code & 0x7 {
            SDS_TYPE_5 => 1,
            SDS_TYPE_8 => 3,
            SDS_TYPE_16 => 5,
            SDS_TYPE_32 => 9,
            SDS_TYPE_64 => 17,
            _ => 0,
        }
    }

    pub fn req_type(string_size: usize) -> u8 {
        if string_size < (1 << 5) {
            SDS_TYPE_5
        } else if string_size < (1 << 8) {
            SDS_TYPE_8
        } else if string_size < (1 << 16) {
            SDS_TYPE_16
        } else if cfg!(target_pointer_width = "64") && string_size < (1usize << 32) {
            SDS_TYPE_32
        } else if cfg!(target_pointer_width = "64") {
            SDS_TYPE_64
        } else {
            SDS_TYPE_32
        }
    }

    pub fn newlen(init: impl AsRef<[u8]>, initlen: usize) -> Self {
        let bytes = init.as_ref();
        let take = initlen.min(bytes.len());
        let mut buf = Vec::with_capacity(take);
        buf.extend_from_slice(&bytes[..take]);
        Self { buf }
    }

    pub fn empty() -> Self {
        Self {
            buf: Vec::with_capacity(0),
        }
    }

    pub fn new(init: impl AsRef<[u8]>) -> Self {
        let bytes = init.as_ref();
        let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        Self::newlen(bytes, nul)
    }

    pub fn dup(&self) -> Self {
        self.clone()
    }

    pub fn updatelen(&mut self) {
        if let Some(pos) = self.buf.iter().position(|&b| b == 0) {
            self.buf.truncate(pos);
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn make_room_for(&mut self, addlen: usize) -> &mut Self {
        let avail = self.buf.capacity().saturating_sub(self.buf.len());
        if avail >= addlen {
            return self;
        }

        let len = self.buf.len();
        let mut newlen = len.saturating_add(addlen);
        if newlen < SDS_MAX_PREALLOC {
            newlen = newlen.saturating_mul(2);
        } else {
            newlen = newlen.saturating_add(SDS_MAX_PREALLOC);
        }

        if newlen > self.buf.capacity() {
            self.buf.reserve_exact(newlen - self.buf.capacity());
        }
        self
    }

    pub fn remove_free_space(&mut self) -> &mut Self {
        self.buf.shrink_to_fit();
        self
    }

    pub fn alloc_size(&self) -> usize {
        Self::hdr_size(Self::req_type(self.buf.capacity())) + self.buf.capacity() + 1
    }

    pub fn alloc_ptr(&self) -> usize {
        self.buf.as_ptr() as usize
    }

    pub fn incr_len(&mut self, incr: isize) {
        if incr >= 0 {
            let incr = incr as usize;
            let new_len = self.buf.len() + incr;
            assert!(new_len <= self.buf.capacity());
            self.buf.resize(new_len, 0);
        } else {
            let decr = (-incr) as usize;
            assert!(decr <= self.buf.len());
            self.buf.truncate(self.buf.len() - decr);
        }
    }

    pub fn growzero(&mut self, len: usize) -> &mut Self {
        let curlen = self.buf.len();
        if len <= curlen {
            return self;
        }
        self.make_room_for(len - curlen);
        self.buf.resize(len, 0);
        self
    }

    pub fn catlen(&mut self, t: impl AsRef<[u8]>, len: usize) -> &mut Self {
        let bytes = t.as_ref();
        let take = len.min(bytes.len());
        self.make_room_for(take);
        self.buf.extend_from_slice(&bytes[..take]);
        self
    }

    pub fn cat(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let bytes = t.as_ref();
        let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        self.catlen(bytes, nul)
    }

    pub fn catsds(&mut self, t: &Sds) -> &mut Self {
        self.catlen(&t.buf, t.buf.len())
    }

    pub fn cpylen(&mut self, t: impl AsRef<[u8]>, len: usize) -> &mut Self {
        let bytes = t.as_ref();
        let take = len.min(bytes.len());
        if self.buf.capacity() < take {
            self.make_room_for(take.saturating_sub(self.buf.len()));
        }
        self.buf.clear();
        self.buf.extend_from_slice(&bytes[..take]);
        self
    }

    pub fn cpy(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let bytes = t.as_ref();
        let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        self.cpylen(bytes, nul)
    }

    pub fn ll_2_str(value: i64) -> String {
        value.to_string()
    }

    pub fn ull_2_str(value: u64) -> String {
        value.to_string()
    }

    pub fn fromlonglong(value: i64) -> Self {
        let s = Self::ll_2_str(value);
        Self::newlen(s.as_bytes(), s.len())
    }

    pub fn catvprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        let mut out = String::new();
        let _ = out.write_fmt(args);
        self.catlen(out.as_bytes(), out.len())
    }

    pub fn catprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        self.catvprintf(args)
    }

    pub fn catfmt(&mut self, fmt_str: &str, argv: &[CatFmtArg<'_>]) -> &mut Self {
        let mut argi = 0usize;
        let bytes = fmt_str.as_bytes();
        let mut i = 0usize;

        while i < bytes.len() {
            if bytes[i] != b'%' {
                self.catlen(&bytes[i..i + 1], 1);
                i += 1;
                continue;
            }

            if i + 1 >= bytes.len() {
                break;
            }

            let next = bytes[i + 1] as char;
            match next {
                's' => {
                    if let Some(CatFmtArg::Str(s)) = argv.get(argi) {
                        self.cat(s.as_bytes());
                    }
                    argi += 1;
                }
                'S' => {
                    if let Some(CatFmtArg::Sds(s)) = argv.get(argi) {
                        self.catsds(s);
                    }
                    argi += 1;
                }
                'i' => {
                    if let Some(CatFmtArg::Int(v)) = argv.get(argi) {
                        self.cat(Self::ll_2_str(*v as i64).as_bytes());
                    }
                    argi += 1;
                }
                'I' => {
                    if let Some(CatFmtArg::LongLong(v)) = argv.get(argi) {
                        self.cat(Self::ll_2_str(*v).as_bytes());
                    }
                    argi += 1;
                }
                'u' => {
                    if let Some(CatFmtArg::UInt(v)) = argv.get(argi) {
                        self.cat(Self::ull_2_str(*v as u64).as_bytes());
                    }
                    argi += 1;
                }
                'U' => {
                    if let Some(CatFmtArg::ULongLong(v)) = argv.get(argi) {
                        self.cat(Self::ull_2_str(*v).as_bytes());
                    }
                    argi += 1;
                }
                _ => {
                    self.catlen(&[bytes[i + 1]], 1);
                }
            }
            i += 2;
        }

        self
    }

    pub fn trim(&mut self, cset: impl AsRef<[u8]>) -> &mut Self {
        let cset = cset.as_ref();
        if self.buf.is_empty() {
            return self;
        }

        let mut start = 0usize;
        let mut end = self.buf.len();

        while start < end && cset.contains(&self.buf[start]) {
            start += 1;
        }
        while end > start && cset.contains(&self.buf[end - 1]) {
            end -= 1;
        }

        if start > 0 || end < self.buf.len() {
            self.buf = self.buf[start..end].to_vec();
        }
        self
    }

    pub fn range(&mut self, mut start: isize, mut end: isize) {
        let len = self.buf.len() as isize;
        if len == 0 {
            return;
        }

        if start < 0 {
            start += len;
            if start < 0 {
                start = 0;
            }
        }
        if end < 0 {
            end += len;
            if end < 0 {
                end = 0;
            }
        }

        let mut newlen = if start > end { 0 } else { end - start + 1 };
        if newlen != 0 {
            if start >= len {
                newlen = 0;
            } else if end >= len {
                end = len - 1;
                newlen = end - start + 1;
            }
        }

        if newlen <= 0 {
            self.buf.clear();
            return;
        }

        let start_usize = start as usize;
        let end_usize = (start + newlen) as usize;
        self.buf = self.buf[start_usize..end_usize].to_vec();
    }

    pub fn tolower(&mut self) {
        for b in &mut self.buf {
            *b = b.to_ascii_lowercase();
        }
    }

    pub fn toupper(&mut self) {
        for b in &mut self.buf {
            *b = b.to_ascii_uppercase();
        }
    }

    pub fn cmp(&self, other: &Sds) -> i32 {
        match self.buf.cmp(&other.buf) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    pub fn splitlen(s: impl AsRef<[u8]>, len: isize, sep: impl AsRef<[u8]>, seplen: usize) -> Vec<Sds> {
        let s = s.as_ref();
        let sep = sep.as_ref();

        if seplen < 1 || len <= 0 || sep.len() < seplen {
            return Vec::new();
        }

        let hay = &s[..(len as usize).min(s.len())];
        let needle = &sep[..seplen];
        let mut out = Vec::new();
        let mut start = 0usize;
        let mut j = 0usize;

        while j + seplen <= hay.len() {
            let matched = if seplen == 1 {
                hay[j] == needle[0]
            } else {
                &hay[j..j + seplen] == needle
            };

            if matched {
                out.push(Sds::newlen(&hay[start..j], j - start));
                start = j + seplen;
                j = start;
            } else {
                j += 1;
            }
        }

        out.push(Sds::newlen(&hay[start..], hay.len() - start));
        out
    }

    pub fn freesplitres(tokens: Vec<Sds>) {
        drop(tokens);
    }

    pub fn catrepr(&mut self, p: impl AsRef<[u8]>, len: usize) -> &mut Self {
        let p = p.as_ref();
        let take = len.min(p.len());
        self.catlen(b"\"", 1);
        for &ch in &p[..take] {
            match ch {
                b'\\' | b'"' => {
                    self.catlen(b"\\", 1);
                    self.catlen(&[ch], 1);
                }
                b'\n' => {
                    self.catlen(b"\\n", 2);
                }
                b'\r' => {
                    self.catlen(b"\\r", 2);
                }
                b'\t' => {
                    self.catlen(b"\\t", 2);
                }
                0x07 => {
                    self.catlen(b"\\a", 2);
                }
                0x08 => {
                    self.catlen(b"\\b", 2);
                }
                c if c.is_ascii_graphic() || c == b' ' => {
                    self.catlen(&[c], 1);
                }
                c => {
                    let escaped = format!("\\x{:02x}", c);
                    self.catlen(escaped.as_bytes(), escaped.len());
                }
            }
        }
        self.catlen(b"\"", 1)
    }

    pub fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    pub fn hex_digit_to_int(c: char) -> i32 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' | 'A' => 10,
            'b' | 'B' => 11,
            'c' | 'C' => 12,
            'd' | 'D' => 13,
            'e' | 'E' => 14,
            'f' | 'F' => 15,
            _ => 0,
        }
    }

    pub fn splitargs(line: &str) -> Option<Vec<Sds>> {
        let bytes = line.as_bytes();
        let mut p = 0usize;
        let mut vector: Vec<Sds> = Vec::new();

        loop {
            while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                p += 1;
            }

            if p >= bytes.len() {
                return Some(vector);
            }

            let mut current = Sds::empty();
            let mut inq = false;
            let mut insq = false;
            let mut done = false;

            while !done {
                let ch = if p < bytes.len() { bytes[p] } else { 0 };

                if inq {
                    if ch == b'\\'
                        && p + 3 < bytes.len()
                        && bytes[p + 1] == b'x'
                        && Self::is_hex_digit(bytes[p + 2] as char)
                        && Self::is_hex_digit(bytes[p + 3] as char)
                    {
                        let byte = (Self::hex_digit_to_int(bytes[p + 2] as char) * 16
                            + Self::hex_digit_to_int(bytes[p + 3] as char))
                            as u8;
                        current.catlen([byte], 1);
                        p += 3;
                    } else if ch == b'\\' && p + 1 < bytes.len() {
                        p += 1;
                        let c = match bytes[p] {
                            b'n' => b'\n',
                            b'r' => b'\r',
                            b't' => b'\t',
                            b'b' => 0x08,
                            b'a' => 0x07,
                            other => other,
                        };
                        current.catlen([c], 1);
                    } else if ch == b'"' {
                        if p + 1 < bytes.len() && !bytes[p + 1].is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.catlen([ch], 1);
                    }
                } else if insq {
                    if ch == b'\\' && p + 1 < bytes.len() && bytes[p + 1] == b'\'' {
                        p += 1;
                        current.catlen(b"'", 1);
                    } else if ch == b'\'' {
                        if p + 1 < bytes.len() && !bytes[p + 1].is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.catlen([ch], 1);
                    }
                } else {
                    match ch {
                        b' ' | b'\n' | b'\r' | b'\t' | 0 => done = true,
                        b'"' => inq = true,
                        b'\'' => insq = true,
                        _ => {
                            current.catlen([ch], 1);
                        }
                    }
                }

                if p < bytes.len() {
                    p += 1;
                } else if !done {
                    return None;
                }
            }

            vector.push(current);
        }
    }

    pub fn mapchars(&mut self, from: &str, to: &str, setlen: usize) -> &mut Self {
        let from_b = from.as_bytes();
        let to_b = to.as_bytes();
        let take = setlen.min(from_b.len()).min(to_b.len());

        for ch in &mut self.buf {
            for i in 0..take {
                if *ch == from_b[i] {
                    *ch = to_b[i];
                    break;
                }
            }
        }
        self
    }

    pub fn join(argv: &[&str], sep: &str) -> Self {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.cat(item.as_bytes());
            if j + 1 != argv.len() {
                join.cat(sep.as_bytes());
            }
        }
        join
    }

    pub fn joinsds(argv: &[Sds], sep: &str, seplen: usize) -> Self {
        let mut join = Sds::empty();
        let sep_b = sep.as_bytes();
        let take = seplen.min(sep_b.len());
        for (j, item) in argv.iter().enumerate() {
            join.catsds(item);
            if j + 1 != argv.len() {
                join.catlen(sep_b, take);
            }
        }
        join
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn realloc(mut buf: Vec<u8>, size: usize) -> Vec<u8> {
        if size > buf.len() {
            buf.resize(size, 0);
        } else {
            buf.truncate(size);
        }
        buf
    }

    pub fn s_malloc(size: usize) -> Vec<u8> {
        Self::malloc(size)
    }

    pub fn s_realloc(buf: Vec<u8>, size: usize) -> Vec<u8> {
        Self::realloc(buf, size)
    }

    pub fn s_free<T>(_value: T) {}

    pub fn test() -> bool {
        let mut state = TestState::default();

        let mut x = Sds::new("foo");
        state.cond(
            "Create a string and obtain the length",
            x.buf.len() == 3 && x.buf == b"foo",
        );

        x = Sds::newlen("foo", 2);
        state.cond(
            "Create a string with specified length",
            x.buf.len() == 2 && x.buf == b"fo",
        );

        x.cat("bar");
        state.cond("Strings concatenation", x.buf.len() == 5 && x.buf == b"fobar");

        x.cpy("a");
        state.cond(
            "sdscpy() against an originally longer string",
            x.buf.len() == 1 && x.buf == b"a",
        );

        x.cpy("xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk");
        state.cond(
            "sdscpy() against an originally shorter string",
            x.buf.len() == 33 && x.buf == b"xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk",
        );

        let mut p = Sds::empty();
        p.catprintf(format_args!("{}", 123));
        state.cond(
            "sdscatprintf() seems working in the base case",
            p.buf.len() == 3 && p.buf == b"123",
        );

        let mut p = Sds::empty();
        p.catlen(b"a", 1);
        p.catlen([0], 1);
        p.catlen(b"b", 1);
        state.cond(
            "sdscatprintf() seems working with \\0 inside of result",
            p.buf.len() == 3 && p.buf == b"a\0b",
        );

        let etalon = vec![b'0'; 1024 * 1024];
        let mut p = Sds::empty();
        let zeros = "0".repeat(etalon.len());
        p.catlen(zeros.as_bytes(), zeros.len());
        state.cond(
            "sdscatprintf() can print 1MB",
            p.buf.len() == etalon.len() && p.buf == etalon,
        );

        let mut q = Sds::new("--");
        q.catfmt(
            "Hello %s World %I,%I--",
            &[
                CatFmtArg::Str("Hi!"),
                CatFmtArg::LongLong(i64::MIN),
                CatFmtArg::LongLong(i64::MAX),
            ],
        );
        state.cond(
            "sdscatfmt() seems working in the base case",
            q.buf.len() == 60
                && q.buf
                    == b"--Hello Hi! World -9223372036854775808,9223372036854775807--",
        );
        println!("[{}]", String::from_utf8_lossy(&q.buf));

        let mut q = Sds::new("--");
        q.catfmt(
            "%u,%U--",
            &[
                CatFmtArg::UInt(u32::MAX),
                CatFmtArg::ULongLong(u64::MAX),
            ],
        );
        state.cond(
            "sdscatfmt() seems working with unsigned numbers",
            q.buf.len() == 35 && q.buf == b"--4294967295,18446744073709551615--",
        );

        let mut t = Sds::new(" x ");
        t.trim(" x");
        state.cond("sdstrim() works when all chars match", t.buf.is_empty());

        let mut t = Sds::new(" x ");
        t.trim(" ");
        state.cond(
            "sdstrim() works when a single char remains",
            t.buf.len() == 1 && t.buf[0] == b'x',
        );

        let mut t = Sds::new("xxciaoyyy");
        t.trim("xy");
        state.cond(
            "sdstrim() correctly trims characters",
            t.buf.len() == 4 && t.buf == b"ciao",
        );

        let mut y = t.dup();
        y.range(1, 1);
        state.cond("sdsrange(...,1,1)", y.buf.len() == 1 && y.buf == b"i");

        y = t.dup();
        y.range(1, -1);
        state.cond("sdsrange(...,1,-1)", y.buf.len() == 3 && y.buf == b"iao");

        y = t.dup();
        y.range(-2, -1);
        state.cond("sdsrange(...,-2,-1)", y.buf.len() == 2 && y.buf == b"ao");

        y = t.dup();
        y.range(2, 1);
        state.cond("sdsrange(...,2,1)", y.buf.is_empty());

        y = t.dup();
        y.range(1, 100);
        state.cond("sdsrange(...,1,100)", y.buf.len() == 3 && y.buf == b"iao");

        y = t.dup();
        y.range(100, 100);
        state.cond("sdsrange(...,100,100)", y.buf.is_empty());

        let a = Sds::new("foo");
        let b = Sds::new("foa");
        state.cond("sdscmp(foo,foa)", a.cmp(&b) > 0);

        let a = Sds::new("bar");
        let b = Sds::new("bar");
        state.cond("sdscmp(bar,bar)", a.cmp(&b) == 0);

        let a = Sds::new("aar");
        let b = Sds::new("bar");
        state.cond("sdscmp(bar,bar)", a.cmp(&b) < 0);

        let raw = Sds::newlen(b"\x07\n\0foo\r", 7);
        let mut repr = Sds::empty();
        repr.catrepr(&raw.buf, raw.buf.len());
        state.cond(
            "sdscatrepr(...data...)",
            repr.buf == b"\"\\a\\n\\x00foo\\r\"",
        );

        let mut x = Sds::new("0");
        state.cond(
            "sdsMakeRoomFor() len",
            x.buf.len() == 1 && x.buf.capacity().saturating_sub(x.buf.len()) == 0,
        );

        for _ in 0..10 {
            let oldlen = x.buf.len();
            x.make_room_for(1);
            let free = x.buf.capacity().saturating_sub(x.buf.len());
            x.buf.push(b'a');
            state.cond(
                "sdsMakeRoomFor() len",
                x.buf.len() == oldlen + 1 && free >= 1,
            );
        }
        state.cond(
            "sdsMakeRoomFor() content",
            x.buf == b"0aaaaaaaaaa",
        );
        state.cond(
            "sdsMakeRoomFor() final length",
            x.buf.len() == 11,
        );

        state.report();
        state.failed == 0
    }

    pub fn test_cond(name: &str, cond: bool) -> bool {
        let status = if cond { "PASSED" } else { "FAILED" };
        println!("- {}: {}", name, status);
        cond
    }

    pub fn test_report() {}

    pub fn main() -> i32 {
        if Self::test() { 0 } else { 1 }
    }

    pub fn main_root() -> i32 {
        Self::main()
    }
}

impl Drop for Sds {
    fn drop(&mut self) {}
}

pub enum CatFmtArg<'a> {
    Str(&'a str),
    Sds(&'a Sds),
    Int(i32),
    LongLong(i64),
    UInt(u32),
    ULongLong(u64),
}

#[derive(Default)]
struct TestState {
    index: usize,
    failed: usize,
}

impl TestState {
    fn cond(&mut self, name: &str, cond: bool) -> bool {
        self.index += 1;
        if !cond {
            self.failed += 1;
        }
        let status = if cond { "PASSED" } else { "FAILED" };
        println!("{} - {}: {}", self.index, name, status);
        cond
    }

    fn report(&self) {
        println!(
            "{} tests, {} passed, {} failed",
            self.index,
            self.index.saturating_sub(self.failed),
            self.failed
        );
    }
}
