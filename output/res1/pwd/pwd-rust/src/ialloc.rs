use std::collections::TryReserveError;

pub struct Ialloc;

impl Ialloc {
    pub fn try_vec_with_capacity<T>(count: u64) -> Result<Vec<T>, TryReserveError> {
        let mut vec = Vec::new();
        let capacity = usize::try_from(count).unwrap_or(usize::MAX);
        vec.try_reserve_exact(capacity)?;
        Ok(vec)
    }

    pub fn try_zeroed_vec(len: u64) -> Result<Vec<u8>, TryReserveError> {
        let len = usize::try_from(len).unwrap_or(usize::MAX);
        let mut vec = Vec::new();
        vec.try_reserve_exact(len)?;
        vec.resize(len, 0);
        Ok(vec)
    }

    pub fn try_realloc_vec<T>(
        mut vec: Vec<T>,
        new_capacity: u64,
    ) -> Result<Vec<T>, TryReserveError> {
        let target = usize::try_from(new_capacity).unwrap_or(usize::MAX);
        if target > vec.capacity() {
            vec.try_reserve_exact(target - vec.capacity())?;
        }
        Ok(vec)
    }

    pub fn try_realloc_array<T: Clone>(
        mut values: Vec<T>,
        new_len: u64,
        fill: T,
    ) -> Result<Vec<T>, TryReserveError> {
        let new_len = usize::try_from(new_len).unwrap_or(usize::MAX);
        if new_len > values.len() {
            values.try_reserve_exact(new_len - values.len())?;
            values.resize(new_len, fill);
        } else {
            values.truncate(new_len);
        }
        Ok(values)
    }
}
