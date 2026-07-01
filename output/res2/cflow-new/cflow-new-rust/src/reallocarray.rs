use crate::realloc::Realloc;

/// Overflow-checked array reallocation utilities.
pub struct Reallocarray;

impl Reallocarray {
    /// Reallocate storage for `nmemb` elements of `size` bytes each.
    ///
    /// Returns `None` if `nmemb * size` overflows or if the underlying
    /// reallocation fails. Otherwise returns the resized buffer.
    pub fn realloc_array(
        buffer: Option<Vec<u8>>,
        nmemb: usize,
        size: usize,
    ) -> Option<Vec<u8>> {
        let nbytes = nmemb.checked_mul(size)?;
        Realloc::rpl_realloc(buffer, nbytes)
    }
}
