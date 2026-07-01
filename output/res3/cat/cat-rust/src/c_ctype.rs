pub struct CCtype;

impl CCtype {
    #[inline]
    pub fn is_ascii(byte: u8) -> bool {
        byte <= 0x7f
    }

    #[inline]
    pub fn is_blank(byte: u8) -> bool {
        matches!(byte, b' ' | b'\t')
    }

    #[inline]
    pub fn is_control(byte: u8) -> bool {
        byte <= 0x1f || byte == 0x7f
    }

    #[inline]
    pub fn is_digit(byte: u8) -> bool {
        byte.is_ascii_digit()
    }

    #[inline]
    pub fn is_lower(byte: u8) -> bool {
        byte.is_ascii_lowercase()
    }

    #[inline]
    pub fn is_upper(byte: u8) -> bool {
        byte.is_ascii_uppercase()
    }

    #[inline]
    pub fn is_alpha(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    #[inline]
    pub fn is_alnum(byte: u8) -> bool {
        byte.is_ascii_alphanumeric()
    }

    #[inline]
    pub fn is_xdigit(byte: u8) -> bool {
        byte.is_ascii_hexdigit()
    }

    #[inline]
    pub fn is_space(byte: u8) -> bool {
        byte.is_ascii_whitespace()
    }

    #[inline]
    pub fn is_print(byte: u8) -> bool {
        matches!(byte, 0x20..=0x7e)
    }

    #[inline]
    pub fn is_graph(byte: u8) -> bool {
        matches!(byte, 0x21..=0x7e)
    }

    #[inline]
    pub fn is_punct(byte: u8) -> bool {
        Self::is_graph(byte) && !Self::is_alnum(byte)
    }

    #[inline]
    pub fn to_lower(byte: u8) -> u8 {
        byte.to_ascii_lowercase()
    }

    #[inline]
    pub fn to_upper(byte: u8) -> u8 {
        byte.to_ascii_uppercase()
    }

    #[inline]
    pub fn is_ascii_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_ascii).unwrap_or(false)
    }

    #[inline]
    pub fn is_blank_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_blank).unwrap_or(false)
    }

    #[inline]
    pub fn is_control_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_control).unwrap_or(false)
    }

    #[inline]
    pub fn is_digit_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_digit).unwrap_or(false)
    }

    #[inline]
    pub fn is_lower_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_lower).unwrap_or(false)
    }

    #[inline]
    pub fn is_upper_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_upper).unwrap_or(false)
    }

    #[inline]
    pub fn is_alpha_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_alpha).unwrap_or(false)
    }

    #[inline]
    pub fn is_alnum_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_alnum).unwrap_or(false)
    }

    #[inline]
    pub fn is_xdigit_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_xdigit).unwrap_or(false)
    }

    #[inline]
    pub fn is_space_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_space).unwrap_or(false)
    }

    #[inline]
    pub fn is_print_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_print).unwrap_or(false)
    }

    #[inline]
    pub fn is_graph_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_graph).unwrap_or(false)
    }

    #[inline]
    pub fn is_punct_i32(value: i32) -> bool {
        u8::try_from(value).map(Self::is_punct).unwrap_or(false)
    }

    #[inline]
    pub fn to_lower_i32(value: i32) -> i32 {
        u8::try_from(value)
            .map(Self::to_lower)
            .map(i32::from)
            .unwrap_or(value)
    }

    #[inline]
    pub fn to_upper_i32(value: i32) -> i32 {
        u8::try_from(value)
            .map(Self::to_upper)
            .map(i32::from)
            .unwrap_or(value)
    }
}
