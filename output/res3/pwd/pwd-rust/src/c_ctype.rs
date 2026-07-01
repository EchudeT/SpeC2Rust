pub struct CCtype;

impl CCtype {
    #[inline]
    pub fn is_ascii(codepoint: i32) -> bool {
        (0..=0x7f).contains(&codepoint)
    }

    #[inline]
    pub fn is_blank(codepoint: i32) -> bool {
        matches!(codepoint, 0x09 | 0x20)
    }

    #[inline]
    pub fn is_control(codepoint: i32) -> bool {
        (0..=0x1f).contains(&codepoint) || codepoint == 0x7f
    }

    #[inline]
    pub fn is_digit(codepoint: i32) -> bool {
        (b'0' as i32..=b'9' as i32).contains(&codepoint)
    }

    #[inline]
    pub fn is_lower(codepoint: i32) -> bool {
        (b'a' as i32..=b'z' as i32).contains(&codepoint)
    }

    #[inline]
    pub fn is_upper(codepoint: i32) -> bool {
        (b'A' as i32..=b'Z' as i32).contains(&codepoint)
    }

    #[inline]
    pub fn is_alpha(codepoint: i32) -> bool {
        Self::is_lower(codepoint) || Self::is_upper(codepoint)
    }

    #[inline]
    pub fn is_alnum(codepoint: i32) -> bool {
        Self::is_alpha(codepoint) || Self::is_digit(codepoint)
    }

    #[inline]
    pub fn is_space(codepoint: i32) -> bool {
        matches!(codepoint, 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x20)
    }

    #[inline]
    pub fn is_xdigit(codepoint: i32) -> bool {
        Self::is_digit(codepoint)
            || (b'a' as i32..=b'f' as i32).contains(&codepoint)
            || (b'A' as i32..=b'F' as i32).contains(&codepoint)
    }

    #[inline]
    pub fn is_graph(codepoint: i32) -> bool {
        (0x21..=0x7e).contains(&codepoint)
    }

    #[inline]
    pub fn is_print(codepoint: i32) -> bool {
        (0x20..=0x7e).contains(&codepoint)
    }

    #[inline]
    pub fn is_punct(codepoint: i32) -> bool {
        Self::is_graph(codepoint) && !Self::is_alnum(codepoint)
    }

    #[inline]
    pub fn to_lower(codepoint: i32) -> i32 {
        if Self::is_upper(codepoint) {
            codepoint + (b'a' as i32 - b'A' as i32)
        } else {
            codepoint
        }
    }

    #[inline]
    pub fn to_upper(codepoint: i32) -> i32 {
        if Self::is_lower(codepoint) {
            codepoint - (b'a' as i32 - b'A' as i32)
        } else {
            codepoint
        }
    }
}
