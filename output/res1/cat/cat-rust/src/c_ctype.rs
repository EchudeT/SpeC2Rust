pub struct CCtype;

impl CCtype {
    #[inline]
    pub const fn is_ascii(c: i32) -> bool {
        matches!(c, 0x00..=0x7f)
    }

    #[inline]
    pub const fn is_blank(c: i32) -> bool {
        matches!(c, 0x20 | 0x09)
    }

    #[inline]
    pub const fn is_control(c: i32) -> bool {
        matches!(c, 0x00..=0x1f | 0x7f)
    }

    #[inline]
    pub const fn is_digit(c: i32) -> bool {
        matches!(c, 0x30..=0x39)
    }

    #[inline]
    pub const fn is_lower(c: i32) -> bool {
        matches!(c, 0x61..=0x7a)
    }

    #[inline]
    pub const fn is_upper(c: i32) -> bool {
        matches!(c, 0x41..=0x5a)
    }

    #[inline]
    pub const fn is_alpha(c: i32) -> bool {
        Self::is_lower(c) || Self::is_upper(c)
    }

    #[inline]
    pub const fn is_alnum(c: i32) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    #[inline]
    pub const fn is_space(c: i32) -> bool {
        matches!(c, 0x20 | 0x09..=0x0d)
    }

    #[inline]
    pub const fn is_xdigit(c: i32) -> bool {
        Self::is_digit(c) || matches!(c, 0x41..=0x46 | 0x61..=0x66)
    }

    #[inline]
    pub const fn is_punct(c: i32) -> bool {
        matches!(
            c,
            0x21..=0x2f | 0x3a..=0x40 | 0x5b..=0x60 | 0x7b..=0x7e
        )
    }

    #[inline]
    pub const fn is_graph(c: i32) -> bool {
        Self::is_alnum(c) || Self::is_punct(c)
    }

    #[inline]
    pub const fn is_print(c: i32) -> bool {
        c == 0x20 || Self::is_graph(c)
    }

    #[inline]
    pub const fn to_lower(c: i32) -> i32 {
        if Self::is_upper(c) {
            c - 0x41 + 0x61
        } else {
            c
        }
    }

    #[inline]
    pub const fn to_upper(c: i32) -> i32 {
        if Self::is_lower(c) {
            c - 0x61 + 0x41
        } else {
            c
        }
    }
}
