use crate::c_ctype::CCtype;

/// ASCII-oriented, locale-independent case-insensitive string comparison.
///
/// This mirrors the behavior of GNU `c_strcasecmp`: bytes are compared after
/// applying C-locale lowercase conversion, stopping at the first difference or
/// at the terminating NUL-equivalent end of the Rust string byte slices.
///
/// The comparison is byte-based rather than Unicode case folding.
pub struct CStrcasecmp;

impl CStrcasecmp {
    /// Compare two strings case-insensitively using C-locale ASCII rules.
    ///
    /// Returns:
    /// - a negative value if `left < right`
    /// - zero if they are equal under ASCII case-insensitive comparison
    /// - a positive value if `left > right`
    pub fn compare(left: &str, right: &str) -> i32 {
        Self::compare_bytes(left.as_bytes(), right.as_bytes())
    }

    /// Compare two byte strings case-insensitively using C-locale ASCII rules.
    ///
    /// This is useful when callers need C-string-like byte semantics without
    /// requiring UTF-8 input.
    pub fn compare_bytes(left: &[u8], right: &[u8]) -> i32 {

        let mut i = 0usize;

        loop {
            let c1 = Self::lower_byte(left.get(i).copied().unwrap_or(0));
            let c2 = Self::lower_byte(right.get(i).copied().unwrap_or(0));

            if c1 == 0 || c1 != c2 {
                return c1 as i32 - c2 as i32;
            }

            i += 1;
        }
    }

    fn lower_byte(byte: u8) -> u8 {
        CCtype::to_lower(byte as i32) as u8
    }
}
