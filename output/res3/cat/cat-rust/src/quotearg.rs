use crate::c32isprint::C32Isprint;
use crate::c_ctype::CCtype;
use crate::c_strcasecmp::CStrcasecmp;
use crate::localcharset::Localcharset;
use std::cmp;
use std::cell::RefCell;

const INT_BITS: usize = u32::BITS as usize;
const QUOTE_BITMAP_WORDS: usize = 256 / INT_BITS;

const FLAG_ELIDE_NULL_BYTES: i32 = 1 << 0;
const FLAG_ELIDE_OUTER_QUOTES: i32 = 1 << 1;
const FLAG_SPLIT_TRIGRAPHS: i32 = 1 << 2;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: i32,
    pub quote_these_too: [u32; QUOTE_BITMAP_WORDS],
    pub left_quote: Option<Vec<u8>>,
    pub right_quote: Option<Vec<u8>>,
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
struct Slot {
    value: String,
}

#[derive(Default)]
struct SharedState {
    default_options: QuotingOptions,
    quote_options: QuotingOptions,
    slots: Vec<Slot>,
}

thread_local! {
    static QUOTEARG_STATE: RefCell<SharedState> = RefCell::new({
        let mut quote_options = QuotingOptions::default();
        quote_options.style = QuotingStyle::Locale;
        SharedState {
            default_options: QuotingOptions::default(),
            quote_options,
            slots: vec![Slot::default()],
        }
    });
}

fn nth_bit(byte: u8) -> (usize, usize) {
    let index = byte as usize;
    (index / INT_BITS, index % INT_BITS)
}

fn bitmap_get(bitmap: &[u32; QUOTE_BITMAP_WORDS], byte: u8) -> bool {
    let (word, bit) = nth_bit(byte);
    ((bitmap[word] >> bit) & 1) != 0
}

fn bitmap_set(bitmap: &mut [u32; QUOTE_BITMAP_WORDS], byte: u8, enabled: bool) -> bool {
    let (word, bit) = nth_bit(byte);
    let previous = ((bitmap[word] >> bit) & 1) != 0;
    if previous != enabled {
        bitmap[word] ^= 1u32 << bit;
    }
    previous
}

fn options_or_default(options: Option<&QuotingOptions>) -> QuotingOptions {
    options.cloned().unwrap_or_else(|| {
        QUOTEARG_STATE.with(|state| state.borrow().default_options.clone())
    })
}

fn bytes_up_to_nul(input: &[u8]) -> &[u8] {
    match input.iter().position(|&b| b == 0) {
        Some(end) => &input[..end],
        None => input,
    }
}

fn display_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub struct Quotearg {
    options: QuotingOptions,
    retained: Vec<String>,
}

impl Quotearg {
    pub fn clone_quoting_options(options: Option<&QuotingOptions>) -> QuotingOptions {
        options_or_default(options)
    }

    pub fn get_quoting_style(options: Option<&QuotingOptions>) -> QuotingStyle {
        options
            .map(|o| o.style)
            .unwrap_or_else(|| QUOTEARG_STATE.with(|state| state.borrow().default_options.style))
    }

    pub fn set_quoting_style(options: Option<&mut QuotingOptions>, style: QuotingStyle) {
        if let Some(options) = options {
            options.style = style;
        } else {
            QUOTEARG_STATE.with(|state| {
                state.borrow_mut().default_options.style = style;
            });
        }
    }

    pub fn set_char_quoting(options: Option<&mut QuotingOptions>, ch: u8, enabled: bool) -> bool {
        if let Some(options) = options {
            bitmap_set(&mut options.quote_these_too, ch, enabled)
        } else {
            QUOTEARG_STATE.with(|state| {
                bitmap_set(&mut state.borrow_mut().default_options.quote_these_too, ch, enabled)
            })
        }
    }

    pub fn set_quoting_flags(options: Option<&mut QuotingOptions>, flags: i32) -> i32 {
        if let Some(options) = options {
            let previous = options.flags;
            options.flags = flags;
            previous
        } else {
            QUOTEARG_STATE.with(|state| {
                let mut state = state.borrow_mut();
                let previous = state.default_options.flags;
                state.default_options.flags = flags;
                previous
            })
        }
    }

    pub fn set_custom_quoting(
        options: Option<&mut QuotingOptions>,
        left_quote: impl Into<Vec<u8>>,
        right_quote: impl Into<Vec<u8>>,
    ) {
        let left_quote = left_quote.into();
        let right_quote = right_quote.into();
        assert!(!left_quote.is_empty());
        assert!(!right_quote.is_empty());

        if let Some(options) = options {
            options.style = QuotingStyle::Custom;
            options.left_quote = Some(left_quote);
            options.right_quote = Some(right_quote);
        } else {
            QUOTEARG_STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.default_options.style = QuotingStyle::Custom;
                state.default_options.left_quote = Some(left_quote);
                state.default_options.right_quote = Some(right_quote);
            });
        }
    }

    pub fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions {
        assert!(style != QuotingStyle::Custom);
        QuotingOptions {
            style,
            ..QuotingOptions::default()
        }
    }

    pub fn gettext_quote(msgid: &str, style: QuotingStyle) -> Vec<u8> {
        let charset = Localcharset::locale_charset();
        if CStrcasecmp::eq_ignore_case(charset, "UTF-8") {
            if msgid.starts_with('`') {
                return vec![0xE2, 0x80, 0x98];
            }
            return vec![0xE2, 0x80, 0x99];
        }
        if CStrcasecmp::eq_ignore_case(charset, "GB18030") {
            if msgid.starts_with('`') {
                return vec![0xA1, 0xAE];
            }
            return vec![0xA1, 0xAF];
        }
        if style == QuotingStyle::CLocale {
            b"\"".to_vec()
        } else {
            b"'".to_vec()
        }
    }

    pub fn buffer_restyled(
        destination: &mut [u8],
        arg: &[u8],
        style: QuotingStyle,
        flags: i32,
        quote_these_too: Option<&[u32; QUOTE_BITMAP_WORDS]>,
        left_quote: Option<&[u8]>,
        right_quote: Option<&[u8]>,
    ) -> usize {
        fn push_byte(out: &mut Vec<u8>, byte: u8, destination: &mut [u8], written: &mut usize) {
            if *written < destination.len() {
                destination[*written] = byte;
            }
            *written += 1;
            out.push(byte);
        }

        fn push_bytes(out: &mut Vec<u8>, bytes: &[u8], destination: &mut [u8], written: &mut usize) {
            for &b in bytes {
                push_byte(out, b, destination, written);
            }
        }

        fn force_outer(style: QuotingStyle, backslash_escapes: bool) -> QuotingStyle {
            if style == QuotingStyle::ShellAlways && backslash_escapes {
                QuotingStyle::ShellEscapeAlways
            } else {
                style
            }
        }

        let mut working_style = style;
        let mut elide_outer_quotes = (flags & FLAG_ELIDE_OUTER_QUOTES) != 0;
        let mut backslash_escapes = false;
        let mut quote_string: Vec<u8> = Vec::new();
        let mut prefix: Vec<u8> = Vec::new();

        match working_style {
            QuotingStyle::CMaybe => {
                working_style = QuotingStyle::C;
                elide_outer_quotes = true;
                if !elide_outer_quotes {
                    prefix.push(b'"');
                }
                backslash_escapes = true;
                quote_string = b"\"".to_vec();
            }
            QuotingStyle::C => {
                if !elide_outer_quotes {
                    prefix.push(b'"');
                }
                backslash_escapes = true;
                quote_string = b"\"".to_vec();
            }
            QuotingStyle::Escape => {
                backslash_escapes = true;
                elide_outer_quotes = false;
            }
            QuotingStyle::Locale | QuotingStyle::CLocale | QuotingStyle::Custom => {
                let left = if working_style == QuotingStyle::Custom {
                    left_quote.unwrap_or(b"'").to_vec()
                } else {
                    Self::gettext_quote("`", working_style)
                };
                let right = if working_style == QuotingStyle::Custom {
                    right_quote.unwrap_or(b"'").to_vec()
                } else {
                    Self::gettext_quote("'", working_style)
                };
                if !elide_outer_quotes {
                    prefix.extend_from_slice(&left);
                }
                backslash_escapes = true;
                quote_string = right;
            }
            QuotingStyle::ShellEscape => {
                backslash_escapes = true;
                elide_outer_quotes = true;
                working_style = force_outer(QuotingStyle::ShellAlways, backslash_escapes);
                if !elide_outer_quotes {
                    prefix.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::Shell => {
                elide_outer_quotes = true;
                working_style = QuotingStyle::ShellAlways;
                if !elide_outer_quotes {
                    prefix.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::ShellEscapeAlways => {
                if !elide_outer_quotes {
                    backslash_escapes = true;
                }
                working_style = QuotingStyle::ShellAlways;
                if !elide_outer_quotes {
                    prefix.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::ShellAlways => {
                working_style = QuotingStyle::ShellAlways;
                if !elide_outer_quotes {
                    prefix.push(b'\'');
                }
                quote_string = b"'".to_vec();
            }
            QuotingStyle::Literal => {
                elide_outer_quotes = false;
            }
        }

        let effective_input = arg;
        let mut rendered = Vec::with_capacity(effective_input.len() + prefix.len() + quote_string.len() + 8);
        let mut written = 0usize;
        push_bytes(&mut rendered, &prefix, destination, &mut written);

        let mut encountered_single_quote = false;
        let mut all_c_and_shell_quote_compat = true;
        let mut pending_shell_escape_end = false;
        let quote_map = quote_these_too;

        let mut i = 0usize;
        while i < effective_input.len() {
            let c = effective_input[i];

            let mut c_and_shell_quote_compat = false;
            let mut escaping = false;
            let mut is_right_quote = false;

            if backslash_escapes
                && working_style != QuotingStyle::ShellAlways
                && !quote_string.is_empty()
                && i + quote_string.len() <= effective_input.len()
                && &effective_input[i..i + quote_string.len()] == quote_string.as_slice()
            {
                if elide_outer_quotes {
                    return Self::buffer_restyled(
                        destination,
                        arg,
                        force_outer(working_style, backslash_escapes),
                        flags & !FLAG_ELIDE_OUTER_QUOTES,
                        None,
                        left_quote,
                        right_quote,
                    );
                }
                is_right_quote = true;
            }

            let mut emit = vec![c];
            let mut handled = false;

            match c {
                0 => {
                    if backslash_escapes {
                        escaping = true;
                        if working_style != QuotingStyle::ShellAlways
                            && i + 1 < effective_input.len()
                            && effective_input[i + 1].is_ascii_digit()
                        {
                            emit = b"\\00".to_vec();
                        } else {
                            emit = b"\\0".to_vec();
                        }
                    } else if (flags & FLAG_ELIDE_NULL_BYTES) != 0 {
                        i += 1;
                        continue;
                    }
                    handled = true;
                }
                b'?' => {
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    if working_style == QuotingStyle::C
                        && (flags & FLAG_SPLIT_TRIGRAPHS) != 0
                        && i + 2 < effective_input.len()
                        && effective_input[i + 1] == b'?'
                        && matches!(
                            effective_input[i + 2],
                            b'!' | b'\'' | b'(' | b')' | b'-' | b'/' | b'<' | b'=' | b'>'
                        )
                    {
                        if elide_outer_quotes {
                            return Self::buffer_restyled(
                                destination,
                                arg,
                                force_outer(working_style, backslash_escapes),
                                flags & !FLAG_ELIDE_OUTER_QUOTES,
                                None,
                                left_quote,
                                right_quote,
                            );
                        }
                        emit = vec![b'?', b'"', b'"', b'?', effective_input[i + 2]];
                        i += 2;
                        handled = true;
                    }
                }
                7 => {
                    if backslash_escapes {
                        emit = b"\\a".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                8 => {
                    if backslash_escapes {
                        emit = b"\\b".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                12 => {
                    if backslash_escapes {
                        emit = b"\\f".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                b'\n' => {
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    if backslash_escapes {
                        emit = b"\\n".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                b'\r' => {
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    if backslash_escapes {
                        emit = b"\\r".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                b'\t' => {
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                    if backslash_escapes {
                        emit = b"\\t".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                11 => {
                    if backslash_escapes {
                        emit = b"\\v".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                b'\\' => {
                    if working_style == QuotingStyle::ShellAlways {
                        if elide_outer_quotes {
                            return Self::buffer_restyled(
                                destination,
                                arg,
                                force_outer(working_style, backslash_escapes),
                                flags & !FLAG_ELIDE_OUTER_QUOTES,
                                None,
                                left_quote,
                                right_quote,
                            );
                        }
                    } else if backslash_escapes && elide_outer_quotes && !quote_string.is_empty() {
                    } else if backslash_escapes {
                        emit = b"\\\\".to_vec();
                        escaping = true;
                        handled = true;
                    }
                }
                b'{' | b'}' => {
                    if effective_input.len() == 1 {
                        if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                            return Self::buffer_restyled(
                                destination,
                                arg,
                                force_outer(working_style, backslash_escapes),
                                flags & !FLAG_ELIDE_OUTER_QUOTES,
                                None,
                                left_quote,
                                right_quote,
                            );
                        }
                    }
                }
                b'#' | b'~' => {
                    if i == 0 && working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                }
                b' ' => {
                    c_and_shell_quote_compat = true;
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                }
                b'!' | b'"' | b'$' | b'&' | b'(' | b')' | b'*' | b';' | b'<' | b'=' | b'>'
                | b'[' | b'^' | b'`' | b'|' => {
                    if working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
                        return Self::buffer_restyled(
                            destination,
                            arg,
                            force_outer(working_style, backslash_escapes),
                            flags & !FLAG_ELIDE_OUTER_QUOTES,
                            None,
                            left_quote,
                            right_quote,
                        );
                    }
                }
                b'\'' => {
                    encountered_single_quote = true;
                    c_and_shell_quote_compat = true;
                    if working_style == QuotingStyle::ShellAlways {
                        if elide_outer_quotes {
                            return Self::buffer_restyled(
                                destination,
                                arg,
                                force_outer(working_style, backslash_escapes),
                                flags & !FLAG_ELIDE_OUTER_QUOTES,
                                None,
                                left_quote,
                                right_quote,
                            );
                        }
                        emit = b"'\\''".to_vec();
                        pending_shell_escape_end = false;
                        handled = true;
                    }
                }
                b'%' | b'+' | b',' | b'-' | b'.' | b'/' | b'0'..=b'9' | b':' | b'A'..=b'Z'
                | b']' | b'_' | b'a'..=b'z' => {
                    c_and_shell_quote_compat = true;
                }
                _ => {
                    if CCtype::is_print(c) {
                        c_and_shell_quote_compat = true;
                    } else if backslash_escapes {
                        emit = vec![b'\\', b'0' + (c >> 6), b'0' + ((c >> 3) & 7), b'0' + (c & 7)];
                        escaping = true;
                        handled = true;
                    } else if !C32Isprint::is_print(c as u32) {
                        c_and_shell_quote_compat = false;
                    }
                }
            }

            let quoted_too = quote_map.is_some_and(|m| {
                ((backslash_escapes && working_style != QuotingStyle::ShellAlways) || elide_outer_quotes)
                    && bitmap_get(m, c)
            });

            if (quoted_too || is_right_quote) && !handled {
                if elide_outer_quotes {
                    return Self::buffer_restyled(
                        destination,
                        arg,
                        force_outer(working_style, backslash_escapes),
                        flags & !FLAG_ELIDE_OUTER_QUOTES,
                        None,
                        left_quote,
                        right_quote,
                    );
                }
                if is_right_quote {
                    emit = vec![b'\\', c];
                } else if backslash_escapes {
                    emit = vec![b'\\', c];
                    escaping = true;
                }
            }

            if pending_shell_escape_end && !escaping {
                push_bytes(&mut rendered, b"''", destination, &mut written);
                pending_shell_escape_end = false;
            }

            if escaping && working_style == QuotingStyle::ShellAlways && !pending_shell_escape_end {
                push_bytes(&mut rendered, b"'$'", destination, &mut written);
                pending_shell_escape_end = true;
            }

            push_bytes(&mut rendered, &emit, destination, &mut written);

            if !c_and_shell_quote_compat {
                all_c_and_shell_quote_compat = false;
            }

            i += 1;
        }

        if rendered.is_empty() && working_style == QuotingStyle::ShellAlways && elide_outer_quotes {
            return Self::buffer_restyled(
                destination,
                arg,
                force_outer(working_style, backslash_escapes),
                flags & !FLAG_ELIDE_OUTER_QUOTES,
                None,
                left_quote,
                right_quote,
            );
        }

        if working_style == QuotingStyle::ShellAlways
            && !elide_outer_quotes
            && encountered_single_quote
            && all_c_and_shell_quote_compat
        {
            return Self::buffer_restyled(
                destination,
                arg,
                QuotingStyle::C,
                flags,
                quote_these_too,
                left_quote,
                right_quote,
            );
        }

        if !quote_string.is_empty() && !elide_outer_quotes {
            push_bytes(&mut rendered, &quote_string, destination, &mut written);
        }

        if written < destination.len() {
            destination[written] = 0;
        }

        written
    }

    pub fn buffer(destination: &mut [u8], arg: &[u8], options: Option<&QuotingOptions>) -> usize {
        let resolved = options_or_default(options);
        Self::buffer_restyled(
            destination,
            arg,
            resolved.style,
            resolved.flags,
            Some(&resolved.quote_these_too),
            resolved.left_quote.as_deref(),
            resolved.right_quote.as_deref(),
        )
    }

    pub fn new(arg: &[u8], options: Option<&QuotingOptions>) -> Self {
        let mut instance = Self {
            options: options_or_default(options),
            retained: Vec::new(),
        };
        let _ = instance.alloc_mem(arg, options);
        instance
    }

    pub fn alloc_mem(&mut self, arg: &[u8], options: Option<&QuotingOptions>) -> (String, usize) {
        let resolved = options_or_default(options);
        let flags = resolved.flags;
        let flags = flags;
        let mut buf = vec![0u8; arg.len().saturating_mul(4).saturating_add(32)];
        let size = Self::buffer_restyled(
            &mut buf,
            arg,
            resolved.style,
            flags,
            Some(&resolved.quote_these_too),
            resolved.left_quote.as_deref(),
            resolved.right_quote.as_deref(),
        );
        if size >= buf.len() {
            buf.resize(size + 1, 0);
            let size2 = Self::buffer_restyled(
                &mut buf,
                arg,
                resolved.style,
                flags,
                Some(&resolved.quote_these_too),
                resolved.left_quote.as_deref(),
                resolved.right_quote.as_deref(),
            );
            let rendered = display_string(&buf[..size2]);
            self.retained.push(rendered.clone());
            return (rendered, size2);
        }
        let rendered = display_string(&buf[..size]);
        self.retained.push(rendered.clone());
        (rendered, size)
    }

    pub fn n_options(n: usize, arg: &[u8], options: &QuotingOptions) -> String {
        QUOTEARG_STATE.with(|state| {
            let mut state = state.borrow_mut();
            if state.slots.len() <= n {
                state.slots.resize_with(n + 1, Slot::default);
            }

            let flags = options.flags | FLAG_ELIDE_NULL_BYTES;
            let mut buf = vec![0u8; arg.len().saturating_mul(4).saturating_add(32)];
            let mut size = Self::buffer_restyled(
                &mut buf,
                arg,
                options.style,
                flags,
                Some(&options.quote_these_too),
                options.left_quote.as_deref(),
                options.right_quote.as_deref(),
            );
            if size >= buf.len() {
                buf.resize(size + 1, 0);
                size = Self::buffer_restyled(
                    &mut buf,
                    arg,
                    options.style,
                    flags,
                    Some(&options.quote_these_too),
                    options.left_quote.as_deref(),
                    options.right_quote.as_deref(),
                );
            }
            state.slots[n].value = display_string(&buf[..size]);
            state.slots[n].value.clone()
        })
    }

    pub fn n(n: usize, arg: &[u8]) -> String {
        let options = QUOTEARG_STATE.with(|state| state.borrow().default_options.clone());
        Self::n_options(n, bytes_up_to_nul(arg), &options)
    }

    pub fn n_mem(n: usize, arg: &[u8]) -> String {
        let options = QUOTEARG_STATE.with(|state| state.borrow().default_options.clone());
        Self::n_options(n, arg, &options)
    }

    pub fn quoted_arg(arg: &[u8]) -> String {
        Self::n(0, arg)
    }

    pub fn mem(arg: &[u8]) -> String {
        Self::n_mem(0, arg)
    }

    pub fn n_style(n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let options = Self::quoting_options_from_style(style);
        Self::n_options(n, bytes_up_to_nul(arg), &options)
    }

    pub fn n_style_mem(n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let options = Self::quoting_options_from_style(style);
        Self::n_options(n, arg, &options)
    }

    pub fn style(style: QuotingStyle, arg: &[u8]) -> String {
        Self::n_style(0, style, arg)
    }

    pub fn style_mem(style: QuotingStyle, arg: &[u8]) -> String {
        Self::n_style_mem(0, style, arg)
    }

    pub fn char_mem(arg: &[u8], ch: u8) -> String {
        let mut options = QUOTEARG_STATE.with(|state| state.borrow().default_options.clone());
        Self::set_char_quoting(Some(&mut options), ch, true);
        Self::n_options(0, arg, &options)
    }

    pub fn char(arg: &[u8], ch: u8) -> String {
        Self::char_mem(bytes_up_to_nul(arg), ch)
    }

    pub fn colon(arg: &[u8]) -> String {
        Self::char(arg, b':')
    }

    pub fn colon_mem(arg: &[u8]) -> String {
        Self::char_mem(arg, b':')
    }

    pub fn n_style_colon(n: usize, style: QuotingStyle, arg: &[u8]) -> String {
        let mut options = Self::quoting_options_from_style(style);
        Self::set_char_quoting(Some(&mut options), b':', true);
        Self::n_options(n, bytes_up_to_nul(arg), &options)
    }

    pub fn n_custom(n: usize, left_quote: &[u8], right_quote: &[u8], arg: &[u8]) -> String {
        Self::n_custom_mem(n, left_quote, right_quote, bytes_up_to_nul(arg))
    }

    pub fn n_custom_mem(
        n: usize,
        left_quote: &[u8],
        right_quote: &[u8],
        arg: &[u8],
    ) -> String {
        let mut options = QUOTEARG_STATE.with(|state| state.borrow().default_options.clone());
        Self::set_custom_quoting(
            Some(&mut options),
            left_quote.to_vec(),
            right_quote.to_vec(),
        );
        Self::n_options(n, arg, &options)
    }

    pub fn custom(left_quote: &[u8], right_quote: &[u8], arg: &[u8]) -> String {
        Self::n_custom(0, left_quote, right_quote, arg)
    }

    pub fn custom_mem(left_quote: &[u8], right_quote: &[u8], arg: &[u8]) -> String {
        Self::n_custom_mem(0, left_quote, right_quote, arg)
    }

    pub fn quote_n_mem(n: usize, arg: &[u8]) -> String {
        let options = QUOTEARG_STATE.with(|state| state.borrow().quote_options.clone());
        Self::n_options(n, arg, &options)
    }

    pub fn quote_mem(arg: &[u8]) -> String {
        Self::quote_n_mem(0, arg)
    }

    pub fn quote_n(n: usize, arg: &[u8]) -> String {
        Self::quote_n_mem(n, bytes_up_to_nul(arg))
    }

    pub fn quote_default(arg: &[u8]) -> String {
        Self::quote_n(0, arg)
    }

    pub fn main_root_quoting_options_01() -> QuotingOptions {
        QUOTEARG_STATE.with(|state| state.borrow().default_options.clone())
    }

    pub fn quoting_options() -> QuotingOptions {
        Self::main_root_quoting_options_01()
    }

    pub fn n_07(n: usize, arg: &[u8]) -> String {
        Self::n(n, arg)
    }

    pub fn main_root_quote_n_10(n: usize, arg: &[u8]) -> String {
        Self::quote_n(n, arg)
    }

    pub fn style_13(style: QuotingStyle, arg: &[u8]) -> String {
        Self::style(style, arg)
    }

    pub fn colon_11(arg: &[u8]) -> String {
        Self::quoted_arg(arg)
    }

    pub fn custom_12(left_quote: &[u8], right_quote: &[u8], arg: &[u8]) -> String {
        Self::custom(left_quote, right_quote, arg)
    }

    pub fn main_root_quoting_options_02() -> QuotingOptions {
        QUOTEARG_STATE.with(|state| state.borrow().quote_options.clone())
    }
}

impl Drop for Quotearg {
    fn drop(&mut self) {
        self.retained.clear();
        QUOTEARG_STATE.with(|state| {
            let mut state = state.borrow_mut();
            let keep = cmp::max(1, state.slots.len().min(1));
            state.slots.truncate(keep);
            if state.slots.is_empty() {
                state.slots.push(Slot::default());
            } else {
                state.slots[0].value.clear();
            }
        });
        let _ = &self.options;
    }
}
