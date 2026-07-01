pub struct Ialloc;

impl Ialloc {
    pub fn allocate<T: Default + Clone>(len: usize) -> Vec<T> {
        vec![T::default(); len]
    }

    pub fn allocate_with<T, F>(len: usize, value: F) -> Vec<T>
    where
        F: FnMut() -> T,
    {
        std::iter::repeat_with(value).take(len).collect()
    }

    pub fn resize<T: Default + Clone>(buffer: &mut Vec<T>, new_len: usize) {
        buffer.resize(new_len, T::default());
    }

    pub fn grow_by<T: Default + Clone>(buffer: &mut Vec<T>, additional: usize) {
        buffer.resize(buffer.len().saturating_add(additional), T::default());
    }

    pub fn duplicate<T: Clone>(slice: &[T]) -> Vec<T> {
        slice.to_vec()
    }

    pub fn clear<T>(buffer: &mut Vec<T>) {
        buffer.clear();
    }

    pub fn take<T>(buffer: &mut Vec<T>) -> Vec<T> {
        std::mem::take(buffer)
    }
}
