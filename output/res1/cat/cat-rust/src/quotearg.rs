use crate::localcharset::Localcharset;

const INT_BITS: usize = u32::BITS as usize;
const CHAR_MASK_WORDS: usize = 256 / INT_BITS;

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
    Clocale,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: u32,
    pub quote_these_too: [u32; CHAR_MASK_WORDS],
    pub left_quote: Option<String>,
    pub right_quote: Option<String>,
}

impl Default for QuotingOptions {
    fn default() -> Self {
        Self {
            style: QuotingStyle::Literal,
            flags: 0,
            quote_these_too: [0; CHAR_MASK_WORDS],
            left_quote: None,
            right_quote: None,
        }
    }
}

pub struct Quotearg {
    slots: Vec<String>,
}

impl Quotearg {
    pub fn clone_quoting_options(options: Option<&QuotingOptions>) -> QuotingOptions {
        options.cloned().unwrap_or_default()
    }

    pub fn get_quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        options.unwrap_or(&Self::main_root_quoting_options_01()).style
    }

    pub fn set_quoting_style(options: &mut QuotingOptions, style: QuotingStyle) {
        options.style = style;
    }

    pub fn set_char_quoting(options: &mut QuotingOptions, ch: u8, should_quote: bool) -> bool {
        let idx = ch as usize / INT_BITS;
        let shift = ch as usize % INT_BITS;
        let mask = 1u32 << shift;
        let previous = (options.quote_these_too[idx] & mask) != 0;
        if should_quote {
            options.quote_these_too[idx] |= mask;
        } else {
            options.quote_these_too[idx] &= !mask;
        }
        previous
    }

    pub fn set_quoting_flags(options: &mut QuotingOptions, flags: u32) -> u32 {
        let previous = options.flags;
        options.flags = flags;
        previous
    }

    pub fn set_custom_quoting(
        options: &mut QuotingOptions,
        left_quote: impl Into<String>,
        right_quote: impl Into<String>,
    ) {
        let left_quote = left_quote.into();
        let right_quote = right_quote.into();
        options.style = QuotingStyle::Custom;
        options.left_quote = Some(left_quote);
        options.right_quote = Some(right_quote);
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
        if locale_code.eq_ignore_ascii_case("UTF-8") {
            return if msgid.starts_with('`') {
                "\u{2018}".to_string()
            } else {
                "\u{2019}".to_string()
            };
        }
        if locale_code.eq_ignore_ascii_case("GB18030") {
            return if msgid.starts_with('`') {
                "\u{00A1}\u{00AE}".to_string()
            } else {
                "\u{00A1}\u{00AF}".to_string()
            };
        }
        match style {
            QuotingStyle::Clocale => "\"".to_string(),
            _ => "'".to_string(),
        }
    }

    pub fn buffer_restyled(
        buffer: &mut [u8],
        arg: &[u8],
        quoting_style: QuotingStyle,
        flags: u32,
        quote_these_too: Option<&[u32; CHAR_MASK_WORDS]>,
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> usize {
        let quoted = Self::render_restyled(
            arg,
            quoting_style,
            flags,
            quote_these_too,
            left_quote,
            right_quote,
        );
        let n = quoted.len().min(buffer.len());
        buffer[..n].copy_from_slice(&quoted[..n]);
        quoted.len()
    }

    pub fn buffer(buffer: &mut [u8], arg: &[u8], options: Option<&QuotingOptions>) -> usize {
        let default_options;
        let options = match options {
            Some(o) => o,
            None => {
                default_options = Self::main_root_quoting_options_01();
                &default_options
            }
        };
        Self::buffer_restyled(
            buffer,
            arg,
            options.style,
            options.flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        )
    }

    pub fn new(arg: &[u8], argsize: usize, options: Option<&QuotingOptions>) -> String {
        Self::alloc_mem(arg, argsize, options).0
    }

    pub fn alloc_mem(
        arg: &[u8],
        argsize: usize,
        options: Option<&QuotingOptions>,
    ) -> (String, usize) {
        let default_options;
        let options = match options {
            Some(o) => o,
            None => {
                default_options = Self::main_root_quoting_options_01();
                &default_options
            }
        };
        let slice = &arg[..argsize.min(arg.len())];
        let rendered = Self::render_restyled(
            slice,
            options.style,
            options.flags,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        );
        let text = String::from_utf8_lossy(&rendered).into_owned();
        let size = text.len();
        (text, size)
    }

    pub fn n_options(&mut self, n: usize, arg: &[u8], argsize: usize, options: &QuotingOptions) -> String {
        let slice = &arg[..argsize.min(arg.len())];
        let rendered = Self::render_restyled(
            slice,
            options.style,
            options.flags | QA_ELIDE_NULL_BYTES,
            Some(&options.quote_these_too),
            options.left_quote.as_deref(),
            options.right_quote.as_deref(),
        );
        let text = String::from_utf8_lossy(&rendered).into_owned();
        if self.slots.len() <= n {
            self.slots.resize(n + 1, String::new());
        }
        self.slots[n] = text.clone();
        text
    }

    pub fn n(&mut self, n: usize, arg: &str) -> String {
        self.n_options(n, arg.as_bytes(), arg.len(), &Self::main_root_quoting_options_01())
    }

    pub fn n_mem(&mut self, n: usize, arg: &[u8], argsize: usize) -> String {
        self.n_options(n, arg, argsize, &Self::main_root_quoting_options_01())
    }

    pub fn quotearg(&mut self, arg: &str) -> String {
        self.n(0, arg)
    }

    pub fn mem(&mut self, arg: &[u8], argsize: usize) -> String {
        self.n_mem(0, arg, argsize)
    }

    pub fn n_style(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let options = Self::quoting_options_from_style(style);
        self.n_options(n, arg.as_bytes(), arg.len(), &options)
    }

    pub fn n_style_mem(&mut self, n: usize, style: QuotingStyle, arg: &[u8], argsize: usize) -> String {
        let options = Self::quoting_options_from_style(style);
        self.n_options(n, arg, argsize, &options)
    }

    pub fn style(&mut self, style: QuotingStyle, arg: &str) -> String {
        self.n_style(0, style, arg)
    }

    pub fn style_mem(&mut self, style: QuotingStyle, arg: &[u8], argsize: usize) -> String {
        self.n_style_mem(0, style, arg, argsize)
    }

    pub fn char_mem(&mut self, arg: &[u8], argsize: usize, ch: u8) -> String {
        let mut options = Self::main_root_quoting_options_01();
        Self::set_char_quoting(&mut options, ch, true);
        self.n_options(0, arg, argsize, &options)
    }

    pub fn char(&mut self, arg: &str, ch: u8) -> String {
        self.char_mem(arg.as_bytes(), arg.len(), ch)
    }

    pub fn colon(&mut self, arg: &str) -> String {
        self.char(arg, b':')
    }

    pub fn colon_mem(&mut self, arg: &[u8], argsize: usize) -> String {
        self.char_mem(arg, argsize, b':')
    }

    pub fn n_style_colon(&mut self, n: usize, style: QuotingStyle, arg: &str) -> String {
        let mut options = Self::quoting_options_from_style(style);
        Self::set_char_quoting(&mut options, b':', true);
        self.n_options(n, arg.as_bytes(), arg.len(), &options)
    }

    pub fn n_custom(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &str,
    ) -> String {
        self.n_custom_mem(n, left_quote, right_quote, arg.as_bytes(), arg.len())
    }

    pub fn n_custom_mem(
        &mut self,
        n: usize,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
        argsize: usize,
    ) -> String {
        let mut options = Self::main_root_quoting_options_01();
        Self::set_custom_quoting(&mut options, left_quote.to_string(), right_quote.to_string());
        self.n_options(n, arg, argsize, &options)
    }

    pub fn custom(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(
        &mut self,
        left_quote: &str,
        right_quote: &str,
        arg: &[u8],
        argsize: usize,
    ) -> String {
        self.n_custom_mem(0, left_quote, right_quote, arg, argsize)
    }

    pub fn quote_n_mem(&mut self, n: usize, arg: &[u8], argsize: usize) -> String {
        self.n_options(n, arg, argsize, &Self::main_root_quoting_options_02())
    }

    pub fn quote_mem(&mut self, arg: &[u8], argsize: usize) -> String {
        self.quote_n_mem(0, arg, argsize)
    }

    pub fn quote_n(&mut self, n: usize, arg: &str) -> String {
        self.quote_n_mem(n, arg.as_bytes(), arg.len())
    }

    pub fn quote(&mut self, arg: &str) -> String {
        self.quote_n(0, arg)
    }

    pub fn main_root_quoting_options_01() -> QuotingOptions {
        QuotingOptions::default()
    }

    pub fn n_07(&mut self, n: usize, arg: &str) -> String {
        self.n(n, arg)
    }

    pub fn left_quote(style: QuotingStyle) -> String {
        match style {
            QuotingStyle::Custom => String::new(),
            _ => Self::gettext_quote("`", style),
        }
    }

    pub fn right_quote(style: QuotingStyle) -> String {
        match style {
            QuotingStyle::Custom => String::new(),
            _ => Self::gettext_quote("'", style),
        }
    }

    pub fn quoting_options() -> QuotingOptions {
        Self::main_root_quoting_options_01()
    }

    pub fn style_13(&mut self, style: QuotingStyle, arg: &str) -> String {
        self.style(style, arg)
    }

    pub fn quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        Self::get_quoting_style(options)
    }

    pub fn colon_11(&mut self, arg: &str) -> String {
        self.colon(arg)
    }

    pub fn custom_12(&mut self, left_quote: &str, right_quote: &str, arg: &str) -> String {
        self.custom(left_quote, right_quote, arg)
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        let mut options = QuotingOptions::default();
        options.style = QuotingStyle::Locale;
        options
    }

    fn render_restyled(
        arg: &[u8],
        mut quoting_style: QuotingStyle,
        mut flags: u32,
        quote_these_too: Option<&[u32; CHAR_MASK_WORDS]>,
        left_quote: Option<&str>,
        right_quote: Option<&str>,
    ) -> Vec<u8> {
        let mut local_left = left_quote.map(|s| s.as_bytes().to_vec());
        let mut local_right = right_quote.map(|s| s.as_bytes().to_vec());
        let mut elide_outer_quotes = (flags & QA_ELIDE_OUTER_QUOTES) != 0;
        let mut backslash_escapes = false;
        let mut quote_string: Vec<u8> = Vec::new();
        let mut out = Vec::new();

        match quoting_style {
            QuotingStyle::CMaybe => {
                quoting_style = QuotingStyle::C;
                elide_outer_quotes = true;
            }
            _ => {}
        }

        match quoting_style {
            QuotingStyle::C => {
                if !elide_outer_quotes {
                    out.push(b'"');
                }
                backslash_escapes = true;
                quote_string = b"\"".to_vec();
            }
            QuotingStyle::Escape => {
                backslash_escapes = true;
                elide_outer_quotes = false;
            }
            QuotingStyle::Locale | QuotingStyle::Clocale | QuotingStyle::Custom => {
                if quoting_style != QuotingStyle::Custom {
                    local_left = Some(Self::gettext_quote("`", quoting_style).into_bytes());
                    local_right = Some(Self::gettext_quote("'", quoting_style).into_bytes());
                }
                let left = local_left.unwrap_or_default();
                let right = local_right.unwrap_or_default();
                if !elide_outer_quotes {
                    out.extend_from_slice(&left);
                }
                backslash_escapes = true;
                quote_string = right.clone();
                local_right = Some(right);
            }
            QuotingStyle::ShellEscape => {
                backslash_escapes = true;
                elide_outer_quotes = true;
                quoting_style = QuotingStyle::ShellAlways;
                quote_string = b"'".to_vec();
            }
            QuotingStyle::Shell => {
                elide_outer_quotes = true;
                quoting_style = QuotingStyle::ShellAlways;
                quote_string = b"'".to_vec();
            }
            QuotingStyle::ShellEscapeAlways => {
                backslash_escapes = true;
                quoting_style = QuotingStyle::ShellAlways;
                if !elide_outer_quotes {
                    out.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::ShellAlways => {
                if !elide_outer_quotes {
                    out.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::Literal => {
                elide_outer_quotes = false;
            }
            QuotingStyle::Custom => {}
        }

        let mut encountered_single_quote = false;

        for &c in arg {
            if c == 0 && (flags & QA_ELIDE_NULL_BYTES) != 0 && !backslash_escapes {
                continue;
            }

            let forced = quote_these_too
                .map(|mask| (mask[c as usize / INT_BITS] >> (c as usize % INT_BITS)) & 1 != 0)
                .unwrap_or(false);

            match c {
                0 => {
                    if backslash_escapes {
                        out.push(b'\\');
                        out.push(b'0');
                    }
                }
                b'\n' => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\n");
                    } else {
                        out.push(c);
                    }
                }
                b'\r' => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\r");
                    } else {
                        out.push(c);
                    }
                }
                b'\t' => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\t");
                    } else {
                        out.push(c);
                    }
                }
                7 => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\a");
                    } else {
                        out.push(c);
                    }
                }
                8 => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\b");
                    } else {
                        out.push(c);
                    }
                }
                12 => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\f");
                    } else {
                        out.push(c);
                    }
                }
                11 => {
                    if backslash_escapes {
                        out.extend_from_slice(b"\\v");
                    } else {
                        out.push(c);
                    }
                }
                b'\\' => {
                    if backslash_escapes && quoting_style != QuotingStyle::ShellAlways {
                        out.extend_from_slice(b"\\\\");
                    } else {
                        out.push(c);
                    }
                }
                b'\'' if quoting_style == QuotingStyle::ShellAlways && !elide_outer_quotes => {
                    encountered_single_quote = true;
                    out.extend_from_slice(br"'\''");
                }
                b'\'' => {
                    encountered_single_quote = true;
                    if forced && backslash_escapes {
                        out.extend_from_slice(br"\'");
                    } else {
                        out.push(c);
                    }
                }
                b'"' if backslash_escapes => {
                    out.extend_from_slice(br#"\""#);
                }
                _ if forced && backslash_escapes => {
                    out.push(b'\\');
                    out.push(c);
                }
                _ => out.push(c),
            }
        }

        if out.is_empty() && quoting_style == QuotingStyle::ShellAlways && elide_outer_quotes {
            flags &= !QA_ELIDE_OUTER_QUOTES;
            return Self::render_restyled(
                arg,
                QuotingStyle::ShellAlways,
                flags,
                None,
                left_quote,
                right_quote,
            );
        }

        if !quote_string.is_empty() && !elide_outer_quotes {
            out.extend_from_slice(&quote_string);
        }

        if quoting_style == QuotingStyle::ShellAlways
            && !elide_outer_quotes
            && encountered_single_quote
            && arg.iter().all(|b| b.is_ascii())
        {
            let mut c_style = Vec::new();
            c_style.push(b'"');
            for &b in arg {
                match b {
                    b'\\' => c_style.extend_from_slice(br#"\\"#),
                    b'"' => c_style.extend_from_slice(br#"\""#),
                    b'\n' => c_style.extend_from_slice(br#"\n"#),
                    b'\r' => c_style.extend_from_slice(br#"\r"#),
                    b'\t' => c_style.extend_from_slice(br#"\t"#),
                    0 => {
                        if (flags & QA_ELIDE_NULL_BYTES) == 0 {
                            c_style.extend_from_slice(br#"\0"#)
                        }
                    }
                    _ => c_style.push(b),
                }
            }
            c_style.push(b'"');
            return c_style;
        }

        out
    }
}

impl Default for Quotearg {
    fn default() -> Self {
        Self { slots: Vec::new() }
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        self.slots.clear();
    }
}
