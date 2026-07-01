use crate::c_ctype::CCtype;
use std::cmp::Ordering;

/// Utilities for ASCII/C-locale style case-insensitive string comparison.
///
/// This mirrors the behavior of GNU-style `c_strcasecmp`: comparison is done
/// byte-by-byte after applying C-locale lowercase folding, so only ASCII
/// letters are case-folded.
pub struct CStrcasecmp;

impl CStrcasecmp {
    /// Compare two strings using C-locale ASCII-only case folding.
    ///
    /// Returns:
    /// - `Ordering::Equal` if the strings are equal ignoring ASCII case
    /// - `Ordering::Less` if `left` sorts before `right`
    /// - `Ordering::Greater` if `left` sorts after `right`
    ///
    /// Comparison is performed on the UTF-8 bytes of the Rust strings, matching
    /// the original byte-oriented C behavior rather than Unicode case folding.
    pub fn compare(left: &str, right: &str) -> Ordering {

        let mut left_bytes = left.as_bytes().iter().copied();
        let mut right_bytes = right.as_bytes().iter().copied();

        loop {
            let c1 = CCtype::to_lower(left_bytes.next().unwrap_or(0));
            let c2 = CCtype::to_lower(right_bytes.next().unwrap_or(0));

            if c1 == 0 {
                return c1.cmp(&c2);
            }

            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
    }

    /// Returns `true` when two strings are equal under C-locale ASCII-only
    /// case-insensitive comparison.
    pub fn eq_ignore_case(left: &str, right: &str) -> bool {
        Self::compare(left, right) == Ordering::Equal
    }
}
