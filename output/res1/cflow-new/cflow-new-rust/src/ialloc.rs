pub struct Ialloc;

impl Ialloc {
    pub fn alloc_default<T: Default + Clone>(count: usize) -> Option<Vec<T>> {
        let mut values = Vec::new();
        values.try_reserve_exact(count).ok()?;
        values.resize(count, T::default());
        Some(values)
    }

    pub fn alloc_with<T: Clone>(count: usize, value: T) -> Option<Vec<T>> {
        let mut values = Vec::new();
        values.try_reserve_exact(count).ok()?;
        values.resize(count, value);
        Some(values)
    }

    pub fn resize_default<T: Default + Clone>(buffer: &mut Vec<T>, new_len: usize) -> bool {
        if new_len > buffer.len() {
            let additional = new_len - buffer.len();
            if buffer.try_reserve_exact(additional).is_err() {
                return false;
            }
        }
        buffer.resize(new_len, T::default());
        true
    }

    pub fn resize_with<T: Clone>(buffer: &mut Vec<T>, new_len: usize, value: T) -> bool {
        if new_len > buffer.len() {
            let additional = new_len - buffer.len();
            if buffer.try_reserve_exact(additional).is_err() {
                return false;
            }
        }
        buffer.resize(new_len, value);
        true
    }

    pub fn duplicate_slice<T: Clone>(slice: &[T]) -> Option<Vec<T>> {
        let mut values = Vec::new();
        values.try_reserve_exact(slice.len()).ok()?;
        values.extend_from_slice(slice);
        Some(values)
    }

    pub fn duplicate_str(text: &str) -> Option<String> {
        let mut result = String::new();
        result.try_reserve(text.len()).ok()?;
        result.push_str(text);
        Some(result)
    }
}
