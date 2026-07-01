use std::collections::BTreeSet;

use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    C,
    Escape,
    Locale,
    Clocale,
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: i32,
    pub quote_these_too: BTreeSet<u8>,
    pub left_quote: Option<String>,
    pub right_quote: Option<String>,
}

impl Default for QuotingOptions {
    fn default() -> Self {
        Self {
            style: QuotingStyle::Literal,
            flags: 0,
            quote_these_too: BTreeSet::new(),
            left_quote: None,
            right_quote: None,
        }
    }
}

pub struct Quotearg {
    slots: Vec<String>,
}

impl Quotearg {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    pub fn clone_quoting_options(options: Option<&QuotingOptions>) -> QuotingOptions {
        options.cloned().unwrap_or_else(Self::main_root_quoting_options_02)
    }

    pub fn get_quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        options
            .map(|o| o.style)
            .unwrap_or_else(|| Self::main_root_quoting_options_02().style)
    }

    pub fn set_quoting_style(options: &mut QuotingOptions, style: QuotingStyle) {
        options.style = style;
    }

    pub fn set_char_quoting(options: &mut QuotingOptions, c: char, enabled: bool) -> bool {
        let byte = if c.is_ascii() { c as u8 } else { b'?' };
        let old = options.quote_these_too.contains(&byte);
        if enabled {
            options.quote_these_too.insert(byte);
        } else {
            options.quote_these_too.remove(&byte);
        }
        old
    }

    pub fn set_quoting_flags(options: &mut QuotingOptions, flags: i32) -> i32 {
        let old = options.flags;
        options.flags = flags;
        old
    }

    pub fn set_custom_quoting(options: &mut QuotingOptions, left_quote: &str, right_quote: &str) {
        options.style = QuotingStyle::Custom;
        options.left_quote = Some(left_quote.to_owned());
        options.right_quote = Some(right_quote.to_owned());
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

        if CStrcasecmp::eq(&locale_code, "UTF-8") {
            return if msgid.starts_with('`') {
                "\u{2018}".to_owned()
            } else {
                "\u{2019}".to_owned()
            };
        }

        if CStrcasecmp::eq(&locale_code, "GB18030") {
            return if msgid.starts_with('`') {
                "\u{00A1}\u{00AE}".to_owned()
            } else {
                "\u{00A1}\u{00AF}".to_owned()
            };
        }

        match style {
            QuotingStyle::Clocale => "\"".to_owned(),
            _ => "'".to_owned(),
        }
    }

    pub fn buffer_restyled(
        output: &mut [u8],
        arg: &[u8],
        style: QuotingStyle,
        flags: i32,
        quote_these_too: Option<&BTreeSet<u8>>,
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> usize {
        let rendered = Self::render(arg, style, flags, quote_these_too, left_quote, right_quote);
        let bytes = rendered.as_bytes();
        let n = output.len().min(bytes.len());
        output[..n].copy_from_slice(&bytes[..n]);
        bytes.len()
    }

    pub fn buffer(output: &mut [u8], arg: &[u8], options: Option<&QuotingOptions>) -> usize {
        let default_options;
        let options = match options {
            Some(options) => options,
            None => {
                default_options = Self::main_root_quoting_options_02();
                &default_options
            }
        };

        Self::buffer_restyled(
            output,
            arg,
            options.style,
            options.flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        )
    }

    pub fn alloc_mem(arg: &[u8], options: Option<&QuotingOptions>) -> (String, usize) {
        let default_options;
        let options = match options {
            Some(options) => options,
            None => {
                default_options = Self::main_root_quoting_options_02();
                &default_options
            }
        };

        let rendered = Self::render(
            arg,
            options.style,
            options.flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        );
        let len = rendered.len();
        (rendered, len)
    }

    pub fn n_options(&mut self, n: usize, arg: &[u8], options: Option<&QuotingOptions>) -> &str {
        if self.slots.len() <= n {
            self.slots.resize(n + 1, String::new());
        }

        let default_options;
        let options = match options {
            Some(options) => options,
            None => {
                default_options = Self::main_root_quoting_options_02();
                &default_options
            }
        };

        self.slots[n] = Self::render(
            arg,
            options.style,
            options.flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        );
        &self.slots[n]
    }

    pub fn n(&mut self, n: usize, arg: &str) -> &str {
        self.n_mem(n, arg.as_bytes())
    }

    pub fn n_mem(&mut self, n: usize, arg: &[u8]) -> &str {
        self.n_options(n, arg, None)
    }

    pub fn quotearg(&mut self, arg: &str) -> &str {
        self.n(0, arg)
    }

    pub fn mem(&mut self, arg: &[u8]) -> &str {
        self.n_mem(0, arg)
    }

    pub fn n_style(&mut self, n: usize, style: QuotingStyle, arg: &str) -> &str {
        self.n_style_mem(n, style, arg.as_bytes())
    }

    pub fn n_style_mem(&mut self, n: usize, style: QuotingStyle, arg: &[u8]) -> &str {
        let options = Self::quoting_options_from_style(style);
        self.n_options(n, arg, Some(&options))
    }

    pub fn style(&mut self, style: QuotingStyle, arg: &str) -> &str {
        self.n_style(0, style, arg)
    }

    pub fn style_mem(&mut self, style: QuotingStyle, arg: &[u8]) -> &str {
        self.n_style_mem(0, style, arg)
    }

    pub fn char_mem(&mut self, arg: &[u8], ch: char) -> &str {
        let mut options = Self::main_root_quoting_options_02();
        Self::set_char_quoting(&mut options, ch, true);
        self.n_options(0, arg, Some(&options))
    }

    pub fn char(&mut self, arg: &str, ch: char) -> &str {
        self.char_mem(arg.as_bytes(), ch)
    }

    pub fn colon(&mut self, arg: &str) -> &str {
        self.char(arg, ':')
    }

    pub fn colon_mem(&mut self, arg: &[u8]) -> &str {
        self.char_mem(arg, ':')
    }

    pub fn n_style_colon(&mut self, n: usize, style: QuotingStyle, arg: &str) -> &str {
        let mut options = Self::quoting_options_from_style(style);
        Self::set_char_quoting(&mut options, ':', true);
        self.n_options(n, arg.as_bytes(), Some(&options))
    }

    pub fn n_custom(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &str,
    ) -> &str {
        self.n_custom_mem(n, left_quote, right_quote, arg.as_bytes())
    }

    pub fn n_custom_mem(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
    ) -> &str {
        let mut options = QuotingOptions::default();
        Self::set_custom_quoting(&mut options, left_quote, right_quote);
        self.n_options(n, arg, Some(&options))
    }

    pub fn custom(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> &str {
        self.n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(&mut self, left_quote: &str, right_quote: &str, arg: &[u8]) -> &str {
        self.n_custom_mem(0, left_quote, right_quote, arg)
    }

    pub fn quote_n_mem(&mut self, n: usize, arg: &[u8]) -> &str {
        let options = Self::quoting_options();
        self.n_options(n, arg, Some(&options))
    }

    pub fn quote_mem(&mut self, arg: &[u8]) -> &str {
        self.quote_n_mem(0, arg)
    }

    pub fn quote_n(&mut self, n: usize, arg: &str) -> &str {
        self.quote_n_mem(n, arg.as_bytes())
    }

    pub fn quote(&mut self, arg: &str) -> &str {
        self.quote_n(0, arg)
    }

    pub fn custom_13(&mut self, arg: &str) -> &str {
        self.custom("<<", ">>", arg)
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        QuotingOptions::default()
    }

    pub fn quoting_options() -> QuotingOptions {
        QuotingOptions {
            style: QuotingStyle::Locale,
            ..QuotingOptions::default()
        }
    }

    pub fn style_14(&mut self, arg: &str) -> &str {
        self.style(QuotingStyle::Shell, arg)
    }

    pub fn n_08(&mut self, n: usize, arg: &str) -> &str {
        self.n(n, arg)
    }

    pub fn colon_12(&mut self, arg: &str) -> &str {
        self.colon(arg)
    }

    pub fn main_root_quote_n_11(&mut self, n: usize, arg: &str) -> &str {
        self.quote_n(n, arg)
    }

    fn render(
        arg: &[u8],
        style: QuotingStyle,
        _flags: i32,
        quote_these_too: Option<&BTreeSet<u8>>,
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> String {
        let forced = quote_these_too.cloned().unwrap_or_default();

        match style {
            QuotingStyle::Literal => Self::render_literal(arg, &forced),
            QuotingStyle::Escape => Self::render_escape(arg, &forced),
            QuotingStyle::C => {
                let mut out = String::from("\"");
                out.push_str(&Self::render_c_body(arg, &forced));
                out.push('"');
                out
            }
            QuotingStyle::Shell => {
                if Self::needs_shell_quotes(arg, &forced) {
                    Self::render_shell_always(arg, &forced)
                } else {
                    Self::render_literal(arg, &forced)
                }
            }
            QuotingStyle::ShellAlways => Self::render_shell_always(arg, &forced),
            QuotingStyle::Locale | QuotingStyle::Clocale => {
                let lq = left_quote
                    .map(str::to_owned)
                    .unwrap_or_else(|| Self::gettext_quote("`", style));
                let rq = right_quote
                    .map(str::to_owned)
                    .unwrap_or_else(|| Self::gettext_quote("'", style));
                let mut out = lq;
                out.push_str(&Self::render_c_body(arg, &forced));
                out.push_str(&rq);
                out
            }
            QuotingStyle::Custom => {
                let lq = left_quote.unwrap_or("");
                let rq = right_quote.unwrap_or("");
                let mut out = String::from(lq);
                out.push_str(&Self::render_c_body(arg, &forced));
                out.push_str(rq);
                out
            }
        }
    }

    fn render_literal(arg: &[u8], forced: &BTreeSet<u8>) -> String {
        let mut out = String::new();
        for &b in arg {
            if forced.contains(&b) {
                out.push('\\');
            }
            out.push(char::from(b));
        }
        out
    }

    fn render_escape(arg: &[u8], forced: &BTreeSet<u8>) -> String {
        let mut out = String::new();
        for &b in arg {
            if forced.contains(&b) || !Self::is_safe_unquoted(b) {
                Self::push_c_escape(&mut out, b);
            } else {
                out.push(char::from(b));
            }
        }
        out
    }

    fn render_c_body(arg: &[u8], forced: &BTreeSet<u8>) -> String {
        let mut out = String::new();
        for &b in arg {
            if forced.contains(&b) {
                Self::push_c_escape(&mut out, b);
            } else {
                match b {
                    b'\n' | b'\r' | b'\t' | b'\x08' | b'\x0c' | b'\\' | b'"' => {
                        Self::push_c_escape(&mut out, b)
                    }
                    0x20..=0x7e => out.push(char::from(b)),
                    _ => Self::push_c_escape(&mut out, b),
                }
            }
        }
        out
    }

    fn render_shell_always(arg: &[u8], forced: &BTreeSet<u8>) -> String {
        let mut out = String::from("'");
        for &b in arg {
            if b == b'\'' {
                out.push_str("'\\''");
            } else if forced.contains(&b) {
                out.push_str("'\\");
                out.push(char::from(b));
                out.push('\'');
            } else {
                out.push(char::from(b));
            }
        }
        out.push('\'');
        out
    }

    fn needs_shell_quotes(arg: &[u8], forced: &BTreeSet<u8>) -> bool {
        arg.is_empty()
            || arg.iter().any(|b| {
                forced.contains(b)
                    || !matches!(
                        *b,
                        b'a'..=b'z'
                            | b'A'..=b'Z'
                            | b'0'..=b'9'
                            | b'_'
                            | b'-'
                            | b'.'
                            | b'/'
                    )
            })
    }

    fn is_safe_unquoted(b: u8) -> bool {
        matches!(b, 0x20..=0x7e) && !matches!(b, b'\\' | b'"' | b'\'')
    }

    fn push_c_escape(out: &mut String, b: u8) {
        match b {
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b'\x08' => out.push_str("\\b"),
            b'\x0c' => out.push_str("\\f"),
            b'\\' => out.push_str("\\\\"),
            b'"' => out.push_str("\\\""),
            b'\'' => out.push_str("\\'"),
            0x20..=0x7e => {
                out.push('\\');
                out.push(char::from(b));
            }
            _ => {
                let escaped = format!("\\{:03o}", b);
                out.push_str(&escaped);
            }
        }
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        self.slots.clear();
        self.slots.shrink_to_fit();
    }
}
