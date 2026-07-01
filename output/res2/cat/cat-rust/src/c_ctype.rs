#![allow(clippy::manual_is_ascii_check)]

/// Locale-independent ASCII character classification and case conversion.
///
/// This module mirrors the intent of GNU `c-ctype`: all operations are based on
/// the fixed ASCII character set rather than the current locale.
pub struct CCtype;

impl CCtype {
    /// Returns `true` if `ch` is a 7-bit ASCII character.
    #[inline]
    pub fn is_ascii(ch: char) -> bool {
        ch.is_ascii()
    }

    /// Returns `true` if `byte` is a 7-bit ASCII value.
    #[inline]
    pub fn is_ascii_byte(byte: u8) -> bool {
        byte <= 0x7f
    }

    /// Returns `true` if `ch` is an ASCII alphabetic character.
    #[inline]
    pub fn is_alpha(ch: char) -> bool {
        ch.is_ascii_alphabetic()
    }

    /// Returns `true` if `ch` is an ASCII uppercase letter.
    #[inline]
    pub fn is_upper(ch: char) -> bool {
        ch.is_ascii_uppercase()
    }

    /// Returns `true` if `ch` is an ASCII lowercase letter.
    #[inline]
    pub fn is_lower(ch: char) -> bool {
        ch.is_ascii_lowercase()
    }

    /// Returns `true` if `ch` is an ASCII decimal digit.
    #[inline]
    pub fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    /// Returns `true` if `ch` is an ASCII hexadecimal digit.
    #[inline]
    pub fn is_xdigit(ch: char) -> bool {
        ch.is_ascii_hexdigit()
    }

    /// Returns `true` if `ch` is an ASCII alphanumeric character.
    #[inline]
    pub fn is_alnum(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
    }

    /// Returns `true` if `ch` is an ASCII blank character: space or horizontal tab.
    #[inline]
    pub fn is_blank(ch: char) -> bool {
        matches!(ch, ' ' | '\t')
    }

    /// Returns `true` if `ch` is an ASCII whitespace character.
    #[inline]
    pub fn is_space(ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\n' | '\r' | '\x0c' | '\x0b')
    }

    /// Returns `true` if `ch` is an ASCII control character or DEL.
    #[inline]
    pub fn is_cntrl(ch: char) -> bool {
        ch.is_ascii_control()
    }

    /// Returns `true` if `ch` is an ASCII printable character excluding space.
    #[inline]
    pub fn is_graph(ch: char) -> bool {
        ch.is_ascii_graphic()
    }

    /// Returns `true` if `ch` is an ASCII printable character including space.
    #[inline]
    pub fn is_print(ch: char) -> bool {
        ch.is_ascii_graphic() || ch == ' '
    }

    /// Returns `true` if `ch` is an ASCII punctuation character.
    #[inline]
    pub fn is_punct(ch: char) -> bool {
        ch.is_ascii_punctuation()
    }

    /// Converts an ASCII uppercase letter to lowercase.
    ///
    /// Non-ASCII characters and non-uppercase ASCII characters are returned unchanged.
    #[inline]
    pub fn to_lower(ch: char) -> char {
        ch.to_ascii_lowercase()
    }

    /// Converts an ASCII lowercase letter to uppercase.
    ///
    /// Non-ASCII characters and non-lowercase ASCII characters are returned unchanged.
    #[inline]
    pub fn to_upper(ch: char) -> char {
        ch.to_ascii_uppercase()
    }

    /// Returns the code point with its ASCII uppercase form applied.
    ///
    /// This is useful when working with byte-oriented or integer-oriented ports of
    /// the original C helpers.
    #[inline]
    pub fn fold_ascii_upper(code: u32) -> u32 {
        char::from_u32(code)
            .map(Self::to_upper)
            .map(|c| c as u32)
            .unwrap_or(code)
    }

    /// Returns the code point with its ASCII lowercase form applied.
    ///
    /// This is useful when working with byte-oriented or integer-oriented ports of
    /// the original C helpers.
    #[inline]
    pub fn fold_ascii_lower(code: u32) -> u32 {
        char::from_u32(code)
            .map(Self::to_lower)
            .map(|c| c as u32)
            .unwrap_or(code)
    }
}
