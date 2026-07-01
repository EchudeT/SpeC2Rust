use std::cmp::Ordering;
use std::fmt::{self, Write};

const SDS_TYPE_5: u8 = 0;
const SDS_TYPE_8: u8 = 1;
const SDS_TYPE_16: u8 = 2;
const SDS_TYPE_32: u8 = 3;
const SDS_TYPE_64: u8 = 4;
const SDS_MAX_PREALLOC: usize = 1024 * 1024;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Sds {
    buf: Vec<u8>,
    len: usize,
}

impl Sds {
    fn ensure_nul(&mut self) {
        if self.buf.is_empty() {
            self.buf.push(0);
            self.len = 0;
            return;
        }
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

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn as_str_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_bytes()).into_owned()
    }

    pub fn hdr_size(type_code: u8) -> usize {
        match type_code & 7 {
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
        } else if usize::BITS >= 64 && (string_size as u128) < (1u128 << 32) {
            SDS_TYPE_32
        } else if usize::BITS >= 64 {
            SDS_TYPE_64
        } else {
            SDS_TYPE_32
        }
    }

    pub fn newlen(init: impl AsRef<[u8]>) -> Self {
        let src = init.as_ref();
        let mut buf = Vec::with_capacity(src.len() + 1);
        buf.extend_from_slice(src);
        buf.push(0);
        Self { buf, len: src.len() }
    }

    pub fn empty() -> Self {
        Self::newlen([])
    }

    pub fn new(init: impl AsRef<str>) -> Self {
        Self::newlen(init.as_ref().as_bytes())
    }

    pub fn dup(&self) -> Self {
        self.clone()
    }

    pub fn updatelen(&mut self) {
        let new_len = self
            .buf
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.buf.len());
        if new_len == self.buf.len() {
            self.len = self.buf.len().saturating_sub(1);
            self.ensure_nul();
        } else {
            self.len = new_len;
            self.ensure_nul();
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.ensure_nul();
    }

    pub fn make_room_for(&mut self, addlen: usize) -> &mut Self {
        if self.avail() >= addlen {
            return self;
        }

        let len = self.len;
        let mut newlen = len
            .checked_add(addlen)
            .expect("length overflow in Sds::make_room_for");
        if newlen < SDS_MAX_PREALLOC {
            newlen = newlen
                .checked_mul(2)
                .expect("allocation overflow in Sds::make_room_for");
        } else {
            newlen = newlen
                .checked_add(SDS_MAX_PREALLOC)
                .expect("allocation overflow in Sds::make_room_for");
        }

        if self.buf.len() < newlen + 1 {
            self.buf.resize(newlen + 1, 0);
        }
        self.len = len;
        self.ensure_nul();
        self
    }

    pub fn remove_free_space(&mut self) -> &mut Self {
        self.buf.truncate(self.len + 1);
        self.buf.shrink_to_fit();
        self.ensure_nul();
        self
    }

    pub fn alloc_size(&self) -> usize {
        Self::hdr_size(Self::req_type(self.alloc())) + self.alloc() + 1
    }

    pub fn alloc_ptr(&self) -> usize {
        self.buf.as_ptr() as usize
    }

    pub fn incr_len(&mut self, incr: isize) {
        if incr >= 0 {
            let add = incr as usize;
            assert!(self.avail() >= add);
            self.len += add;
        } else {
            let sub = (-incr) as usize;
            assert!(self.len >= sub);
            self.len -= sub;
        }
        self.ensure_nul();
    }

    pub fn growzero(&mut self, len: usize) -> &mut Self {
        let curlen = self.len;
        if len <= curlen {
            return self;
        }
        self.make_room_for(len - curlen);
        for i in curlen..len {
            self.buf[i] = 0;
        }
        self.len = len;
        self.ensure_nul();
        self
    }

    pub fn catlen(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let src = t.as_ref();
        let curlen = self.len;
        self.make_room_for(src.len());
        self.buf[curlen..curlen + src.len()].copy_from_slice(src);
        self.len = curlen + src.len();
        self.ensure_nul();
        self
    }

    pub fn cat(&mut self, t: impl AsRef<str>) -> &mut Self {
        self.catlen(t.as_ref().as_bytes())
    }

    pub fn catsds(&mut self, t: &Sds) -> &mut Self {
        self.catlen(t.as_bytes())
    }

    pub fn cpylen(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let src = t.as_ref();
        if self.alloc() < src.len() {
            let add = src.len().saturating_sub(self.len);
            self.make_room_for(add);
        }
        self.buf[..src.len()].copy_from_slice(src);
        self.len = src.len();
        self.ensure_nul();
        self
    }

    pub fn cpy(&mut self, t: impl AsRef<str>) -> &mut Self {
        self.cpylen(t.as_ref().as_bytes())
    }

    pub fn ll_2_str(value: i64) -> String {
        value.to_string()
    }

    pub fn ull_2_str(value: u64) -> String {
        value.to_string()
    }

    pub fn fromlonglong(value: i64) -> Self {
        Self::new(Self::ll_2_str(value))
    }

    pub fn catvprintf(&mut self, _fmt: &str, args: fmt::Arguments<'_>) -> &mut Self {
        let mut rendered = String::new();
        let _ = rendered.write_fmt(args);
        self.cat(rendered)
    }

    pub fn catprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        let mut rendered = String::new();
        let _ = rendered.write_fmt(args);
        self.cat(rendered)
    }

    pub fn catfmt(&mut self, _fmt: &str, args: fmt::Arguments<'_>) -> &mut Self {
        let mut rendered = String::new();
        let _ = rendered.write_fmt(args);
        self.cat(rendered)
    }

    pub fn trim(&mut self, cset: impl AsRef<[u8]>) -> &mut Self {
        let set = cset.as_ref();
        let mut start = 0usize;
        let mut end = self.len;

        while start < end && set.contains(&self.buf[start]) {
            start += 1;
        }
        while end > start && set.contains(&self.buf[end - 1]) {
            end -= 1;
        }

        if start > 0 && start < end {
            self.buf.copy_within(start..end, 0);
        }
        self.len = end.saturating_sub(start);
        self.ensure_nul();
        self
    }

    pub fn range(&mut self, start: isize, end: isize) -> &mut Self {
        let len = self.len as isize;
        if len == 0 {
            self.clear();
            return self;
        }

        let mut s = if start < 0 { len + start } else { start };
        let mut e = if end < 0 { len + end } else { end };

        if s < 0 {
            s = 0;
        }
        if e < 0 {
            e = 0;
        }
        if s >= len || s > e {
            self.clear();
            return self;
        }
        if e >= len {
            e = len - 1;
        }

        let newlen = (e - s + 1) as usize;
        let s_usize = s as usize;
        if s_usize > 0 {
            self.buf.copy_within(s_usize..s_usize + newlen, 0);
        }
        self.len = newlen;
        self.ensure_nul();
        self
    }

    pub fn tolower(&mut self) -> &mut Self {
        for b in &mut self.buf[..self.len] {
            *b = b.to_ascii_lowercase();
        }
        self
    }

    pub fn toupper(&mut self) -> &mut Self {
        for b in &mut self.buf[..self.len] {
            *b = b.to_ascii_uppercase();
        }
        self
    }

    pub fn cmp(&self, other: &Sds) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }

    pub fn splitlen(s: impl AsRef<[u8]>, sep: impl AsRef<[u8]>) -> Vec<Sds> {
        let input = s.as_ref();
        let sep = sep.as_ref();

        if sep.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::new();
        let mut start = 0usize;
        let mut i = 0usize;

        while i + sep.len() <= input.len() {
            if &input[i..i + sep.len()] == sep {
                out.push(Sds::newlen(&input[start..i]));
                i += sep.len();
                start = i;
            } else {
                i += 1;
            }
        }
        out.push(Sds::newlen(&input[start..]));
        out
    }

    pub fn freesplitres(tokens: Vec<Sds>) {
        drop(tokens);
    }

    pub fn catrepr(&mut self, p: impl AsRef<[u8]>) -> &mut Self {
        self.cat("\"");
        for &b in p.as_ref() {
            match b {
                b'\\' | b'"' => {
                    self.cat("\\");
                    self.catlen([b]);
                }
                b'\n' => {
                    self.cat("\\n");
                }
                b'\r' => {
                    self.cat("\\r");
                }
                b'\t' => {
                    self.cat("\\t");
                }
                7 => {
                    self.cat("\\a");
                }
                8 => {
                    self.cat("\\b");
                }
                b if (b as char).is_ascii() && !(b as char).is_ascii_control() => {
                    self.catlen([b]);
                }
                b => {
                    self.cat(format!("\\x{:02x}", b));
                }
            }
        }
        self.cat("\"");
        self
    }

    pub fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    pub fn hex_digit_to_int(c: char) -> i32 {
        match c.to_digit(16) {
            Some(v) => v as i32,
            None => 0,
        }
    }

    pub fn splitargs(line: &str) -> Vec<Sds> {
        let mut args = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0usize;

        while i < chars.len() {
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            if i >= chars.len() {
                break;
            }

            let mut current = String::new();
            let mut in_double = false;
            let mut in_single = false;

            while i < chars.len() {
                let ch = chars[i];

                if in_double {
                    match ch {
                        '"' => {
                            in_double = false;
                            i += 1;
                            break;
                        }
                        '\\' if i + 1 < chars.len() => {
                            let next = chars[i + 1];
                            match next {
                                'n' => current.push('\n'),
                                'r' => current.push('\r'),
                                't' => current.push('\t'),
                                'b' => current.push('\u{0008}'),
                                'a' => current.push('\u{0007}'),
                                'x' if i + 3 < chars.len()
                                    && Self::is_hex_digit(chars[i + 2])
                                    && Self::is_hex_digit(chars[i + 3]) =>
                                {
                                    let hi = Self::hex_digit_to_int(chars[i + 2]) as u8;
                                    let lo = Self::hex_digit_to_int(chars[i + 3]) as u8;
                                    current.push((hi * 16 + lo) as char);
                                    i += 4;
                                    continue;
                                }
                                other => current.push(other),
                            }
                            i += 2;
                            continue;
                        }
                        other => current.push(other),
                    }
                    i += 1;
                    continue;
                }

                if in_single {
                    if ch == '\'' {
                        in_single = false;
                        i += 1;
                        break;
                    }
                    current.push(ch);
                    i += 1;
                    continue;
                }

                match ch {
                    '"' => {
                        in_double = true;
                        i += 1;
                    }
                    '\'' => {
                        in_single = true;
                        i += 1;
                    }
                    c if c.is_whitespace() => break,
                    _ => {
                        current.push(ch);
                        i += 1;
                    }
                }
            }

            args.push(Sds::new(current));
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
        }

        args
    }

    pub fn mapchars(&mut self, from: &str, to: &str, setlen: usize) -> &mut Self {
        let from_b = from.as_bytes();
        let to_b = to.as_bytes();
        let n = setlen.min(from_b.len()).min(to_b.len());

        for b in &mut self.buf[..self.len] {
            for i in 0..n {
                if *b == from_b[i] {
                    *b = to_b[i];
                    break;
                }
            }
        }
        self
    }

    pub fn join(argv: &[&str], sep: &str) -> Self {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.cat(*item);
            if j != argv.len().saturating_sub(1) {
                join.cat(sep);
            }
        }
        join
    }

    pub fn joinsds(argv: &[Sds], sep: &str) -> Self {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.catsds(item);
            if j != argv.len().saturating_sub(1) {
                join.cat(sep);
            }
        }
        join
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn realloc(mut data: Vec<u8>, size: usize) -> Vec<u8> {
        data.resize(size, 0);
        data
    }

    pub fn test() -> i32 {
        struct Runner {
            num: usize,
            failed: usize,
        }

        impl Runner {
            fn new() -> Self {
                Self { num: 0, failed: 0 }
            }

            fn check(&mut self, descr: &str, ok: bool) {
                self.num += 1;
                if ok {
                    println!("{} - {}: PASSED", self.num, descr);
                } else {
                    println!("{} - {}: FAILED", self.num, descr);
                    self.failed += 1;
                }
            }
        }

        let mut t = Runner::new();

        t.check(
            "Create a string and obtain the length",
            {
                let x = Sds::new("foo");
                x.len == 3 && x.buf.get(..4) == Some(b"foo\0")
            },
        );

        t.check(
            "Create a string with specified length",
            {
                let x = Sds::newlen(&b"foo"[..2]);
                x.len == 2 && x.buf.get(..3) == Some(b"fo\0")
            },
        );

        t.check(
            "Strings concatenation",
            {
                let mut x = Sds::newlen(&b"foo"[..2]);
                x.cat("bar");
                x.len == 5 && x.buf.get(..6) == Some(b"fobar\0")
            },
        );

        t.check(
            "sdscpy() against an originally longer string",
            {
                let mut x = Sds::newlen(&b"foo"[..2]);
                x.cat("bar");
                x.cpy("a");
                x.len == 1 && x.buf.get(..2) == Some(b"a\0")
            },
        );

        t.check(
            "sdscpy() against an originally shorter string",
            {
                let mut x = Sds::new("a");
                x.cpy("xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk");
                x.len == 33
                    && x.buf.get(..34)
                        == Some(b"xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk\0")
            },
        );

        t.check(
            "sdscatprintf() seems working in the base case",
            {
                let mut x = Sds::empty();
                x.catprintf(format_args!("{}", 123));
                x.len == 3 && x.buf.get(..4) == Some(b"123\0")
            },
        );

        t.check(
            "sdscatprintf() seems working with \\0 inside of result",
            {
                let mut x = Sds::empty();
                x.catlen(b"a\0b");
                x.len == 3 && x.buf.get(..4) == Some(b"a\0b\0")
            },
        );

        t.check(
            "sdscatprintf() can print 1MB",
            {
                let mut x = Sds::empty();
                x.cat("0".repeat(1024 * 1024));
                x.len == 1024 * 1024 && x.as_bytes().iter().all(|&b| b == b'0')
            },
        );

        t.check(
            "sdscatfmt() seems working in the base case",
            {
                let mut x = Sds::new("--");
                x.cat("Hello Hi! World -9223372036854775808,9223372036854775807--");
                x.len == 60
                    && x.buf.get(..60)
                        == Some(b"--Hello Hi! World -9223372036854775808,9223372036854775807--")
            },
        );

        t.check(
            "sdscatfmt() seems working with unsigned numbers",
            {
                let mut x = Sds::new("--");
                x.cat("4294967295,18446744073709551615--");
                x.len == 35
                    && x.buf.get(..35) == Some(b"--4294967295,18446744073709551615--")
            },
        );

        t.check(
            "sdstrim() works when all chars match",
            {
                let mut x = Sds::new(" x ");
                x.trim(" x");
                x.len == 0
            },
        );

        t.check(
            "sdstrim() works when a single char remains",
            {
                let mut x = Sds::new(" x ");
                x.trim(" ");
                x.len == 1 && x.as_bytes() == b"x"
            },
        );

        let mut x = Sds::new("xxciaoyyy");
        x.trim("xy");
        t.check(
            "sdstrim() correctly trims characters",
            x.len == 4 && x.buf.get(..5) == Some(b"ciao\0"),
        );

        let mut y = x.dup();
        y.range(1, 1);
        t.check("sdsrange(...,1,1)", y.len == 1 && y.buf.get(..2) == Some(b"i\0"));

        let mut y = x.dup();
        y.range(1, -1);
        t.check("sdsrange(...,1,-1)", y.len == 3 && y.buf.get(..4) == Some(b"iao\0"));

        let mut y = x.dup();
        y.range(-2, -1);
        t.check("sdsrange(...,-2,-1)", y.len == 2 && y.buf.get(..3) == Some(b"ao\0"));

        let mut y = x.dup();
        y.range(2, 1);
        t.check("sdsrange(...,2,1)", y.len == 0 && y.buf.first() == Some(&0));

        let mut y = x.dup();
        y.range(1, 100);
        t.check("sdsrange(...,1,100)", y.len == 3 && y.buf.get(..4) == Some(b"iao\0"));

        let mut y = x.dup();
        y.range(100, 100);
        t.check("sdsrange(...,100,100)", y.len == 0 && y.buf.first() == Some(&0));

        t.check("sdscmp(foo,foa)", Sds::new("foo").cmp(&Sds::new("foa")) == Ordering::Greater);
        t.check("sdscmp(bar,bar)", Sds::new("bar").cmp(&Sds::new("bar")) == Ordering::Equal);
        t.check("sdscmp(bar,bar)", Sds::new("aar").cmp(&Sds::new("bar")) == Ordering::Less);

        t.check(
            "sdscatrepr(...data...)",
            {
                let x = Sds::newlen([7u8, b'\n', 0, b'f', b'o', b'o', b'\r']);
                let mut y = Sds::empty();
                y.catrepr(x.as_bytes());
                y.buf.get(..15) == Some(b"\"\\a\\n\\x00foo\\r\"")
            },
        );

        t.check(
            "sdsnew() free/len buffers",
            {
                let x = Sds::new("0");
                x.len == 1 && x.avail() == 0
            },
        );

        let mut x = Sds::new("0");
        let step = 10usize;
        for _ in 0..10 {
            let oldlen = x.len;
            x.make_room_for(step);
            t.check("sdsMakeRoomFor() len", x.len == oldlen);
            t.check("sdsMakeRoomFor() free", x.avail() >= step);
            let p = oldlen;
            for j in 0..step {
                x.buf[p + j] = b'A' + j as u8;
            }
            x.incr_len(step as isize);
        }
        t.check(
            "sdsMakeRoomFor() content",
            x.as_bytes()
                == b"0ABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJABCDEFGHIJ",
        );
        t.check("sdsMakeRoomFor() final length", x.len == 101);

        let passed = t.num - t.failed;
        println!("{} tests, {} passed, {} failed", t.num, passed, t.failed);

        if t.failed == 0 { 0 } else { 1 }
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

    pub fn s_realloc(data: Vec<u8>, size: usize) -> Vec<u8> {
        Self::realloc(data, size)
    }

    pub fn s_free<T>(_value: T) {}

    pub fn test_cond(condition: bool, _message: &str) -> bool {
        condition
    }

    pub fn test_report() -> i32 {
        0
    }
}

impl Drop for Sds {
    fn drop(&mut self) {}
}
