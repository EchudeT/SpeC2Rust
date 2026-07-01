pub struct Ialloc;

impl Ialloc {
    pub fn checked_len(len: isize) -> Option<usize> {
        usize::try_from(len).ok()
    }

    pub fn empty_vec<T>() -> Vec<T> {
        Vec::new()
    }

    pub fn with_capacity<T>(len: isize) -> Option<Vec<T>> {
        let capacity = Self::checked_len(len)?;
        Some(Vec::with_capacity(capacity))
    }

    pub fn repeat_default<T: Default + Clone>(len: isize) -> Option<Vec<T>> {
        let count = Self::checked_len(len)?;
        Some(vec![T::default(); count])
    }

    pub fn resize_with_default<T: Default + Clone>(vec: &mut Vec<T>, new_len: isize) -> Option<()> {
        let new_len = Self::checked_len(new_len)?;
        vec.resize(new_len, T::default());
        Some(())
    }

    pub fn duplicate_slice<T: Clone>(slice: &[T]) -> Vec<T> {
        slice.to_vec()
    }

    pub fn duplicate_prefix<T: Clone>(slice: &[T], len: isize) -> Option<Vec<T>> {
        let len = Self::checked_len(len)?;
        if len > slice.len() {
            return None;
        }
        Some(slice[..len].to_vec())
    }
}
