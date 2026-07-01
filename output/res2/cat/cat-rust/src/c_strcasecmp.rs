use crate::c_ctype::CCtype;
use std::cmp::Ordering;

/// ASCII/C-locale-oriented case-insensitive string comparison.
pub struct CStrcasecmp;

impl CStrcasecmp {
    /// Compares two strings using byte-wise ASCII lowercase folding.
    ///
    /// This mirrors the behavior of the original C helper: each byte is folded
    /// with C-locale lowercase rules, comparison stops at the first difference,
    /// and the result is expressed as an ordering.
    pub fn compare(s1: &str, s2: &str) -> Ordering {
        if s1.as_ptr() == s2.as_ptr() && s1.len() == s2.len() {
            return Ordering::Equal;
        }

        let mut left = s1.bytes();
        let mut right = s2.bytes();

        loop {
            let c1 = left
                .next()
                .map(|b| CCtype::fold_ascii_upper(b as u32))
                .unwrap_or(0);
            let c2 = right
                .next()
                .map(|b| CCtype::fold_ascii_upper(b as u32))
                .unwrap_or(0);

            if c1 == 0 {
                return c1.cmp(&c2);
            }

            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
    }

    /// Returns true when two strings compare equal under ASCII/C-locale
    /// case-insensitive comparison.
    pub fn eq_ignore_case(s1: &str, s2: &str) -> bool {
        Self::compare(s1, s2) == Ordering::Equal
    }
}
