use crate::localcharset::Localcharset;
use std::cell::RefCell;

const QA_ELIDE_NULL_BYTES: u32 = 1 << 0;
const QA_ELIDE_OUTER_QUOTES: u32 = 1 << 1;
const QA_SPLIT_TRIGRAPHS: u32 = 1 << 2;

const INT_BITS: usize = u32::BITS as usize;
const QUOTE_BITMAP_WORDS: usize = 256 / INT_BITS;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
    C,
    CMaybe,
    Escape,
    Locale,
    Clocale,
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: u32,
    pub quote_these_too: [u32; QUOTE_BITMAP_WORDS],
    pub left_quote: Option<String>,
    pub right_quote: Option<String>,
}

impl Default for QuotingOptions {
    fn default() -> Self {
        Self {
            style: QuotingStyle::Literal,
            flags: 0,
            quote_these_too: [0; QUOTE_BITMAP_WORDS],
            left_quote: None,
            right_quote: None,
        }
    }
}

#[derive(Default)]
struct SlotState {
    slots: Vec<String>,
}

thread_local! {
    static DEFAULT_OPTIONS: RefCell<QuotingOptions> = RefCell::new(QuotingOptions::default());
    static QUOTE_OPTIONS_01: RefCell<QuotingOptions> = RefCell::new(QuotingOptions::default());
    static QUOTE_OPTIONS_02: RefCell<QuotingOptions> = RefCell::new(QuotingOptions::default());
    static SLOT_STATE: RefCell<SlotState> = RefCell::new(SlotState::default());
}

fn quote_bit_enabled(bits: &[u32; QUOTE_BITMAP_WORDS], byte: u8) -> bool {
    let idx = byte as usize / INT_BITS;
    let shift = byte as usize % INT_BITS;
    ((bits[idx] >> shift) & 1) != 0
}

fn set_quote_bit(bits: &mut [u32; QUOTE_BITMAP_WORDS], byte: u8, value: bool) -> bool {
    let idx = byte as usize / INT_BITS;
    let shift = byte as usize % INT_BITS;
    let old = ((bits[idx] >> shift) & 1) != 0;
    let mask = 1u32 << shift;
    if value {
        bits[idx] |= mask;
    } else {
        bits[idx] &= !mask;
    }
    old
}

fn ascii_printable(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

fn escape_octal(byte: u8, out: &mut String) {
    out.push('\\');
    out.push(char::from(b'0' + ((byte >> 6) & 0x07)));
    out.push(char::from(b'0' + ((byte >> 3) & 0x07)));
    out.push(char::from(b'0' + (byte & 0x07)));
}

fn escape_c(byte: u8, out: &mut String) {
    match byte {
        b'\x07' => out.push_str("\\a"),
        b'\x08' => out.push_str("\\b"),
        b'\x0c' => out.push_str("\\f"),
        b'\n' => out.push_str("\\n"),
        b'\r' => out.push_str("\\r"),
        b'\t' => out.push_str("\\t"),
        b'\x0b' => out.push_str("\\v"),
        b'\\' => out.push_str("\\\\"),
        b'"' => out.push_str("\\\""),
        b'\'' => out.push_str("\\'"),
        0 => out.push_str("\\0"),
        b if ascii_printable(b) => out.push(char::from(b)),
        b => escape_octal(b, out),
    }
}

fn shell_needs_quotes(bytes: &[u8], quote_bits: &[u32; QUOTE_BITMAP_WORDS]) -> bool {
    if bytes.is_empty() {
        return true;
    }
    for &b in bytes {
        if quote_bit_enabled(quote_bits, b) {
            return true;
        }
        match b {
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'_'
            | b'+'
            | b','
            | b'.'
            | b'/'
            | b':'
            | b'-' => {}
            _ => return true,
        }
    }
    false
}

fn shell_single_quote(bytes: &[u8]) -> String {
    let mut out = String::new();
    out.push('\'');
    for &b in bytes {
        if b == b'\'' {
            out.push_str("'\\''");
        } else if b.is_ascii() {
            out.push(char::from(b));
        } else {
            out.push_str(String::from_utf8_lossy(&[b]).as_ref());
        }
    }
    out.push('\'');
    out
}

fn shell_escape_style(bytes: &[u8], always_outer: bool, quote_bits: &[u32; QUOTE_BITMAP_WORDS]) -> String {
    let needs_outer = always_outer || shell_needs_quotes(bytes, quote_bits);
    if !needs_outer {
        let mut out = String::with_capacity(bytes.len());
        for &b in bytes {
            out.push(char::from(b));
        }
        return out;
    }

    let mut out = String::new();
    out.push('\'');
    for &b in bytes {
        if quote_bit_enabled(quote_bits, b) {
            if b == b'\'' {
                out.push_str("'\\''");
            } else if b.is_ascii_graphic() || b == b' ' {
                out.push(char::from(b));
            } else {
                out.push_str("'$'");
                escape_c(b, &mut out);
                out.push('\'');
            }
            continue;
        }

        match b {
            b'\'' => out.push_str("'\\''"),
            b'\n' | b'\r' | b'\t' | b'\x07' | b'\x08' | b'\x0c' | b'\x0b' | 0 => {
                out.push_str("'$'");
                escape_c(b, &mut out);
                out.push('\'');
            }
            _ if b.is_ascii() => out.push(char::from(b)),
            _ => out.push_str(String::from_utf8_lossy(&[b]).as_ref()),
        }
    }
    out.push('\'');
    out
}

fn quote_pair_for_style(style: QuotingStyle, options: &QuotingOptions) -> Option<(String, String)> {
    match style {
        QuotingStyle::C | QuotingStyle::CMaybe => Some(("\"".to_string(), "\"".to_string())),
        QuotingStyle::Locale | QuotingStyle::Clocale => Some((
            Quotearg::gettext_quote("`", style).to_string(),
            Quotearg::gettext_quote("'", style).to_string(),
        )),
        QuotingStyle::Custom => Some((
            options
                .left_quote
                .clone()
                .expect("custom quoting requires a left quote"),
            options
                .right_quote
                .clone()
                .expect("custom quoting requires a right quote"),
        )),
        _ => None,
    }
}

fn quote_bytes(bytes: &[u8], options: &QuotingOptions, elide_null: bool) -> String {
    let mut filtered = Vec::with_capacity(bytes.len());
    if elide_null {
        filtered.extend(bytes.iter().copied().filter(|b| *b != 0));
    } else {
        filtered.extend_from_slice(bytes);
    }

    match options.style {
        QuotingStyle::Literal => {
            let mut out = String::new();
            for &b in &filtered {
                if b.is_ascii() {
                    out.push(char::from(b));
                } else {
                    escape_octal(b, &mut out);
                }
            }
            out
        }
        QuotingStyle::Escape => {
            let mut out = String::new();
            for &b in &filtered {
                if quote_bit_enabled(&options.quote_these_too, b) {
                    escape_c(b, &mut out);
                } else {
                    escape_c(b, &mut out);
                }
            }
            out
        }
        QuotingStyle::C | QuotingStyle::CMaybe | QuotingStyle::Locale | QuotingStyle::Clocale | QuotingStyle::Custom => {
            let (left, right) = quote_pair_for_style(options.style, options).expect("quoted style requires delimiters");
            let mut out = String::new();
            let elide_outer = options.flags & QA_ELIDE_OUTER_QUOTES != 0;
            if !elide_outer {
                out.push_str(&left);
            }
            for &b in &filtered {
                let must_quote = quote_bit_enabled(&options.quote_these_too, b);
                if must_quote {
                    escape_c(b, &mut out);
                    continue;
                }

                if right.len() == 1 && b == right.as_bytes()[0] {
                    out.push('\\');
                    out.push(char::from(b));
                    continue;
                }

                match b {
                    b if b.is_ascii() && ascii_printable(b) && b != b'\\' => out.push(char::from(b)),
                    b => escape_c(b, &mut out),
                }
            }
            if !elide_outer {
                out.push_str(&right);
            }
            out
        }
        QuotingStyle::Shell => {
            if shell_needs_quotes(&filtered, &options.quote_these_too) {
                shell_single_quote(&filtered)
            } else {
                let mut out = String::new();
                for &b in &filtered {
                    out.push(char::from(b));
                }
                out
            }
        }
        QuotingStyle::ShellAlways => shell_single_quote(&filtered),
        QuotingStyle::ShellEscape => shell_escape_style(&filtered, false, &options.quote_these_too),
        QuotingStyle::ShellEscapeAlways => shell_escape_style(&filtered, true, &options.quote_these_too),
    }
}

pub struct Quotearg {
    owned: Vec<String>,
}

impl Quotearg {
    pub fn new() -> Self {
        Self { owned: Vec::new() }
    }

    pub fn clone_quoting_options(options: Option<&QuotingOptions>) -> QuotingOptions {
        match options {
            Some(o) => o.clone(),
            None => DEFAULT_OPTIONS.with(|o| o.borrow().clone()),
        }
    }

    pub fn get_quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        match options {
            Some(o) => o.style,
            None => DEFAULT_OPTIONS.with(|o| o.borrow().style),
        }
    }

    pub fn set_quoting_style(options: Option<&mut QuotingOptions>, style: QuotingStyle) {
        match options {
            Some(o) => o.style = style,
            None => {
                DEFAULT_OPTIONS.with(|o| o.borrow_mut().style = style);
            }
        }
    }

    pub fn set_char_quoting(options: Option<&mut QuotingOptions>, c: char, enabled: bool) -> bool {
        let byte = c as u32;
        let byte = u8::try_from(byte).expect("set_char_quoting only supports byte-sized characters");
        match options {
            Some(o) => set_quote_bit(&mut o.quote_these_too, byte, enabled),
            None => {
                DEFAULT_OPTIONS.with(|o| {
                    let mut guard = o.borrow_mut();
                    set_quote_bit(&mut guard.quote_these_too, byte, enabled)
                })
            }
        }
    }

    pub fn set_quoting_flags(options: Option<&mut QuotingOptions>, flags: u32) -> u32 {
        match options {
            Some(o) => {
                let old = o.flags;
                o.flags = flags;
                old
            }
            None => {
                DEFAULT_OPTIONS.with(|o| {
                    let mut guard = o.borrow_mut();
                    let old = guard.flags;
                    guard.flags = flags;
                    old
                })
            }
        }
    }

    pub fn set_custom_quoting(
        options: Option<&mut QuotingOptions>,
        left_quote: impl Into<String>,
        right_quote: impl Into<String>,
    ) {
        let left = left_quote.into();
        let right = right_quote.into();
        assert!(
            !left.is_empty() && !right.is_empty(),
            "custom quoting requires non-empty delimiters"
        );
        match options {
            Some(o) => {
                o.style = QuotingStyle::Custom;
                o.left_quote = Some(left);
                o.right_quote = Some(right);
            }
            None => {
                DEFAULT_OPTIONS.with(|o| {
                    let mut guard = o.borrow_mut();
                    guard.style = QuotingStyle::Custom;
                    guard.left_quote = Some(left);
                    guard.right_quote = Some(right);
                });
            }
        }
    }

    pub fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions {
        assert!(style != QuotingStyle::Custom, "custom style requires explicit delimiters");
        QuotingOptions {
            style,
            ..QuotingOptions::default()
        }
    }

    pub fn gettext_quote(msgid: &str, style: QuotingStyle) -> &'static str {
        let charset = Localcharset::locale_charset();
        if charset.eq_ignore_ascii_case("UTF-8") {
            return if msgid.starts_with('`') { "\u{2018}" } else { "\u{2019}" };
        }
        if charset.eq_ignore_ascii_case("GB18030") {
            return if msgid.starts_with('`') { "\u{00A1}\u{00AE}" } else { "\u{00A1}\u{00AF}" };
        }
        if style == QuotingStyle::Clocale {
            "\""
        } else {
            "'"
        }
    }

    pub fn buffer_restyled(
        buffer: &mut [u8],
        arg: &[u8],
        style: QuotingStyle,
        flags: u32,
        quote_these_too: &[u32; QUOTE_BITMAP_WORDS],
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> usize {
        let options = QuotingOptions {
            style,
            flags,
            quote_these_too: *quote_these_too,
            left_quote: left_quote.map(str::to_string),
            right_quote: right_quote.map(str::to_string),
        };
        let rendered = quote_bytes(arg, &options, flags & QA_ELIDE_NULL_BYTES != 0);
        let bytes = rendered.as_bytes();
        let n = buffer.len().min(bytes.len());
        buffer[..n].copy_from_slice(&bytes[..n]);
        if n < buffer.len() {
            buffer[n] = 0;
        }
        bytes.len()
    }

    pub fn buffer(buffer: &mut [u8], arg: &[u8], options: Option<&QuotingOptions>) -> usize {
        let owned_default;
        let opts = match options {
            Some(o) => o,
            None => {
                owned_default = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
                &owned_default
            }
        };
        Self::buffer_restyled(
            buffer,
            arg,
            opts.style,
            opts.flags,
            &opts.quote_these_too,
            opts.left_quote.as_deref(),
            opts.right_quote.as_deref(),
        )
    }

    pub fn alloc_mem(arg: &[u8], size: Option<&mut usize>, options: Option<&QuotingOptions>) -> String {
        let owned_default;
        let opts = match options {
            Some(o) => o,
            None => {
                owned_default = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
                &owned_default
            }
        };
        let rendered = quote_bytes(arg, opts, size.is_none() || (opts.flags & QA_ELIDE_NULL_BYTES != 0));
        if let Some(out_size) = size {
            *out_size = rendered.len();
        }
        rendered
    }

    pub fn n_options(n: usize, arg: &[u8], options: &QuotingOptions) -> String {
        let rendered = quote_bytes(arg, options, true);
        SLOT_STATE.with(|s| {
            let mut slots = s.borrow_mut();
            if slots.slots.len() <= n {
                slots.slots.resize(n + 1, String::new());
            }
            slots.slots[n] = rendered.clone();
        });
        rendered
    }

    pub fn n(&self, n: usize, arg: &str) -> String {
        let options = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
        Self::n_options(n, arg.as_bytes(), &options)
    }

    pub fn n_mem(&self, n: usize, arg: &[u8]) -> String {
        let options = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
        Self::n_options(n, arg, &options)
    }

    pub fn quotearg(&self, arg: &str) -> String {
        self.n(0, arg)
    }

    pub fn mem(&self, arg: &[u8]) -> String {
        self.n_mem(0, arg)
    }

    pub fn n_style(&self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let options = Self::quoting_options_from_style(style);
        Self::n_options(n, arg.as_bytes(), &options)
    }

    pub fn n_style_mem(&self, n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let options = Self::quoting_options_from_style(style);
        Self::n_options(n, arg, &options)
    }

    pub fn style(&self, style: QuotingStyle, arg: &str) -> String {
        self.n_style(0, style, arg)
    }

    pub fn style_mem(&self, style: QuotingStyle, arg: &[u8]) -> String {
        self.n_style_mem(0, style, arg)
    }

    pub fn char_mem(&self, arg: &[u8], ch: char) -> String {
        let mut options = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
        Self::set_char_quoting(Some(&mut options), ch, true);
        Self::n_options(0, arg, &options)
    }

    pub fn char(&self, arg: &str, ch: char) -> String {
        self.char_mem(arg.as_bytes(), ch)
    }

    pub fn colon(&self, arg: &str) -> String {
        self.char(arg, ':')
    }

    pub fn colon_mem(&self, arg: &[u8]) -> String {
        self.char_mem(arg, ':')
    }

    pub fn n_style_colon(&self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let mut options = Self::quoting_options_from_style(style);
        Self::set_char_quoting(Some(&mut options), ':', true);
        Self::n_options(n, arg.as_bytes(), &options)
    }

    pub fn n_custom(
        &self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &str,
    ) -> String {
        self.n_custom_mem(n, left_quote, right_quote, arg.as_bytes())
    }

    pub fn n_custom_mem(
        &self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
    ) -> String {
        let mut options = DEFAULT_OPTIONS.with(|o| o.borrow().clone());
        Self::set_custom_quoting(Some(&mut options), left_quote, right_quote);
        Self::n_options(n, arg, &options)
    }

    pub fn custom(&self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(&self, left_quote: &str, right_quote: &str, arg: &[u8]) -> String {
        self.n_custom_mem(0, left_quote, right_quote, arg)
    }

    pub fn quote_n_mem(&self, n: usize, arg: &[u8]) -> String {
        let options = QUOTE_OPTIONS_01.with(|o| o.borrow().clone());
        Self::n_options(n, arg, &options)
    }

    pub fn quote_mem(&self, arg: &[u8]) -> String {
        self.quote_n_mem(0, arg)
    }

    pub fn quote_n(&self, n: usize, arg: &str) -> String {
        self.quote_n_mem(n, arg.as_bytes())
    }

    pub fn quote(&self, arg: &str) -> String {
        self.quote_n(0, arg)
    }

    pub fn custom_13(&self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.custom(left_quote, right_quote, arg)
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        QUOTE_OPTIONS_02.with(|o| o.borrow().clone())
    }

    pub fn quoting_options() -> QuotingOptions {
        DEFAULT_OPTIONS.with(|o| o.borrow().clone())
    }

    pub fn style_14(&self, style: QuotingStyle, arg: &str) -> String {
        self.style(style, arg)
    }

    pub fn quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        Self::get_quoting_style(options)
    }

    pub fn push_str(&mut self, value: impl Into<String>) {
        self.owned.push(value.into());
    }

    pub fn n_08(&self, n: usize, arg: &str) -> String {
        self.n(n, arg)
    }

    pub fn colon_12(&self, arg: &str) -> String {
        self.colon(arg)
    }

    pub fn main_root_quote_n_11(&self, n: usize, arg: &str) -> String {
        self.quote_n(n, arg)
    }

    pub fn main_root_quoting_options_01() -> QuotingOptions {
        QUOTE_OPTIONS_01.with(|o| o.borrow().clone())
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        self.owned.clear();
        SLOT_STATE.with(|s| s.borrow_mut().slots.clear());
    }
}
