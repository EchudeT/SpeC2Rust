use std::collections::TryReserveError;

/// Replacement-style reallocation behavior modeled in an idiomatic Rust API.
///
/// This port preserves the observable behavior of `gnu/realloc.c`:
/// - `None` input acts like allocation,
/// - a zero-size request frees the existing buffer and returns `None`,
/// - oversize or allocation failure returns `None`,
/// - otherwise the existing buffer is resized and returned.
pub struct Realloc;

impl Realloc {
    /// Resize an optional owned byte buffer to `new_size`.
    ///
    /// The buffer contents are preserved according to `Vec<u8>` resize
    /// semantics; newly added bytes are zero-initialized.
    pub fn rpl_realloc(buffer: Option<Vec<u8>>, new_size: usize) -> Option<Vec<u8>> {
        match buffer {
            None => {
                if Self::oversized(new_size) {
                    return None;
                }
                Self::allocate(new_size)
            }
            Some(mut buffer) => {
                if new_size == 0 {
                    return None;
                }

                if Self::oversized(new_size) {
                    return None;
                }

                if Self::try_resize(&mut buffer, new_size).is_err() {
                    return None;
                }

                Some(buffer)
            }
        }
    }

    fn allocate(size: usize) -> Option<Vec<u8>> {
        let mut buffer = Vec::new();
        if Self::try_resize(&mut buffer, size).is_err() {
            return None;
        }
        Some(buffer)
    }

    fn try_resize(buffer: &mut Vec<u8>, new_size: usize) -> Result<(), TryReserveError> {
        if new_size > buffer.len() {
            buffer.try_reserve_exact(new_size - buffer.len())?;
        }
        buffer.resize(new_size, 0);
        Ok(())
    }

    fn oversized(size: usize) -> bool {
        size == usize::MAX
    }
}
