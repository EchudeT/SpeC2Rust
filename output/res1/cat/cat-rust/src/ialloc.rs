use std::borrow::Borrow;
use std::mem;

/// Safe allocation-size arithmetic and duplication helpers inspired by
/// gnulib's `ialloc` inline utilities.
pub struct Ialloc;

impl Ialloc {
    /// Computes `count * size`, returning `None` on overflow.
    pub fn checked_array_size(count: usize, size: usize) -> Option<usize> {
        count.checked_mul(size)
    }

    /// Computes `base + extra`, returning `None` on overflow.
    pub fn checked_size_add(base: usize, extra: usize) -> Option<usize> {
        base.checked_add(extra)
    }

    /// Computes the byte count needed for `count` values of `T`.
    pub fn bytes_for<T>(count: usize) -> Option<usize> {
        Self::checked_array_size(count, mem::size_of::<T>())
    }

    /// Allocates a zero-filled byte buffer of `size` bytes.
    pub fn zeroed_bytes(size: usize) -> Vec<u8> {
        vec![0; size]
    }

    /// Allocates a zero-filled byte buffer large enough for `count` values of `T`.
    pub fn zeroed_bytes_for<T>(count: usize) -> Option<Vec<u8>> {
        Self::bytes_for::<T>(count).map(Self::zeroed_bytes)
    }

    /// Clones a borrowed slice-like value into an owned vector.
    pub fn duplicate_slice<T: Clone, S>(value: S) -> Vec<T>
    where
        S: Borrow<[T]>,
    {
        value.borrow().to_vec()
    }

    /// Clones the first `count` elements of a slice-like value into an owned vector.
    pub fn duplicate_prefix<T: Clone, S>(value: S, count: usize) -> Vec<T>
    where
        S: Borrow<[T]>,
    {
        let slice = value.borrow();
        slice[..slice.len().min(count)].to_vec()
    }

    /// Duplicates a string slice into an owned `String`.
    pub fn duplicate_str(value: &str) -> String {
        value.to_owned()
    }

    /// Duplicates at most `byte_count` bytes from a string slice, truncating
    /// to a valid UTF-8 boundary.
    pub fn duplicate_str_prefix(value: &str, byte_count: usize) -> String {
        if byte_count >= value.len() {
            return value.to_owned();
        }

        let mut end = 0;
        for (index, ch) in value.char_indices() {
            let next = index + ch.len_utf8();
            if next > byte_count {
                break;
            }
            end = next;
        }
        value[..end].to_owned()
    }
}
