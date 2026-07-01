pub struct Xsize;

impl Xsize {
    pub const fn add(a: usize, b: usize) -> Option<usize> {
        a.checked_add(b)
    }

    pub const fn mul(a: usize, b: usize) -> Option<usize> {
        a.checked_mul(b)
    }

    pub const fn add_mul(base: usize, count: usize, size: usize) -> Option<usize> {
        match count.checked_mul(size) {
            Some(product) => base.checked_add(product),
            None => None,
        }
    }

    pub const fn two_times(value: usize) -> Option<usize> {
        value.checked_mul(2)
    }

    pub const fn with_header(header: usize, payload_len: usize) -> Option<usize> {
        header.checked_add(payload_len)
    }

    pub const fn array_bytes<T>(count: usize) -> Option<usize> {
        count.checked_mul(std::mem::size_of::<T>())
    }

    pub const fn object_plus_array<T>(prefix: usize, count: usize) -> Option<usize> {
        match count.checked_mul(std::mem::size_of::<T>()) {
            Some(array_bytes) => prefix.checked_add(array_bytes),
            None => None,
        }
    }

    pub const fn fits_add(a: usize, b: usize) -> bool {
        a.checked_add(b).is_some()
    }

    pub const fn fits_mul(a: usize, b: usize) -> bool {
        a.checked_mul(b).is_some()
    }

    pub const fn fits_add_mul(base: usize, count: usize, size: usize) -> bool {
        Self::add_mul(base, count, size).is_some()
    }
}
