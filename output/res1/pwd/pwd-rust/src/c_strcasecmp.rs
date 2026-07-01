use crate::c_ctype::CCtype;
use std::cmp::Ordering;

/// ASCII-oriented, case-insensitive string comparison helpers.
///
/// This mirrors the behavior of the original `c_strcasecmp` logic while using
/// Rust-style APIs: inputs are borrowed byte strings or `&str`, case folding is
/// ASCII-only, and ordering is determined byte-by-byte after lowercasing.
///
/// The integer result follows the traditional comparison convention:
/// - `0` if equal ignoring ASCII case,
/// - a negative value if the left side sorts before the right side,
/// - a positive value if the left side sorts after the right side.
pub struct CStrcasecmp;

impl CStrcasecmp {
    /// Compare two byte slices as NUL-terminated C strings, using ASCII-only
    /// case-insensitive comparison.
    ///
    /// Comparison stops at the first `0` byte in either input, matching C
    /// string semantics. If no interior `0` is present, the full slice is used.
    pub fn compare_bytes(left: &[u8], right: &[u8]) -> i32 {
        let mut i = 0usize;

        loop {
            let c1 = lower_c_byte(c_string_byte(left, i));
            let c2 = lower_c_byte(c_string_byte(right, i));

            if c1 == 0 || c1 != c2 {
                return i32::from(c1) - i32::from(c2);
            }

            i += 1;
        }
    }

    /// Compare two Rust strings using ASCII-only case-insensitive ordering.
    ///
    /// Since `&str` cannot contain implicit terminators, the full string
    /// contents are compared.
    pub fn compare(left: &str, right: &str) -> i32 {
        Self::compare_bytes(left.as_bytes(), right.as_bytes())
    }

    /// Compare two inputs and return a Rust `Ordering`.
    pub fn ordering(left: &str, right: &str) -> Ordering {
        Self::compare(left, right).cmp(&0)
    }

    /// Return whether two strings are equal under ASCII-only case folding.
    pub fn eq(left: &str, right: &str) -> bool {
        Self::compare(left, right) == 0
    }
}

fn c_string_byte(bytes: &[u8], index: usize) -> u8 {
    bytes.get(index).copied().unwrap_or(0)
}

fn lower_c_byte(byte: u8) -> u8 {
    let ch = char::from(byte);
    let lowered = CCtype::to_lower(ch);
    let code = lowered as u32;
    u8::try_from(code).unwrap_or(byte)
}
