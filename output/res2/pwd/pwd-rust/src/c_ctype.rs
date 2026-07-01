pub struct CCtype;

impl CCtype {
    #[inline]
    pub fn is_ascii(c: char) -> bool {
        c.is_ascii()
    }

    #[inline]
    pub fn is_blank(c: char) -> bool {
        matches!(c, ' ' | '\t')
    }

    #[inline]
    pub fn is_control(c: char) -> bool {
        c.is_ascii_control()
    }

    #[inline]
    pub fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    #[inline]
    pub fn is_lower(c: char) -> bool {
        c.is_ascii_lowercase()
    }

    #[inline]
    pub fn is_upper(c: char) -> bool {
        c.is_ascii_uppercase()
    }

    #[inline]
    pub fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    #[inline]
    pub fn is_alnum(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    #[inline]
    pub fn is_xdigit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    #[inline]
    pub fn is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }

    #[inline]
    pub fn is_print(c: char) -> bool {
        c.is_ascii_graphic() || c == ' '
    }

    #[inline]
    pub fn is_graph(c: char) -> bool {
        c.is_ascii_graphic()
    }

    #[inline]
    pub fn is_punct(c: char) -> bool {
        c.is_ascii_punctuation()
    }

    #[inline]
    pub fn to_lower(c: char) -> char {
        c.to_ascii_lowercase()
    }

    #[inline]
    pub fn to_upper(c: char) -> char {
        c.to_ascii_uppercase()
    }
}
