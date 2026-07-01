use crate::strerror::Strerror;

/// Provides GNU-style error-string overrides for errno values whose standard
/// platform messages are known to be inconsistent, missing, or less useful.
///
/// Unlike the original C helper that returned a nullable pointer to either an
/// override string or `NULL`, this Rust API returns `Option<String>`.
/// - `Some(message)` means an override is available for `errnum`.
/// - `None` means callers should fall back to the platform/default strerror.
pub struct StrerrorOverride;

impl StrerrorOverride {
    /// Returns an override message for `errnum` when this module has a
    /// canonical replacement; otherwise returns `None`.
    ///
    /// The current Rust migration keeps this conservative and only supplies
    /// a stable fallback when the standard message resolution yields no useful
    /// text.
    pub fn message(errnum: i32) -> Option<String> {
        let message = Strerror::message(errnum);
        if message.trim().is_empty() || message.starts_with("Unknown error") {
            Some(format!("Error {}", errnum))
        } else {
            None
        }
    }
}
