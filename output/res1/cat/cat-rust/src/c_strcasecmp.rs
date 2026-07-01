use crate::c_ctype::CCtype;

/// ASCII/C-locale case-insensitive string comparison utilities.
pub struct CStrcasecmp;

impl CStrcasecmp {
    /// Compare two strings lexically using C-locale lowercase folding.
    ///
    /// This mirrors the behavior of `c_strcasecmp`: bytes are compared from
    /// left to right after applying `c_tolower`-style folding, stopping at the
    /// first difference or the first NUL byte. The return value is negative,
    /// zero, or positive according to lexical ordering.
    pub fn compare(s1: &str, s2: &str) -> i32 {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();

        if b1 == b2 {
            return 0;
        }

        let mut i = 0usize;

        loop {
            let c1_raw = b1.get(i).copied().unwrap_or(0);
            let c2_raw = b2.get(i).copied().unwrap_or(0);

            let c1 = Self::to_lower(c1_raw);
            let c2 = Self::to_lower(c2_raw);

            if c1 == 0 || c1 != c2 {
                return i32::from(c1) - i32::from(c2);
            }

            i += 1;
        }
    }

    fn to_lower(byte: u8) -> u8 {
        let value = byte as i32;
        let lowered = CCtype::c_tolower(value);
        u8::try_from(lowered).unwrap_or(byte)
    }
}
