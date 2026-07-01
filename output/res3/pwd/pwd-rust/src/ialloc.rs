pub struct Ialloc;

impl Ialloc {
    pub fn checked_len<T>(count: usize) -> Option<usize> {
        count.checked_mul(std::mem::size_of::<T>())
    }

    pub fn with_capacity<T>(count: usize) -> Vec<T> {
        Vec::with_capacity(count)
    }

    pub fn zeroed<T: Default + Clone>(count: usize) -> Vec<T> {
        vec![T::default(); count]
    }

    pub fn duplicate<T: Clone>(slice: &[T]) -> Vec<T> {
        slice.to_vec()
    }

    pub fn duplicate_with_extra_default<T: Default + Clone>(slice: &[T], extra: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(slice.len().saturating_add(extra));
        out.extend_from_slice(slice);
        out.resize(slice.len().saturating_add(extra), T::default());
        out
    }

    pub fn grow_with_default<T: Default + Clone>(buffer: &mut Vec<T>, new_len: usize) {
        if new_len > buffer.len() {
            buffer.resize(new_len, T::default());
        } else {
            buffer.truncate(new_len);
        }
    }

    pub fn reserve_more<T>(buffer: &mut Vec<T>, additional: usize) {
        buffer.reserve(additional);
    }

    pub fn shrink_to_fit<T>(buffer: &mut Vec<T>) {
        buffer.shrink_to_fit();
    }
}
