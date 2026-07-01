use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::io::Write as IoWrite;

const SDS_TYPE_5: u8 = 0;
const SDS_TYPE_8: u8 = 1;
const SDS_TYPE_16: u8 = 2;
const SDS_TYPE_32: u8 = 3;
const SDS_TYPE_64: u8 = 4;
const SDS_MAX_PREALLOC: usize = 1024 * 1024;
const SDS_LLSTR_SIZE: usize = 21;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SdsType {
    Type5,
    Type8,
    Type16,
    Type32,
    Type64,
}

impl SdsType {
    fn code(self) -> u8 {
        match self {
            Self::Type5 => SDS_TYPE_5,
            Self::Type8 => SDS_TYPE_8,
            Self::Type16 => SDS_TYPE_16,
            Self::Type32 => SDS_TYPE_32,
            Self::Type64 => SDS_TYPE_64,
        }
    }

    fn hdr_size(self) -> usize {
        match self {
            Self::Type5 => 1,
            Self::Type8 => 3,
            Self::Type16 => 5,
            Self::Type32 => 9,
            Self::Type64 => 17,
        }
    }
}

pub struct Sds {
    buf: Vec<u8>,
    len: usize,
}

impl Clone for Sds {
    fn clone(&self) -> Self {
        self.dup()
    }
}

impl Sds {
    fn ensure_terminator(&mut self) {
        if self.buf.len() <= self.len {
            self.buf.resize(self.len + 1, 0);
        } else {
            self.buf[self.len] = 0;
        }
    }

    fn alloc(&self) -> usize {
        self.buf.len().saturating_sub(1)
    }

    fn avail(&self) -> usize {
        self.alloc().saturating_sub(self.len)
    }

    pub fn hdr_size(type_code: u8) -> usize {
        match type_code & 0x7 {
            SDS_TYPE_5 => SdsType::Type5.hdr_size(),
            SDS_TYPE_8 => SdsType::Type8.hdr_size(),
            SDS_TYPE_16 => SdsType::Type16.hdr_size(),
            SDS_TYPE_32 => SdsType::Type32.hdr_size(),
            SDS_TYPE_64 => SdsType::Type64.hdr_size(),
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

    pub fn newlen(init: Option<&[u8]>, initlen: usize) -> Self {
        let mut ty = Self::req_type(initlen);
        if ty == SDS_TYPE_5 && initlen == 0 {
            ty = SDS_TYPE_8;
        }

        let mut buf = vec![0u8; initlen + 1];
        if let Some(src) = init {
            let copy_len = initlen.min(src.len());
            buf[..copy_len].copy_from_slice(&src[..copy_len]);
        }

        let _ = ty;
        Self { buf, len: initlen }
    }

    pub fn empty() -> Self {
        Self::newlen(Some(b""), 0)
    }

    pub fn new(init: &str) -> Self {
        Self::newlen(Some(init.as_bytes()), init.len())
    }

    pub fn dup(&self) -> Self {
        Self::newlen(Some(&self.buf[..self.len]), self.len)
    }

    pub fn updatelen(&mut self) {
        self.len = self
            .buf
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.buf.len());
        self.ensure_terminator();
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.ensure_terminator();
    }

    pub fn make_room_for(&mut self, addlen: usize) -> &mut Self {
        if self.avail() >= addlen {
            return self;
        }

        let len = self.len;
        let mut newlen = len + addlen;
        let reqlen = newlen;

        if newlen < SDS_MAX_PREALLOC {
            newlen *= 2;
        } else {
            newlen += SDS_MAX_PREALLOC;
        }

        let mut ty = Self::req_type(newlen);
        if ty == SDS_TYPE_5 {
            ty = SDS_TYPE_8;
        }
        let _hdrlen = Self::hdr_size(ty);
        assert!(_hdrlen + newlen + 1 > reqlen);

        if self.alloc() < newlen {
            self.buf.resize(newlen + 1, 0);
        }
        self.len = len;
        self.buf[self.len] = 0;
        self
    }

    pub fn remove_free_space(&mut self) -> &mut Self {
        if self.avail() == 0 {
            return self;
        }
        self.buf.truncate(self.len + 1);
        self.ensure_terminator();
        self
    }

    pub fn alloc_size(&self) -> usize {
        let alloc = self.alloc();
        Self::hdr_size(Self::req_type(alloc)) + alloc + 1
    }

    pub fn alloc_ptr(&self) -> usize {
        self.buf.as_ptr() as usize
    }

    pub fn incr_len(&mut self, incr: isize) {
        if incr >= 0 {
            let incr = incr as usize;
            assert!(self.avail() >= incr);
            self.len += incr;
        } else {
            let dec = (-incr) as usize;
            assert!(self.len >= dec);
            self.len -= dec;
        }
        self.ensure_terminator();
    }

    pub fn growzero(&mut self, len: usize) -> &mut Self {
        let curlen = self.len;
        if len <= curlen {
            return self;
        }
        self.make_room_for(len - curlen);
        if self.buf.len() < len + 1 {
            self.buf.resize(len + 1, 0);
        }
        for b in &mut self.buf[curlen..=len] {
            *b = 0;
        }
        self.len = len;
        self
    }

    pub fn catlen(&mut self, t: &[u8]) -> &mut Self {
        let curlen = self.len;
        self.make_room_for(t.len());
        if self.buf.len() < curlen + t.len() + 1 {
            self.buf.resize(curlen + t.len() + 1, 0);
        }
        self.buf[curlen..curlen + t.len()].copy_from_slice(t);
        self.len = curlen + t.len();
        self.buf[self.len] = 0;
        self
    }

    pub fn cat(&mut self, t: &str) -> &mut Self {
        self.catlen(t.as_bytes())
    }

    pub fn catsds(&mut self, t: &Sds) -> &mut Self {
        self.catlen(&t.buf[..t.len])
    }

    pub fn cpylen(&mut self, t: &[u8]) -> &mut Self {
        if self.alloc() < t.len() {
            self.make_room_for(t.len().saturating_sub(self.len));
        }
        if self.buf.len() < t.len() + 1 {
            self.buf.resize(t.len() + 1, 0);
        }
        self.buf[..t.len()].copy_from_slice(t);
        self.len = t.len();
        self.buf[self.len] = 0;
        self
    }

    pub fn cpy(&mut self, t: &str) -> &mut Self {
        self.cpylen(t.as_bytes())
    }

    pub fn ll_2_str(value: i64) -> String {
        value.to_string()
    }

    pub fn ull_2_str(value: u64) -> String {
        value.to_string()
    }

    pub fn fromlonglong(value: i64) -> Self {
        let s = Self::ll_2_str(value);
        Self::newlen(Some(s.as_bytes()), s.len())
    }

    pub fn catvprintf(&mut self, _fmt: &str, args: fmt::Arguments<'_>) -> &mut Self {
        let rendered = fmt::format(args);
        self.catlen(rendered.as_bytes())
    }

    pub fn catprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        let rendered = fmt::format(args);
        self.catlen(rendered.as_bytes())
    }

    pub fn catfmt(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        let rendered = fmt::format(args);
        self.catlen(rendered.as_bytes())
    }

    pub fn trim(&mut self, cset: &str) -> &mut Self {
        let set = cset.as_bytes();
        if self.len == 0 {
            return self;
        }

        let mut sp = 0usize;
        let mut ep = self.len - 1;

        while sp < self.len && set.contains(&self.buf[sp]) {
            sp += 1;
        }

        while ep > sp && set.contains(&self.buf[ep]) {
            ep -= 1;
        }

        let newlen = if sp >= self.len { 0 } else { (ep - sp) + 1 };

        if sp != 0 && newlen != 0 {
            self.buf.copy_within(sp..sp + newlen, 0);
        }
        self.len = newlen;
        self.ensure_terminator();
        self
    }

    pub fn range(&mut self, mut start: isize, mut end: isize) {
        let len = self.len;
        if len == 0 {
            return;
        }

        let slen = len as isize;
        if start < 0 {
            start += slen;
            if start < 0 {
                start = 0;
            }
        }
        if end < 0 {
            end += slen;
            if end < 0 {
                end = 0;
            }
        }

        let mut newlen = if start > end {
            0
        } else {
            (end - start + 1) as usize
        };

        if newlen != 0 {
            if start >= slen {
                newlen = 0;
            } else if end >= slen {
                end = slen - 1;
                newlen = (end - start + 1) as usize;
            }
        }

        if start > 0 && newlen > 0 {
            let s = start as usize;
            self.buf.copy_within(s..s + newlen, 0);
        }
        self.len = newlen;
        self.ensure_terminator();
    }

    pub fn tolower(&mut self) {
        for b in &mut self.buf[..self.len] {
            *b = b.to_ascii_lowercase();
        }
    }

    pub fn toupper(&mut self) {
        for b in &mut self.buf[..self.len] {
            *b = b.to_ascii_uppercase();
        }
    }

    pub fn cmp(&self, other: &Sds) -> i32 {
        let minlen = self.len.min(other.len);
        match self.buf[..minlen].cmp(&other.buf[..minlen]) {
            Ordering::Equal => {
                if self.len > other.len {
                    1
                } else if self.len < other.len {
                    -1
                } else {
                    0
                }
            }
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }

    pub fn splitlen(s: &[u8], sep: &[u8]) -> Vec<Sds> {
        if sep.is_empty() || s.is_empty() {
            return Vec::new();
        }

        let mut tokens = Vec::new();
        let mut start = 0usize;
        let mut j = 0usize;

        while j + sep.len() <= s.len() {
            let matched = if sep.len() == 1 {
                s[j] == sep[0]
            } else {
                &s[j..j + sep.len()] == sep
            };

            if matched {
                tokens.push(Sds::newlen(Some(&s[start..j]), j - start));
                start = j + sep.len();
                j += sep.len();
            } else {
                j += 1;
            }
        }

        tokens.push(Sds::newlen(Some(&s[start..]), s.len() - start));
        tokens
    }

    pub fn freesplitres(tokens: Vec<Sds>) {
        drop(tokens);
    }

    pub fn catrepr(&mut self, p: &[u8]) -> &mut Self {
        self.catlen(b"\"");
        for &ch in p {
            match ch {
                b'\\' | b'"' => {
                    self.catprintf(format_args!("\\{}", ch as char));
                }
                b'\n' => {
                    self.catlen(b"\\n");
                }
                b'\r' => {
                    self.catlen(b"\\r");
                }
                b'\t' => {
                    self.catlen(b"\\t");
                }
                0x07 => {
                    self.catlen(b"\\a");
                }
                0x08 => {
                    self.catlen(b"\\b");
                }
                _ => {
                    if (ch as char).is_ascii_graphic() || ch == b' ' {
                        self.catprintf(format_args!("{}", ch as char));
                    } else {
                        self.catprintf(format_args!("\\x{:02x}", ch));
                    }
                }
            }
        }
        self.catlen(b"\"");
        self
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
            while p < bytes.len() && (bytes[p] as char).is_ascii_whitespace() {
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
                        current.catlen(&[byte]);
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
                        current.catlen(&[c]);
                    } else if ch == b'"' {
                        if p + 1 < bytes.len() && !(bytes[p + 1] as char).is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.catlen(&[ch]);
                    }
                } else if insq {
                    if ch == b'\\' && p + 1 < bytes.len() && bytes[p + 1] == b'\'' {
                        p += 1;
                        current.catlen(b"'");
                    } else if ch == b'\'' {
                        if p + 1 < bytes.len() && !(bytes[p + 1] as char).is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.catlen(&[ch]);
                    }
                } else {
                    match ch {
                        b' ' | b'\n' | b'\r' | b'\t' | 0 => done = true,
                        b'"' => inq = true,
                        b'\'' => insq = true,
                        _ => {
                            current.catlen(&[ch]);
                        }
                    }
                }

                if p < bytes.len() {
                    p += 1;
                }
            }

            vector.push(current);
        }
    }

    pub fn mapchars(&mut self, from: &str, to: &str, setlen: usize) -> &mut Self {
        let from_b = from.as_bytes();
        let to_b = to.as_bytes();
        let setlen = setlen.min(from_b.len()).min(to_b.len());

        for j in 0..self.len {
            for i in 0..setlen {
                if self.buf[j] == from_b[i] {
                    self.buf[j] = to_b[i];
                    break;
                }
            }
        }
        self
    }

    pub fn join(argv: &[&str], sep: &str) -> Sds {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.cat(item);
            if j != argv.len().saturating_sub(1) {
                join.cat(sep);
            }
        }
        join
    }

    pub fn joinsds(argv: &[Sds], sep: &str) -> Sds {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.catsds(item);
            if j != argv.len().saturating_sub(1) {
                join.catlen(sep.as_bytes());
            }
        }
        join
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        vec![0u8; size]
    }

    pub fn realloc(mut buf: Vec<u8>, size: usize) -> Vec<u8> {
        buf.resize(size, 0);
        buf
    }

    pub fn test() -> i32 {
        let mut testnum = 0usize;
        let mut failed = 0usize;

        fn test_cond_numbered(testnum: &mut usize, failed: &mut usize, name: &str, cond: bool) {
            *testnum += 1;
            if cond {
                let _ = writeln!(std::io::stdout(), "{} - {}: PASSED", *testnum, name);
            } else {
                *failed += 1;
                let _ = writeln!(std::io::stdout(), "{} - {}: FAILED", *testnum, name);
            }
        }

        let mut x = Sds::new("foo");
        let mut y: Sds;

        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "Create a string and obtain the length",
            x.len == 3 && &x.buf[..4] == b"foo\0",
        );

        x = Sds::newlen(Some(b"foo"), 2);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "Create a string with specified length",
            x.len == 2 && &x.buf[..3] == b"fo\0",
        );

        x.cat("bar");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "Strings concatenation",
            x.len == 5 && &x.buf[..6] == b"fobar\0",
        );

        x.cpy("a");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscpy() against an originally longer string",
            x.len == 1 && &x.buf[..2] == b"a\0",
        );

        x.cpy("xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscpy() against an originally shorter string",
            x.len == 33 && &x.buf[..34] == b"xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk\0",
        );

        x = Sds::empty();
        x.catprintf(format_args!("{}", 123));
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatprintf() seems working in the base case",
            x.len == 3 && &x.buf[..4] == b"123\0",
        );

        x = Sds::empty();
        x.catlen(b"a");
        x.catlen(&[0]);
        x.catlen(b"b");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatprintf() seems working with \\0 inside of result",
            x.len == 3 && &x.buf[..4] == b"a\0b\0",
        );

        let etalon_len = 1024 * 1024;
        let etalon = vec![b'0'; etalon_len];
        x = Sds::empty();
        x.catlen(&etalon);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatprintf() can print 1MB",
            x.len == etalon_len && &x.buf[..x.len] == etalon.as_slice(),
        );

        x = Sds::new("--");
        x.catfmt(format_args!(
            "Hello {} World {},{}--",
            "Hi!",
            i64::MIN,
            i64::MAX
        ));
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatfmt() seems working in the base case",
            x.len == 60
                && &x.buf[..60]
                    == b"--Hello Hi! World -9223372036854775808,9223372036854775807--",
        );

        x = Sds::new("--");
        x.catfmt(format_args!("{},{}--", u32::MAX, u64::MAX));
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatfmt() seems working with unsigned numbers",
            x.len == 35 && &x.buf[..35] == b"--4294967295,18446744073709551615--",
        );

        x = Sds::new(" x ");
        x.trim(" x");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdstrim() works when all chars match",
            x.len == 0,
        );

        x = Sds::new(" x ");
        x.trim(" ");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdstrim() works when a single char remains",
            x.len == 1 && x.buf[0] == b'x',
        );

        x = Sds::new("xxciaoyyy");
        x.trim("xy");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdstrim() correctly trims characters",
            x.len == 4 && &x.buf[..5] == b"ciao\0",
        );

        y = x.dup();
        y.range(1, 1);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,1,1)",
            y.len == 1 && &y.buf[..2] == b"i\0",
        );

        y = x.dup();
        y.range(1, -1);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,1,-1)",
            y.len == 3 && &y.buf[..4] == b"iao\0",
        );

        y = x.dup();
        y.range(-2, -1);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,-2,-1)",
            y.len == 2 && &y.buf[..3] == b"ao\0",
        );

        y = x.dup();
        y.range(2, 1);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,2,1)",
            y.len == 0 && y.buf[0] == 0,
        );

        y = x.dup();
        y.range(1, 100);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,1,100)",
            y.len == 3 && &y.buf[..4] == b"iao\0",
        );

        y = x.dup();
        y.range(100, 100);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsrange(...,100,100)",
            y.len == 0 && y.buf[0] == 0,
        );

        x = Sds::new("foo");
        y = Sds::new("foa");
        test_cond_numbered(&mut testnum, &mut failed, "sdscmp(foo,foa)", x.cmp(&y) > 0);

        x = Sds::new("bar");
        y = Sds::new("bar");
        test_cond_numbered(&mut testnum, &mut failed, "sdscmp(bar,bar)", x.cmp(&y) == 0);

        x = Sds::new("aar");
        y = Sds::new("bar");
        test_cond_numbered(&mut testnum, &mut failed, "sdscmp(bar,bar)", x.cmp(&y) < 0);

        x = Sds::newlen(Some(b"\x07\n\x00foo\r"), 7);
        y = Sds::empty();
        y.catrepr(&x.buf[..x.len]);
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdscatrepr(...data...)",
            &y.buf[..15] == b"\"\\a\\n\\x00foo\\r\"",
        );

        x = Sds::new("0");
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsnew() free/len buffers",
            x.len == 1 && x.avail() == 0,
        );

        for _ in 0..10 {
            let oldlen = x.len;
            x.make_room_for(10);
            test_cond_numbered(
                &mut testnum,
                &mut failed,
                "sdsMakeRoomFor() len",
                x.len == oldlen,
            );
            test_cond_numbered(
                &mut testnum,
                &mut failed,
                "sdsMakeRoomFor() free",
                x.avail() >= 10,
            );
            for j in 0..10 {
                x.buf[oldlen + j] = b'A' + j as u8;
            }
            x.incr_len(10);
        }

        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsMakeRoomFor() content",
            &x.buf[..101]
                == b"0ABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJ",
        );
        test_cond_numbered(
            &mut testnum,
            &mut failed,
            "sdsMakeRoomFor() final length",
            x.len == 101,
        );

        Self::test_report(testnum - failed, failed);
        if failed == 0 { 0 } else { 1 }
    }

    pub fn main() -> i32 {
        Self::test()
    }

    pub fn main_root() -> i32 {
        Self::main()
    }

    pub fn s_malloc(size: usize) -> Vec<u8> {
        Self::malloc(size)
    }

    pub fn s_realloc(buf: Vec<u8>, size: usize) -> Vec<u8> {
        Self::realloc(buf, size)
    }

    pub fn s_free<T>(_value: T) {}

    pub fn test_cond(name: &str, cond: bool, passed: &mut usize, failed: &mut usize) {
        if cond {
            *passed += 1;
            let _ = writeln!(std::io::stdout(), "[ok] {name}");
        } else {
            *failed += 1;
            let _ = writeln!(std::io::stdout(), "[err] {name}");
        }
    }

    pub fn test_report(passed: usize, failed: usize) {
        let _ = writeln!(
            std::io::stdout(),
            "{} tests, {} passed, {} failed",
            passed + failed,
            passed,
            failed
        );
    }
}

impl Drop for Sds {
    fn drop(&mut self) {}
}
