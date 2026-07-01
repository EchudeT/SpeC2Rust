use crate::c32isprint::C32Isprint;
use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;
use crate::mbrtoc32::{Mbrtoc32, Mbrtoc32Result};

const INT_BITS: usize = u32::BITS as usize;
const QUOTE_BITMAP_WORDS: usize = 256 / INT_BITS;

const QA_ELIDE_NULL_BYTES: u32 = 1 << 0;
const QA_ELIDE_OUTER_QUOTES: u32 = 1 << 1;
const QA_SPLIT_TRIGRAPHS: u32 = 1 << 2;

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
    CLocale,
    Custom,
}

#[derive(Clone, Debug)]
pub struct QuotingOptions {
    style: QuotingStyle,
    flags: u32,
    quote_these_too: [u32; QUOTE_BITMAP_WORDS],
    left_quote: Option<Vec<u8>>,
    right_quote: Option<Vec<u8>>,
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

#[derive(Clone, Debug, Default)]
pub struct Quotearg {
    options: QuotingOptions,
    slots: Vec<String>,
}

impl Quotearg {
    pub fn clone_quoting_options(&self) -> QuotingOptions {
        self.options.clone()
    }

    pub fn get_quoting_style(&self) -> QuotingStyle {
        self.options.style
    }

    pub fn set_quoting_style(&mut self, style: QuotingStyle) {
        self.options.style = style;
    }

    pub fn set_char_quoting(&mut self, ch: u8, should_quote: bool) -> bool {
        let word = (ch as usize) / INT_BITS;
        let shift = (ch as usize) % INT_BITS;
        let prev = ((self.options.quote_these_too[word] >> shift) & 1) != 0;
        let bit = 1u32 << shift;
        if should_quote {
            self.options.quote_these_too[word] |= bit;
        } else {
            self.options.quote_these_too[word] &= !bit;
        }
        prev
    }

    pub fn set_quoting_flags(&mut self, flags: u32) -> u32 {
        let prev = self.options.flags;
        self.options.flags = flags;
        prev
    }

    pub fn set_custom_quoting(&mut self, left_quote: &str, right_quote: &str) {
        assert!(!left_quote.is_empty() || !right_quote.is_empty() || true);
        self.options.style = QuotingStyle::Custom;
        self.options.left_quote = Some(left_quote.as_bytes().to_vec());
        self.options.right_quote = Some(right_quote.as_bytes().to_vec());
    }

    pub fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions {
        assert!(style != QuotingStyle::Custom);
        QuotingOptions {
            style,
            ..QuotingOptions::default()
        }
    }

    pub fn gettext_quote(msgid: &str, style: QuotingStyle) -> String {
        let locale_code = Localcharset::locale_charset();
        if CStrcasecmp::eq_ignore_case(&locale_code, "UTF-8") {
            if msgid.as_bytes().first() == Some(&b'`') {
                "‘".to_string()
            } else {
                "’".to_string()
            }
        } else if CStrcasecmp::eq_ignore_case(&locale_code, "GB18030") {
            if msgid.as_bytes().first() == Some(&b'`') {
                String::from_utf8_lossy(b"\xA1\xAE").into_owned()
            } else {
                String::from_utf8_lossy(b"\xA1\xAF").into_owned()
            }
        } else if style == QuotingStyle::CLocale {
            "\"".to_string()
        } else {
            "'".to_string()
        }
    }

    pub fn buffer_restyled(
        dst: &mut String,
        src: &[u8],
        style: QuotingStyle,
        flags: u32,
        quote_these_too: Option<&[u32; QUOTE_BITMAP_WORDS]>,
        left_quote: Option<&[u8]>,
        right_quote: Option<&[u8]>,
    ) -> usize {
        let rendered = render(src, src.len(), style, flags, quote_these_too, left_quote, right_quote);
        *dst = String::from_utf8_lossy(&rendered).into_owned();
        rendered.len()
    }

    pub fn buffer(&self, dst: &mut String, src: &[u8]) -> usize {
        Self::buffer_restyled(
            dst,
            src,
            self.options.style,
            self.options.flags,
            Some(&self.options.quote_these_too),
            self.options.left_quote.as_deref(),
            self.options.right_quote.as_deref(),
        )
    }

    pub fn new(arg: &str) -> Self {
        let mut q = Self::default();
        let rendered = q.alloc_mem(arg.as_bytes());
        q.slots.push(rendered);
        q
    }

    pub fn alloc_mem(&self, src: &[u8]) -> String {
        let mut flags = self.options.flags;
        flags &= !QA_ELIDE_NULL_BYTES;
        String::from_utf8_lossy(&render(src, src.len(), self.options.style, flags, Some(&self.options.quote_these_too), self.options.left_quote.as_deref(), self.options.right_quote.as_deref())).into_owned()
    }

    pub fn n_options(&mut self, n: usize, src: &[u8], options: &QuotingOptions) -> String {
        let mut flags = options.flags | QA_ELIDE_NULL_BYTES;
        let rendered = render(
            src,
            src.len(),
            options.style,
            flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        );
        let s = String::from_utf8_lossy(&rendered).into_owned();
        if self.slots.len() <= n {
            self.slots.resize(n + 1, String::new());
        }
        self.slots[n] = s.clone();
        flags = flags;
        s
    }

    pub fn n(&mut self, n: usize, arg: &str) -> String {
        self.n_options(n, arg.as_bytes(), &self.options.clone())
    }

    pub fn n_mem(&mut self, n: usize, arg: &[u8]) -> String {
        self.n_options(n, arg, &self.options.clone())
    }

    pub fn quotearg(&mut self, arg: &str) -> String {
        self.n(0, arg)
    }

    pub fn mem(&mut self, arg: &[u8]) -> String {
        self.n_mem(0, arg)
    }

    pub fn n_style(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let o = Self::quoting_options_from_style(style);
        self.n_options(n, arg.as_bytes(), &o)
    }

    pub fn n_style_mem(&mut self, n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let o = Self::quoting_options_from_style(style);
        self.n_options(n, arg, &o)
    }

    pub fn style(&mut self, style: QuotingStyle, arg: &str) -> String {
        self.n_style(0, style, arg)
    }

    pub fn style_mem(&mut self, style: QuotingStyle, arg: &[u8]) -> String {
        self.n_style_mem(0, style, arg)
    }

    pub fn char_mem(&mut self, arg: &[u8], ch: u8) -> String {
        let mut options = self.options.clone();
        set_char_quoting_on(&mut options, ch, true);
        self.n_options(0, arg, &options)
    }

    pub fn char(&mut self, arg: &str, ch: u8) -> String {
        self.char_mem(arg.as_bytes(), ch)
    }

    pub fn colon(&mut self, arg: &str) -> String {
        self.char(arg, b':')
    }

    pub fn colon_mem(&mut self, arg: &[u8]) -> String {
        self.char_mem(arg, b':')
    }

    pub fn n_style_colon(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let mut options = Self::quoting_options_from_style(style);
        set_char_quoting_on(&mut options, b':', true);
        self.n_options(n, arg.as_bytes(), &options)
    }

    pub fn n_custom(&mut self, n: usize, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.n_custom_mem(n, left_quote, right_quote, arg.as_bytes())
    }

    pub fn n_custom_mem(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
    ) -> String {
        let mut o = self.options.clone();
        o.style = QuotingStyle::Custom;
        o.left_quote = Some(left_quote.as_bytes().to_vec());
        o.right_quote = Some(right_quote.as_bytes().to_vec());
        self.n_options(n, arg, &o)
    }

    pub fn custom(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(&mut self, left_quote: &str, right_quote: &str, arg: &[u8]) -> String {
        self.n_custom_mem(0, left_quote, right_quote, arg)
    }

    pub fn quote_n_mem(&mut self, n: usize, arg: &[u8]) -> String {
        let o = Self::main_root_quoting_options_02();
        self.n_options(n, arg, &o)
    }

    pub fn quote_mem(&mut self, arg: &[u8]) -> String {
        self.quote_n_mem(0, arg)
    }

    pub fn quote_n(&mut self, n: usize, arg: &str) -> String {
        self.quote_n_mem(n, arg.as_bytes())
    }

    pub fn quote(&mut self, arg: &str) -> String {
        self.quote_n(0, arg)
    }

    pub fn main_root_quoting_options_01() -> QuotingOptions {
        QuotingOptions::default()
    }

    pub fn quoting_options() -> QuotingOptions {
        Self::main_root_quoting_options_01()
    }

    pub fn n_07(&mut self, n: usize, arg: &str) -> String {
        self.n(n, arg)
    }

    pub fn main_root_quote_n_10(&mut self, n: usize, arg: &str) -> String {
        self.quote_n(n, arg)
    }

    pub fn style_13(&mut self, style: QuotingStyle, arg: &str) -> String {
        self.style(style, arg)
    }

    pub fn colon_11(&mut self, arg: &str) -> String {
        self.colon(arg)
    }

    pub fn custom_12(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.custom(left_quote, right_quote, arg)
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        let mut o = QuotingOptions::default();
        o.style = QuotingStyle::Locale;
        o
    }

    pub fn left_quote(&self) -> Option<String> {
        self.options
            .left_quote
            .as_ref()
            .map(|v| String::from_utf8_lossy(v).into_owned())
    }

    pub fn right_quote(&self) -> Option<String> {
        self.options
            .right_quote
            .as_ref()
            .map(|v| String::from_utf8_lossy(v).into_owned())
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        self.slots.clear();
    }
}

fn set_char_quoting_on(options: &mut QuotingOptions, ch: u8, should_quote: bool) -> bool {
    let word = (ch as usize) / INT_BITS;
    let shift = (ch as usize) % INT_BITS;
    let prev = ((options.quote_these_too[word] >> shift) & 1) != 0;
    let bit = 1u32 << shift;
    if should_quote {
        options.quote_these_too[word] |= bit;
    } else {
        options.quote_these_too[word] &= !bit;
    }
    prev
}

fn is_unibyte_locale() -> bool {
    !CStrcasecmp::eq_ignore_case(&Localcharset::locale_charset(), "UTF-8")
}

fn default_quotes_for_style(style: QuotingStyle) -> (Vec<u8>, Vec<u8>) {
    (
        Quotearg::gettext_quote("`", style).into_bytes(),
        Quotearg::gettext_quote("'", style).into_bytes(),
    )
}

fn render(
    arg: &[u8],
    argsize: usize,
    mut quoting_style: QuotingStyle,
    flags: u32,
    quote_these_too: Option<&[u32; QUOTE_BITMAP_WORDS]>,
    mut left_quote: Option<&[u8]>,
    mut right_quote: Option<&[u8]>,
) -> Vec<u8> {
    let mut out = Vec::new();
    let mut quote_string: Option<Vec<u8>> = None;
    let mut quote_string_len = 0usize;
    let mut backslash_escapes = false;
    let unibyte_locale = is_unibyte_locale();
    let mut elide_outer_quotes = (flags & QA_ELIDE_OUTER_QUOTES) != 0;
    let mut encountered_single_quote = false;
    let mut all_c_and_shell_quote_compat = true;
    let mut pending_shell_escape_end = false;
    let mut orig_probe = false;

    match quoting_style {
        QuotingStyle::CMaybe => {
            quoting_style = QuotingStyle::C;
            elide_outer_quotes = true;
            if !elide_outer_quotes {
                out.push(b'"');
            }
            backslash_escapes = true;
            quote_string = Some(vec![b'"']);
            quote_string_len = 1;
        }
        QuotingStyle::C => {
            if !elide_outer_quotes {
                out.push(b'"');
            }
            backslash_escapes = true;
            quote_string = Some(vec![b'"']);
            quote_string_len = 1;
        }
        QuotingStyle::Escape => {
            backslash_escapes = true;
            elide_outer_quotes = false;
        }
        QuotingStyle::Locale | QuotingStyle::CLocale | QuotingStyle::Custom => {
            let (lq, rq) = if quoting_style != QuotingStyle::Custom {
                default_quotes_for_style(quoting_style)
            } else {
                (
                    left_quote.unwrap_or_default().to_vec(),
                    right_quote.unwrap_or_default().to_vec(),
                )
            };
            if !elide_outer_quotes {
                out.extend_from_slice(&lq);
            }
            backslash_escapes = true;
            quote_string_len = rq.len();
            quote_string = Some(rq);
        }
        QuotingStyle::ShellEscape => {
            backslash_escapes = true;
            elide_outer_quotes = true;
            quoting_style = QuotingStyle::ShellAlways;
            if !elide_outer_quotes {
                out.push(b'\'');
            }
            quote_string = Some(vec![b'\'']);
            quote_string_len = 1;
        }
        QuotingStyle::Shell => {
            elide_outer_quotes = true;
            quoting_style = QuotingStyle::ShellAlways;
            if !elide_outer_quotes {
                out.push(b'\'');
            }
            quote_string = Some(vec![b'\'']);
            quote_string_len = 1;
        }
        QuotingStyle::ShellEscapeAlways => {
            if !elide_outer_quotes {
                backslash_escapes = true;
            }
            quoting_style = QuotingStyle::ShellAlways;
            if !elide_outer_quotes {
                out.push(b'\'');
            }
            quote_string = Some(vec![b'\'']);
            quote_string_len = 1;
        }
        QuotingStyle::ShellAlways => {
            if !elide_outer_quotes {
                out.push(b'\'');
            }
            quote_string = Some(vec![b'\'']);
            quote_string_len = 1;
        }
        QuotingStyle::Literal => {
            elide_outer_quotes = false;
        }
        QuotingStyle::Custom => unreachable!(),
    }

    let mut i = 0usize;
    while i < argsize {
        let mut c = arg[i];
        let mut esc = 0u8;
        let mut is_right_quote = false;
        let mut escaping = false;
        let mut c_and_shell_quote_compat = false;

        if backslash_escapes
            && quoting_style != QuotingStyle::ShellAlways
            && quote_string_len > 0
            && i + quote_string_len <= argsize
            && quote_string
                .as_ref()
                .map(|q| &arg[i..i + quote_string_len] == q.as_slice())
                .unwrap_or(false)
        {
            if elide_outer_quotes {
                return render(
                    arg,
                    argsize,
                    if quoting_style == QuotingStyle::ShellAlways && backslash_escapes {
                        QuotingStyle::ShellEscapeAlways
                    } else {
                        quoting_style
                    },
                    flags & !QA_ELIDE_OUTER_QUOTES,
                    None,
                    left_quote,
                    right_quote,
                );
            }
            is_right_quote = true;
        }

        match c {
            0 => {
                if backslash_escapes {
                    if elide_outer_quotes {
                        return render(
                            arg,
                            argsize,
                            if quoting_style == QuotingStyle::ShellAlways && backslash_escapes {
                                QuotingStyle::ShellEscapeAlways
                            } else {
                                quoting_style
                            },
                            flags & !QA_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                    if quoting_style != QuotingStyle::ShellAlways
                        && i + 1 < argsize
                        && arg[i + 1].is_ascii_digit()
                    {
                        out.push(b'0');
                        out.push(b'0');
                    }
                    c = b'0';
                } else if (flags & QA_ELIDE_NULL_BYTES) != 0 {
                    i += 1;
                    continue;
                }
            }
            b'?' => {
                if quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                    return render(
                        arg,
                        argsize,
                        if quoting_style == QuotingStyle::ShellAlways && backslash_escapes {
                            QuotingStyle::ShellEscapeAlways
                        } else {
                            quoting_style
                        },
                        flags & !QA_ELIDE_OUTER_QUOTES,
                        None,
                        left_quote,
                        right_quote,
                    );
                }
                if quoting_style == QuotingStyle::C
                    && (flags & QA_SPLIT_TRIGRAPHS) != 0
                    && i + 2 < argsize
                    && arg[i + 1] == b'?'
                    && matches!(arg[i + 2], b'!' | b'\'' | b'(' | b')' | b'-' | b'/' | b'<' | b'=' | b'>')
                {
                    if elide_outer_quotes {
                        return render(
                            arg,
                            argsize,
                            if quoting_style == QuotingStyle::ShellAlways && backslash_escapes {
                                QuotingStyle::ShellEscapeAlways
                            } else {
                                quoting_style
                            },
                            flags & !QA_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    out.extend_from_slice(b"?\"\"?");
                    c = arg[i + 2];
                    i += 2;
                }
            }
            7 => {
                esc = b'a';
                if backslash_escapes {
                    c = esc;
                    if elide_outer_quotes {
                        return render(
                            arg,
                            argsize,
                            if quoting_style == QuotingStyle::ShellAlways && backslash_escapes {
                                QuotingStyle::ShellEscapeAlways
                            } else {
                                quoting_style
                            },
                            flags & !QA_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            8 => {
                esc = b'b';
                if backslash_escapes {
                    c = esc;
                    if elide_outer_quotes {
                        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                    }
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            12 => {
                esc = b'f';
                if backslash_escapes {
                    c = esc;
                    if elide_outer_quotes {
                        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                    }
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            b'\n' => {
                esc = b'n';
                if quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                    return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                }
                if backslash_escapes {
                    c = esc;
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            b'\r' => {
                esc = b'r';
                if quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                    return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                }
                if backslash_escapes {
                    c = esc;
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            b'\t' => {
                esc = b't';
                if quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                    return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                }
                if backslash_escapes {
                    c = esc;
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            11 => {
                esc = b'v';
                if backslash_escapes {
                    c = esc;
                    if elide_outer_quotes {
                        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                    }
                    escaping = true;
                    if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                        out.extend_from_slice(b"'$'");
                        pending_shell_escape_end = true;
                    }
                    out.push(b'\\');
                }
            }
            b'\\' => {
                esc = b'\\';
                if quoting_style == QuotingStyle::ShellAlways {
                    if elide_outer_quotes {
                        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                    }
                } else if backslash_escapes && elide_outer_quotes && quote_string_len > 0 {
                } else if backslash_escapes {
                    c = esc;
                    escaping = true;
                    out.push(b'\\');
                }
            }
            b'{' | b'}' => {
                if argsize == 1 {}
            }
            b'#' | b'~' | b' ' | b'!' | b'"' | b'$' | b'&' | b'(' | b')' | b'*' | b';' | b'<' | b'='
            | b'>' | b'[' | b'^' | b'`' | b'|' => {
                if c == b' ' {
                    c_and_shell_quote_compat = true;
                }
                if quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                    return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                }
            }
            b'\'' => {
                encountered_single_quote = true;
                c_and_shell_quote_compat = true;
                if quoting_style == QuotingStyle::ShellAlways {
                    if elide_outer_quotes {
                        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                    }
                    orig_probe = true;
                    out.extend_from_slice(b"'\\''");
                    pending_shell_escape_end = false;
                    i += 1;
                    continue;
                }
            }
            b'%' | b'+' | b',' | b'-' | b'.' | b'/' | b'0'..=b'9' | b':' | b'A'..=b'Z' | b']' | b'_'
            | b'a'..=b'z' => {
                c_and_shell_quote_compat = true;
            }
            _ => {
                let mut m = 1usize;
                let mut printable = if unibyte_locale {
                    c.is_ascii_graphic() || c == b' '
                } else {
                    let mut state = Mbrtoc32::default();
                    let mut offset = 0usize;
                    let mut ok = true;
                    while i + offset < argsize {
                        let mut w = 0u32;
                        match Mbrtoc32::mbrtoc_32(
                            Some(&mut w),
                            Some(&arg[i + offset..]),
                            argsize - (i + offset),
                            Some(&mut state),
                        ) {
                            Mbrtoc32Result::Complete { bytes, .. } => {
                                if bytes == 0 {
                                    break;
                                }
                                if !C32Isprint::is_print(w) {
                                    ok = false;
                                }
                                m += bytes.saturating_sub(1);
                                offset += bytes;
                                if offset > 0 {
                                    break;
                                }
                            }
                            Mbrtoc32Result::Incomplete | Mbrtoc32Result::Invalid => {
                                ok = false;
                                break;
                            }
                            Mbrtoc32Result::Null { .. } => break,
                        }
                    }
                    ok
                };
                c_and_shell_quote_compat = printable;
                if m > 1 || (backslash_escapes && !printable) {
                    let ilim = i + m;
                    loop {
                        if backslash_escapes && !printable {
                            if elide_outer_quotes {
                                return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
                            }
                            escaping = true;
                            if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                                out.extend_from_slice(b"'$'");
                                pending_shell_escape_end = true;
                            }
                            out.push(b'\\');
                            out.push(b'0' + (c >> 6));
                            out.push(b'0' + ((c >> 3) & 7));
                            c = b'0' + (c & 7);
                        } else if is_right_quote {
                            out.push(b'\\');
                            is_right_quote = false;
                        }
                        if ilim <= i + 1 {
                            break;
                        }
                        if pending_shell_escape_end && !escaping {
                            out.extend_from_slice(b"''");
                            pending_shell_escape_end = false;
                        }
                        out.push(c);
                        i += 1;
                        c = arg[i];
                    }
                }
            }
        }

        let quote_forced = quote_these_too
            .map(|bitmap| ((bitmap[(c as usize) / INT_BITS] >> ((c as usize) % INT_BITS)) & 1) != 0)
            .unwrap_or(false);

        if (((backslash_escapes && quoting_style != QuotingStyle::ShellAlways) || elide_outer_quotes)
            && quote_forced)
            || is_right_quote
        {
            if elide_outer_quotes {
                return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
            }
            escaping = true;
            if quoting_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                out.extend_from_slice(b"'$'");
                pending_shell_escape_end = true;
            }
            out.push(b'\\');
        }

        if pending_shell_escape_end && !escaping {
            out.extend_from_slice(b"''");
            pending_shell_escape_end = false;
        }

        out.push(c);

        if !c_and_shell_quote_compat {
            all_c_and_shell_quote_compat = false;
        }

        i += 1;
    }

    if out.is_empty() && quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
        return render(arg, argsize, quoting_style, flags & !QA_ELIDE_OUTER_QUOTES, None, left_quote, right_quote);
    }

    if quoting_style == QuotingStyle::ShellAlways && !elide_outer_quotes && encountered_single_quote {
        if all_c_and_shell_quote_compat {
            return render(arg, argsize, QuotingStyle::C, flags, quote_these_too, left_quote, right_quote);
        } else if orig_probe {
            out.clear();
            return render(arg, argsize, quoting_style, flags, quote_these_too, left_quote, right_quote);
        }
    }

    if let Some(qs) = quote_string {
        if !elide_outer_quotes {
            out.extend_from_slice(&qs);
        }
    }

    out
}
