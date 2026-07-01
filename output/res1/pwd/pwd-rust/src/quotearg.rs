thread_local! {
    static DEFAULT_QUOTING_OPTIONS: RefCell<QuotingOptions> = RefCell::new(QuotingOptions::default());
    static SLOT_STORE: RefCell<Vec<String>> = RefCell::new(Vec::new());
}
use crate::localcharset::Localcharset;
use std::collections::BTreeSet;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: i32,
    pub quote_these_too: [u32; 8],
    pub left_quote: Option<String>,
    pub right_quote: Option<String>,
}

impl Default for QuotingOptions {
    fn default() -> Self {
        Self {
            style: QuotingStyle::Literal,
            flags: 0,
            quote_these_too: [0; 8],
            left_quote: None,
            right_quote: None,
        }
    }
}


fn quote_quoting_options_cell() -> &'static QuotingOptions {
    static CELL: OnceLock<QuotingOptions> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut options = QuotingOptions {
            style: QuotingStyle::Locale,
            ..QuotingOptions::default()
        };
        let _ = set_char_bit(&mut options, b':' as char, true);
        options
    })
}


fn normalize_c_string_len(input: &str, explicit: Option<usize>) -> &[u8] {
    let bytes = input.as_bytes();
    match explicit {
        Some(n) => &bytes[..n.min(bytes.len())],
        None => match bytes.iter().position(|&b| b == 0) {
            Some(end) => &bytes[..end],
            None => bytes,
        },
    }
}

fn bytes_to_display(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b'\0' => out.push_str("\\0"),
            0x20..=0x7e => out.push(b as char),
            _ => {
                let _ = std::fmt::Write::write_fmt(&mut out, format_args!("\\x{:02x}", b));
            }
        }
    }
    out
}

fn char_is_forced(options: &QuotingOptions, byte: u8) -> bool {
    let idx = (byte as usize) / 32;
    let shift = (byte as usize) % 32;
    ((options.quote_these_too[idx] >> shift) & 1) != 0
}

fn set_char_bit(options: &mut QuotingOptions, c: char, enabled: bool) -> bool {
    let uc = (c as u32 & 0xff) as usize;
    let idx = uc / 32;
    let shift = uc % 32;
    let previous = ((options.quote_these_too[idx] >> shift) & 1) != 0;
    if previous != enabled {
        options.quote_these_too[idx] ^= 1u32 << shift;
    }
    previous
}

fn quote_delimiters(options: &QuotingOptions) -> (String, String) {
    match options.style {
        QuotingStyle::Literal => (String::new(), String::new()),
        QuotingStyle::Custom => (
            options.left_quote.clone().unwrap_or_default(),
            options.right_quote.clone().unwrap_or_default(),
        ),
        QuotingStyle::Locale | QuotingStyle::Clocale => (
            Quotearg::gettext_quote("`", options.style).to_string(),
            Quotearg::gettext_quote("'", options.style).to_string(),
        ),
        QuotingStyle::C | QuotingStyle::CMaybe => ("\"".to_string(), "\"".to_string()),
        _ => ("'".to_string(), "'".to_string()),
    }
}

fn style_needs_outer_quotes(style: QuotingStyle) -> bool {
    !matches!(style, QuotingStyle::Literal)
}

fn render_bytes(bytes: &[u8], options: &QuotingOptions) -> String {
    let (left, right) = quote_delimiters(options);
    let mut out = String::new();

    let use_quotes = style_needs_outer_quotes(options.style)
        || !left.is_empty()
        || !right.is_empty()
        || bytes.iter().any(|&b| {
            matches!(
                b,
                b' ' | b'\t' | b'\n' | b'\r' | b'\'' | b'"' | b'\\' | b':'
            ) || char_is_forced(options, b)
                || !(0x20..=0x7e).contains(&b)
        });

    if use_quotes {
        out.push_str(&left);
    }

    for &b in bytes {
        let forced = char_is_forced(options, b);
        match options.style {
            QuotingStyle::Literal => {
                if forced || !(0x20..=0x7e).contains(&b) || b == b'\\' {
                    let _ = std::fmt::Write::write_fmt(&mut out, format_args!("\\x{:02x}", b));
                } else {
                    out.push(b as char);
                }
            }
            QuotingStyle::C | QuotingStyle::CMaybe | QuotingStyle::Escape => match b {
                b'\\' => out.push_str("\\\\"),
                b'"' => out.push_str("\\\""),
                b'\'' if forced => out.push_str("\\'"),
                b'\n' => out.push_str("\\n"),
                b'\r' => out.push_str("\\r"),
                b'\t' => out.push_str("\\t"),
                b'\0' => out.push_str("\\0"),
                0x20..=0x7e if !forced => out.push(b as char),
                _ => {
                    let _ = std::fmt::Write::write_fmt(&mut out, format_args!("\\x{:02x}", b));
                }
            },
            _ => match b {
                b'\'' => out.push_str("'\\''"),
                b'\\' | b'\n' | b'\r' | b'\t' | b'\0' if forced || matches!(b, b'\\' | b'\n' | b'\r' | b'\t' | b'\0') => {
                    match b {
                        b'\\' => out.push_str("\\\\"),
                        b'\n' => out.push_str("\\n"),
                        b'\r' => out.push_str("\\r"),
                        b'\t' => out.push_str("\\t"),
                        b'\0' => out.push_str("\\0"),
                        _ => {}
                    }
                }
                0x20..=0x7e if !forced => out.push(b as char),
                _ => {
                    let _ = std::fmt::Write::write_fmt(&mut out, format_args!("\\x{:02x}", b));
                }
            },
        }
    }

    if use_quotes {
        out.push_str(&right);
    }

    out
}

fn with_default_options<T>(o: Option<&QuotingOptions>, f: impl FnOnce(&QuotingOptions) -> T) -> T {
    if let Some(options) = o {
        f(options)
    } else {
        DEFAULT_QUOTING_OPTIONS.with(|cell| {
            let guard = cell.borrow();
            f(&guard)
        })
    }
}

fn with_default_options_mut<T>(
    o: Option<&mut QuotingOptions>,
    f: impl FnOnce(&mut QuotingOptions) -> T,
) -> T {
    if let Some(options) = o {
        f(options)
    } else {
        DEFAULT_QUOTING_OPTIONS.with(|cell| {
            let mut guard = cell.borrow_mut();
            f(&mut guard)
        })
    }
}

pub struct Quotearg {
    slots: BTreeSet<usize>,
}

impl Quotearg {
    pub fn clone_quoting_options(options: Option<&QuotingOptions>) -> QuotingOptions {
        with_default_options(options, Clone::clone)
    }

    pub fn get_quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        with_default_options(options, |o| o.style)
    }

    pub fn set_quoting_style(options: Option<&mut QuotingOptions>, style: QuotingStyle) {
        with_default_options_mut(options, |o| o.style = style);
    }

    pub fn set_char_quoting(options: Option<&mut QuotingOptions>, c: char, i: bool) -> bool {
        with_default_options_mut(options, |o| set_char_bit(o, c, i))
    }

    pub fn set_quoting_flags(options: Option<&mut QuotingOptions>, flags: i32) -> i32 {
        with_default_options_mut(options, |o| {
            let previous = o.flags;
            o.flags = flags;
            previous
        })
    }

    pub fn set_custom_quoting(
        options: Option<&mut QuotingOptions>,
        left_quote: &str,
        right_quote: &str,
    ) {
        assert!(
            !left_quote.is_empty() || !right_quote.is_empty() || left_quote == right_quote,
            "custom quoting delimiters must be provided together"
        );
        with_default_options_mut(options, |o| {
            o.style = QuotingStyle::Custom;
            o.left_quote = Some(left_quote.to_string());
            o.right_quote = Some(right_quote.to_string());
        });
    }

    pub fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions {
        assert!(style != QuotingStyle::Custom, "custom style requires delimiters");
        QuotingOptions {
            style,
            ..QuotingOptions::default()
        }
    }

    pub fn gettext_quote(msgid: &str, style: QuotingStyle) -> &'static str {
        let locale_code = Localcharset::locale_charset();
        if locale_code.eq_ignore_ascii_case("UTF-8") {
            return if msgid.starts_with('`') {
                "\u{2018}"
            } else {
                "\u{2019}"
            };
        }
        if locale_code.eq_ignore_ascii_case("GB18030") {
            return if msgid.starts_with('`') {
                "\u{00a1}\u{00ae}"
            } else {
                "\u{00a1}\u{00af}"
            };
        }
        if style == QuotingStyle::Clocale {
            "\""
        } else {
            "'"
        }
    }

    pub fn buffer_restyled(
        buffer: &mut String,
        arg: &[u8],
        quoting_style: QuotingStyle,
        flags: i32,
        quote_these_too: &[u32; 8],
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> usize {
        let options = QuotingOptions {
            style: quoting_style,
            flags,
            quote_these_too: *quote_these_too,
            left_quote: left_quote.map(ToOwned::to_owned),
            right_quote: right_quote.map(ToOwned::to_owned),
        };
        let rendered = render_bytes(arg, &options);
        buffer.clear();
        buffer.push_str(&rendered);
        rendered.len()
    }

    pub fn buffer(
        buffer: &mut String,
        arg: &[u8],
        options: Option<&QuotingOptions>,
    ) -> usize {
        let rendered = with_default_options(options, |o| render_bytes(arg, o));
        buffer.clear();
        buffer.push_str(&rendered);
        rendered.len()
    }

    pub fn new(arg: &str, argsize: usize, options: Option<&QuotingOptions>) -> Self {
        let mut this = Self {
            slots: BTreeSet::new(),
        };
        let bytes = normalize_c_string_len(arg, Some(argsize));
        let rendered = with_default_options(options, |o| render_bytes(bytes, o));
        SLOT_STORE.with(|cell| {
            let mut store = cell.borrow_mut();
            store.push(rendered);
            let index = store.len() - 1;
            this.slots.insert(index);
        });
        this
    }

    pub fn alloc_mem(arg: &[u8], options: Option<&QuotingOptions>) -> (String, usize) {
        let rendered = with_default_options(options, |o| render_bytes(arg, o));
        let len = rendered.len();
        (rendered, len)
    }

    pub fn n_options(
        &mut self,
        n: usize,
        arg: &[u8],
        options: Option<&QuotingOptions>,
    ) -> String {
        let rendered = with_default_options(options, |o| render_bytes(arg, o));
        SLOT_STORE.with(|cell| {
            let mut store = cell.borrow_mut();
            if store.len() <= n {
                store.resize(n + 1, String::new());
            }
            store[n] = rendered.clone();
            self.slots.insert(n);
        });
        rendered
    }

    pub fn n(&mut self, n: usize, arg: &str) -> String {
        let bytes = normalize_c_string_len(arg, None);
        self.n_options(n, bytes, None)
    }

    pub fn n_mem(&mut self, n: usize, arg: &[u8]) -> String {
        self.n_options(n, arg, None)
    }

    pub fn quotearg(&mut self, arg: &str) -> String {
        self.n(0, arg)
    }

    pub fn mem(&mut self, arg: &[u8]) -> String {
        self.n_mem(0, arg)
    }

    pub fn n_style(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let bytes = normalize_c_string_len(arg, None);
        self.n_style_mem(n, style, bytes)
    }

    pub fn n_style_mem(&mut self, n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let options = Self::quoting_options_from_style(style);
        self.n_options(n, arg, Some(&options))
    }

    pub fn style(&mut self, style: QuotingStyle, arg: &str) -> String {
        self.n_style(0, style, arg)
    }

    pub fn style_mem(&mut self, style: QuotingStyle, arg: &[u8]) -> String {
        self.n_style_mem(0, style, arg)
    }

    pub fn char_mem(&mut self, arg: &[u8], ch: char) -> String {
        let mut options = Self::clone_quoting_options(None);
        let _ = Self::set_char_quoting(Some(&mut options), ch, true);
        self.n_options(0, arg, Some(&options))
    }

    pub fn char(&mut self, arg: &str, ch: char) -> String {
        let bytes = normalize_c_string_len(arg, None);
        self.char_mem(bytes, ch)
    }

    pub fn colon(&mut self, arg: &str) -> String {
        self.char(arg, ':')
    }

    pub fn colon_mem(&mut self, arg: &[u8]) -> String {
        self.char_mem(arg, ':')
    }

    pub fn n_style_colon(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let mut options = Self::quoting_options_from_style(style);
        let _ = Self::set_char_quoting(Some(&mut options), ':', true);
        let bytes = normalize_c_string_len(arg, None);
        self.n_options(n, bytes, Some(&options))
    }

    pub fn n_custom(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &str,
    ) -> String {
        let bytes = normalize_c_string_len(arg, None);
        self.n_custom_mem(n, left_quote, right_quote, bytes)
    }

    pub fn n_custom_mem(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
    ) -> String {
        let mut options = QuotingOptions::default();
        Self::set_custom_quoting(Some(&mut options), left_quote, right_quote);
        self.n_options(n, arg, Some(&options))
    }

    pub fn custom(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(&mut self, left_quote: &str, right_quote: &str, arg: &[u8]) -> String {
        self.n_custom_mem(0, left_quote, right_quote, arg)
    }

    pub fn quote_n_mem(&mut self, n: usize, arg: &[u8]) -> String {
        self.n_options(n, arg, Some(Self::quoting_options()))
    }

    pub fn quote_mem(&mut self, arg: &[u8]) -> String {
        self.quote_n_mem(0, arg)
    }

    pub fn quote_n(&mut self, n: usize, arg: &str) -> String {
        let bytes = normalize_c_string_len(arg, None);
        self.quote_n_mem(n, bytes)
    }

    pub fn quote(&mut self, arg: &str) -> String {
        self.quote_n(0, arg)
    }

    pub fn left_quote(&self) -> &'static str {
        Self::gettext_quote("`", QuotingStyle::Locale)
    }

    pub fn right_quote(&self) -> &'static str {
        Self::gettext_quote("'", QuotingStyle::Locale)
    }

    pub fn quoting_options() -> &'static QuotingOptions {
        quote_quoting_options_cell()
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        Self::quoting_options().clone()
    }

    pub fn quoting_style() -> QuotingStyle {
        QuotingStyle::Literal
    }

    pub fn style_14(&mut self, arg: &str) -> String {
        self.style(Self::quoting_style(), arg)
    }

    pub fn n_08(&mut self, n: usize, arg: &str) -> String {
        self.n(n, arg)
    }

    pub fn colon_12(&mut self, arg: &str) -> String {
        self.colon(arg)
    }

    pub fn main_root_quoting_options_01() -> QuotingOptions {
        let mut options = QuotingOptions {
            style: QuotingStyle::Locale,
            ..QuotingOptions::default()
        };
        let _ = Self::set_char_quoting(Some(&mut options), ':', true);
        options
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        SLOT_STORE.with(|cell| {
            let mut store = cell.borrow_mut();
            for &index in &self.slots {
                if index < store.len() {
                    store[index].clear();
                }
            }
        });
    }
}
