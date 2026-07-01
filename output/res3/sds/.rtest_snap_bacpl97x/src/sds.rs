use std::cmp::Ordering;
use std::fmt::{self, Write};

const SDS_MAX_PREALLOC: usize = 1024 * 1024;
const SDS_LLSTR_SIZE: usize = 21;
const SDS_TYPE_5: u8 = 0;
const SDS_TYPE_8: u8 = 1;
const SDS_TYPE_16: u8 = 2;
const SDS_TYPE_32: u8 = 3;
const SDS_TYPE_64: u8 = 4;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Sds {
    buf: Vec<u8>,
    len: usize,
}

impl Sds {
    pub fn hdr_size(ty: u8) -> usize {
        match ty & 7 {
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
        } else if usize::BITS >= 64 && string_size < (1usize << 32) {
            SDS_TYPE_32
        } else if usize::BITS >= 64 {
            SDS_TYPE_64
        } else {
            SDS_TYPE_32
        }
    }

    pub fn newlen(init: impl AsRef<[u8]>) -> Self {
        let bytes = init.as_ref();
        let mut buf = Vec::with_capacity(bytes.len() + 1);
        buf.extend_from_slice(bytes);
        buf.push(0);
        Self {
            buf,
            len: bytes.len(),
        }
    }

    pub fn empty() -> Self {
        let mut s = Self::newlen([]);
        if Self::req_type(0) == SDS_TYPE_5 {
            s.make_room_for(0);
        }
        s
    }

    pub fn new(init: &str) -> Self {
        Self::newlen(init.as_bytes())
    }

    pub fn dup(&self) -> Self {
        Self::newlen(self.as_bytes())
    }

    pub fn updatelen(&mut self) {
        self.len = self
            .buf
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.buf.len());
        if self.buf.len() == self.len {
            self.buf.push(0);
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        if self.buf.is_empty() {
            self.buf.push(0);
        } else {
            self.buf[0] = 0;
            self.buf.truncate(1);
        }
    }

    pub fn make_room_for(&mut self, addlen: usize) -> &mut Self {
        let avail = self.avail();
        if avail >= addlen {
            return self;
        }

        let len = self.len;
        let mut newlen = len + addlen;
        if newlen < SDS_MAX_PREALLOC {
            newlen *= 2;
        } else {
            newlen += SDS_MAX_PREALLOC;
        }

        let target_capacity = newlen + 1;
        if self.buf.capacity() < target_capacity {
            self.buf.reserve(target_capacity - self.buf.capacity());
        }
        if self.buf.len() < self.len + 1 {
            self.buf.resize(self.len + 1, 0);
        }
        self
    }

    pub fn remove_free_space(&mut self) -> &mut Self {
        if self.avail() == 0 {
            return self;
        }
        self.buf.truncate(self.len + 1);
        self.buf.shrink_to_fit();
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
            let inc = incr as usize;
            assert!(self.avail() >= inc);
            let new_len = self.len + inc;
            if self.buf.len() < new_len + 1 {
                self.buf.resize(new_len + 1, 0);
            }
            self.len = new_len;
            self.buf[self.len] = 0;
        } else {
            let dec = (-incr) as usize;
            assert!(self.len >= dec);
            self.len -= dec;
            if self.buf.len() < self.len + 1 {
                self.buf.resize(self.len + 1, 0);
            }
            self.buf[self.len] = 0;
            self.buf.truncate(self.len + 1);
        }
    }

    pub fn growzero(&mut self, len: usize) -> &mut Self {
        let curlen = self.len;
        if len <= curlen {
            return self;
        }
        self.make_room_for(len - curlen);
        if self.buf.len() < len + 1 {
            self.buf.resize(len + 1, 0);
        } else {
            for b in &mut self.buf[curlen..=len] {
                *b = 0;
            }
        }
        self.len = len;
        self
    }

    pub fn catlen(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let t = t.as_ref();
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
        self.catlen(t.as_bytes())
    }

    pub fn cpylen(&mut self, t: impl AsRef<[u8]>) -> &mut Self {
        let t = t.as_ref();
        if self.alloc() < t.len() {
            let needed = t.len().saturating_sub(self.len);
            self.make_room_for(needed);
        }
        if self.buf.len() < t.len() + 1 {
            self.buf.resize(t.len() + 1, 0);
        }
        self.buf[..t.len()].copy_from_slice(t);
        self.len = t.len();
        self.buf[self.len] = 0;
        self.buf.truncate(self.len + 1);
        self
    }

    pub fn cpy(&mut self, t: &str) -> &mut Self {
        self.cpylen(t.as_bytes())
    }

    pub fn ll_2_str(value: i64) -> String {
        let mut s = [0u8; SDS_LLSTR_SIZE];
        let len = if value < 0 {
            let mut v: u128 = if value == i64::MIN {
                (i64::MAX as u128) + 1
            } else {
                (-value) as u128
            };
            let mut p = 0usize;
            loop {
                s[p] = b'0' + (v % 10) as u8;
                p += 1;
                v /= 10;
                if v == 0 {
                    break;
                }
            }
            s[p] = b'-';
            p += 1;
            s[..p].reverse();
            p
        } else {
            let mut v = value as u128;
            let mut p = 0usize;
            loop {
                s[p] = b'0' + (v % 10) as u8;
                p += 1;
                v /= 10;
                if v == 0 {
                    break;
                }
            }
            s[..p].reverse();
            p
        };
        String::from_utf8_lossy(&s[..len]).into_owned()
    }

    pub fn ull_2_str(value: u64) -> String {
        let mut s = [0u8; SDS_LLSTR_SIZE];
        let mut v = value;
        let mut p = 0usize;
        loop {
            s[p] = b'0' + (v % 10) as u8;
            p += 1;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        s[..p].reverse();
        String::from_utf8_lossy(&s[..p]).into_owned()
    }

    pub fn fromlonglong(value: i64) -> Self {
        Self::newlen(Self::ll_2_str(value).into_bytes())
    }

    pub fn catvprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        let mut rendered = String::new();
        let _ = rendered.write_fmt(args);
        self.catlen(rendered.as_bytes())
    }

    pub fn catprintf(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        self.catvprintf(args)
    }

    pub fn catfmt(&mut self, args: fmt::Arguments<'_>) -> &mut Self {
        self.catvprintf(args)
    }

    pub fn trim(&mut self, cset: &str) -> &mut Self {
        let set = cset.as_bytes();
        if self.len == 0 {
            return self;
        }
        let mut sp = 0usize;
        let mut ep = self.len.saturating_sub(1);
        while sp < self.len && set.contains(&self.buf[sp]) {
            sp += 1;
        }
        while ep > sp && set.contains(&self.buf[ep]) {
            ep -= 1;
        }
        let newlen = if sp >= self.len { 0 } else { ep - sp + 1 };
        if sp != 0 && newlen != 0 {
            self.buf.copy_within(sp..sp + newlen, 0);
        }
        self.len = newlen;
        if self.buf.len() < self.len + 1 {
            self.buf.resize(self.len + 1, 0);
        }
        self.buf[self.len] = 0;
        self.buf.truncate(self.len + 1);
        self
    }

    pub fn range(&mut self, mut start: isize, mut end: isize) {
        let len = self.len as isize;
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

        let mut newlen = if start > end {
            0
        } else {
            (end - start + 1) as usize
        };

        if newlen != 0 {
            if start >= len {
                newlen = 0;
            } else if end >= len {
                end = len - 1;
                newlen = (end - start + 1) as usize;
            }
        }

        if start > 0 && newlen > 0 {
            self.buf
                .copy_within(start as usize..start as usize + newlen, 0);
        }
        self.len = newlen;
        if self.buf.len() < self.len + 1 {
            self.buf.resize(self.len + 1, 0);
        }
        self.buf[self.len] = 0;
        self.buf.truncate(self.len + 1);
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
        let mut out = Vec::new();
        let mut start = 0usize;
        let mut j = 0usize;
        while j + sep.len() <= s.len() {
            let matched = if sep.len() == 1 {
                s[j] == sep[0]
            } else {
                &s[j..j + sep.len()] == sep
            };
            if matched {
                out.push(Sds::newlen(&s[start..j]));
                start = j + sep.len();
                j += sep.len();
            } else {
                j += 1;
            }
        }
        out.push(Sds::newlen(&s[start..]));
        out
    }

    pub fn freesplitres(tokens: &mut Vec<Sds>) {
        tokens.clear();
    }

    pub fn catrepr(&mut self, p: &[u8]) -> &mut Self {
        self.catlen(b"\"");
        for &b in p {
            match b {
                b'\\' | b'"' => {
                    self.catprintf(format_args!("\\{}", b as char));
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
                    if (b as char).is_ascii_graphic() || b == b' ' {
                        self.catprintf(format_args!("{}", b as char));
                    } else {
                        self.catprintf(format_args!("\\x{:02x}", b));
                    }
                }
            }
        }
        self.catlen(b"\"")
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
        let mut current: Option<Sds> = None;
        let mut vector: Vec<Sds> = Vec::new();

        loop {
            while p < bytes.len() && bytes[p].is_ascii_whitespace() {
                p += 1;
            }
            if p >= bytes.len() {
                return Some(vector);
            }

            let mut inq = false;
            let mut insq = false;
            let mut done = false;
            if current.is_none() {
                current = Some(Sds::empty());
            }

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
                        current.as_mut().unwrap().catlen([byte]);
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
                        current.as_mut().unwrap().catlen([c]);
                    } else if ch == b'"' {
                        if p + 1 < bytes.len() && !bytes[p + 1].is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.as_mut().unwrap().catlen([ch]);
                    }
                } else if insq {
                    if ch == b'\\' && p + 1 < bytes.len() && bytes[p + 1] == b'\'' {
                        p += 1;
                        current.as_mut().unwrap().catlen(b"'");
                    } else if ch == b'\'' {
                        if p + 1 < bytes.len() && !bytes[p + 1].is_ascii_whitespace() {
                            return None;
                        }
                        done = true;
                    } else if ch == 0 {
                        return None;
                    } else {
                        current.as_mut().unwrap().catlen([ch]);
                    }
                } else {
                    match ch {
                        b' ' | b'\n' | b'\r' | b'\t' | 0 => done = true,
                        b'"' => inq = true,
                        b'\'' => insq = true,
                        _ => {
                            current.as_mut().unwrap().catlen([ch]);
                        }
                    }
                }
                if p < bytes.len() {
                    p += 1;
                }
                if p >= bytes.len() && !done && !inq && !insq {
                    done = true;
                }
            }

            vector.push(current.take().unwrap());
        }
    }

    pub fn mapchars(&mut self, from: &str, to: &str, setlen: usize) -> &mut Self {
        let fromb = from.as_bytes();
        let tob = to.as_bytes();
        let n = setlen.min(fromb.len()).min(tob.len());
        for j in 0..self.len {
            for i in 0..n {
                if self.buf[j] == fromb[i] {
                    self.buf[j] = tob[i];
                    break;
                }
            }
        }
        self
    }

    pub fn join(argv: &[&str], sep: &str) -> Self {
        let mut join = Sds::empty();
        for (j, item) in argv.iter().enumerate() {
            join.cat(item);
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
                join.catlen(sep.as_bytes());
            }
        }
        join
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    pub fn realloc(mut ptr: Vec<u8>, size: usize) -> Vec<u8> {
        ptr.resize(size, 0);
        ptr
    }

    pub fn test() -> i32 {
        let mut passed = 0usize;
        let mut failed = 0usize;

        let mut x = Sds::new("foo");
        Sds::test_cond(
            "Create a string and obtain the length",
            x.len() == 3 && x.as_slice_with_nul() == b"foo\0",
            &mut passed,
            &mut failed,
        );

        x = Sds::newlen(&b"foo"[..2]);
        Sds::test_cond(
            "Create a string with specified length",
            x.len() == 2 && x.as_slice_with_nul() == b"fo\0",
            &mut passed,
            &mut failed,
        );

        x.cat("bar");
        Sds::test_cond(
            "Strings concatenation",
            x.len() == 5 && x.as_slice_with_nul() == b"fobar\0",
            &mut passed,
            &mut failed,
        );

        x.cpy("a");
        Sds::test_cond(
            "sdscpy() against an originally longer string",
            x.len() == 1 && x.as_slice_with_nul() == b"a\0",
            &mut passed,
            &mut failed,
        );

        x.cpy("xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk");
        Sds::test_cond(
            "sdscpy() against an originally shorter string",
            x.len() == 33 && x.as_bytes() == b"xyzxxxxxxxxxxyyyyyyyyyykkkkkkkkkk",
            &mut passed,
            &mut failed,
        );

        x = Sds::empty();
        x.catprintf(format_args!("{}", 123));
        Sds::test_cond(
            "sdscatprintf() seems working in the base case",
            x.len() == 3 && x.as_slice_with_nul() == b"123\0",
            &mut passed,
            &mut failed,
        );

        x = Sds::empty();
        x.catlen(b"a").catlen([0]).catlen(b"b");
        Sds::test_cond(
            "sdscatprintf() seems working with \\0 inside of result",
            x.len() == 3 && x.as_slice_with_nul() == b"a\0b\0",
            &mut passed,
            &mut failed,
        );

        let mut y = Sds::empty();
        y.catrepr(b"\x07\n\0foo\r");
        Sds::test_cond(
            "sdscatrepr(...data...)",
            y.as_bytes() == b"\"\\a\\n\\x00foo\\r\"",
            &mut passed,
            &mut failed,
        );

        Sds::test_report(passed, failed);
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

    pub fn s_realloc(ptr: Vec<u8>, size: usize) -> Vec<u8> {
        Self::realloc(ptr, size)
    }

    pub fn s_free<T>(_ptr: T) {}

    pub fn test_cond(descr: &str, condition: bool, passed: &mut usize, failed: &mut usize) {
        let total = *passed + *failed + 1;
        print!("{total} - {descr}: ");
        if condition {
            println!("PASSED");
            *passed += 1;
        } else {
            println!("FAILED");
            *failed += 1;
        }
    }

    pub fn test_report(passed: usize, failed: usize) {
        let total = passed + failed;
        println!("{total} tests, {passed} passed, {failed} failed");
        if failed != 0 {
            println!("=== WARNING === We have failed tests here...");
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn as_slice_with_nul(&self) -> &[u8] {
        &self.buf[..self.len + 1]
    }

    fn alloc(&self) -> usize {
        self.buf.capacity().saturating_sub(1)
    }

    fn avail(&self) -> usize {
        self.alloc().saturating_sub(self.len)
    }
}

impl Drop for Sds {
    fn drop(&mut self) {}
}
