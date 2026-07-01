pub struct CCtype;

impl CCtype {
    #[inline]
    pub fn is_ascii(codepoint: u32) -> bool {
        codepoint <= 0x7f
    }

    #[inline]
    pub fn is_blank(ch: char) -> bool {
        matches!(ch, ' ' | '\t')
    }

    #[inline]
    pub fn is_control(ch: char) -> bool {
        Self::is_ascii(ch as u32) && ((ch as u32) < 0x20 || ch == '\u{7f}')
    }

    #[inline]
    pub fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    #[inline]
    pub fn is_lower(ch: char) -> bool {
        ch.is_ascii_lowercase()
    }

    #[inline]
    pub fn is_upper(ch: char) -> bool {
        ch.is_ascii_uppercase()
    }

    #[inline]
    pub fn is_alpha(ch: char) -> bool {
        ch.is_ascii_alphabetic()
    }

    #[inline]
    pub fn is_alnum(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
    }

    #[inline]
    pub fn is_xdigit(ch: char) -> bool {
        ch.is_ascii_hexdigit()
    }

    #[inline]
    pub fn is_space(ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0b}' | '\u{0c}')
    }

    #[inline]
    pub fn is_print(ch: char) -> bool {
        Self::is_ascii(ch as u32) && !Self::is_control(ch)
    }

    #[inline]
    pub fn is_graph(ch: char) -> bool {
        Self::is_print(ch) && ch != ' '
    }

    #[inline]
    pub fn is_punct(ch: char) -> bool {
        Self::is_graph(ch) && !Self::is_alnum(ch)
    }

    #[inline]
    pub fn to_lower(ch: char) -> char {
        if ch.is_ascii() {
            ch.to_ascii_lowercase()
        } else {
            ch
        }
    }

    #[inline]
    pub fn to_upper(ch: char) -> char {
        if ch.is_ascii() {
            ch.to_ascii_uppercase()
        } else {
            ch
        }
    }
}
