use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    pub fn gl_attribute_pure() -> bool {
        true
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Vec::<u8>::new());
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            XallocDie::fail();
        }
        buffer.resize(size, 0);
        buffer
    }

    pub fn imalloc(size: isize) -> Vec<u8> {
        let size = Self::idx_to_usize(size);
        Self::malloc(size)
    }

    pub fn char_alloc(size: usize) -> Vec<u8> {
        Self::nmalloc(size, 1)
    }

    pub fn realloc(buffer: Option<Vec<u8>>, new_size: usize) -> Vec<u8> {
        match buffer {
            None => Self::malloc(new_size),
            Some(mut buffer) => {
                if new_size > buffer.len() {
                    let additional = new_size - buffer.len();
                    if buffer.try_reserve_exact(additional).is_err() {
                        XallocDie::fail();
                    }
                }
                buffer.resize(new_size, 0);
                buffer
            }
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, new_size: isize) -> Vec<u8> {
        let new_size = Self::idx_to_usize(new_size);
        Self::realloc(buffer, new_size)
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, count: usize, element_size: usize) -> Vec<u8> {
        let total = match count.checked_mul(element_size) {
            Some(total) => total,
            None => XallocDie::fail(),
        };
        Self::realloc(buffer, total)
    }

    pub fn irealloc_array(
        buffer: Option<Vec<u8>>,
        count: isize,
        element_size: isize,
    ) -> Vec<u8> {
        let count = Self::idx_to_usize(count);
        let element_size = Self::idx_to_usize(element_size);
        Self::realloc_array(buffer, count, element_size)
    }

    pub fn nmalloc(count: usize, element_size: usize) -> Vec<u8> {
        Self::realloc_array(None, count, element_size)
    }

    pub fn inmalloc(count: isize, element_size: isize) -> Vec<u8> {
        Self::irealloc_array(None, count, element_size)
    }

    pub fn x_2_realloc(buffer: Option<Vec<u8>>, size_slot: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, size_slot, 1)
    }

    pub fn x_2_nrealloc(
        buffer: Option<Vec<u8>>,
        count_slot: &mut usize,
        element_size: usize,
    ) -> Vec<u8> {
        if element_size == 0 {
            XallocDie::fail();
        }

        let mut count = *count_slot;

        if buffer.is_none() {
            if count == 0 {
                const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;
                count = DEFAULT_MXFAST / element_size;
                if count == 0 {
                    count = 1;
                }
            }
        } else {
            count = match count.checked_add((count >> 1) + 1) {
                Some(next) => next,
                None => XallocDie::fail(),
            };
        }

        let result = Self::realloc_array(buffer, count, element_size);
        *count_slot = count;
        result
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        count_slot: &mut isize,
        min_increase: isize,
        max_count: isize,
        element_size: isize,
    ) -> Vec<u8> {
        let n0 = *count_slot;
        if n0 < 0 || min_increase < 0 || element_size <= 0 {
            XallocDie::fail();
        }

        const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;

        let mut n = n0.saturating_add(n0 >> 1);
        if max_count >= 0 && max_count < n {
            n = max_count;
        }

        let s = Self::idx_to_usize(element_size);

        let adjusted_nbytes = match Self::idx_to_usize(n).checked_mul(s) {
            Some(nbytes) => {
                if nbytes < DEFAULT_MXFAST {
                    DEFAULT_MXFAST
                } else {
                    0
                }
            }
            None => usize::MAX.min(isize::MAX as usize),
        };

        let mut nbytes = match Self::idx_to_usize(n).checked_mul(s) {
            Some(nbytes) => nbytes,
            None => usize::MAX.min(isize::MAX as usize),
        };

        if adjusted_nbytes != 0 {
            n = (adjusted_nbytes / s) as isize;
            nbytes = adjusted_nbytes - adjusted_nbytes % s;
        }

        if buffer.is_none() {
            *count_slot = 0;
        }

        let current = *count_slot;
        if n - current < min_increase {
            n = match current.checked_add(min_increase) {
                Some(v) => v,
                None => XallocDie::fail(),
            };
            if max_count >= 0 && max_count < n {
                XallocDie::fail();
            }
            nbytes = match Self::idx_to_usize(n).checked_mul(s) {
                Some(v) => v,
                None => XallocDie::fail(),
            };
        }

        let result = Self::realloc(buffer, nbytes);
        *count_slot = n;
        result
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: isize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(count: usize, element_size: usize) -> Vec<u8> {
        let total = match count.checked_mul(element_size) {
            Some(total) => total,
            None => XallocDie::fail(),
        };
        let buffer = vec![0; total];
        Self::check_nonnull(buffer)
    }

    pub fn icalloc(count: isize, element_size: isize) -> Vec<u8> {
        let count = Self::idx_to_usize(count);
        let element_size = Self::idx_to_usize(element_size);
        Self::calloc(count, element_size)
    }

    pub fn memdup(data: &[u8]) -> Vec<u8> {
        let mut copy = Self::malloc(data.len());
        copy.copy_from_slice(data);
        copy
    }

    pub fn imemdup(data: &[u8], size: isize) -> Vec<u8> {
        let size = Self::idx_to_usize(size);
        if size > data.len() {
            XallocDie::fail();
        }
        let mut copy = Self::imalloc(size as isize);
        copy.copy_from_slice(&data[..size]);
        copy
    }

    pub fn ximemdup_0(data: &[u8], size: isize) -> Vec<u8> {
        let size = Self::idx_to_usize(size);
        if size > data.len() {
            XallocDie::fail();
        }
        let alloc_size = match size.checked_add(1) {
            Some(v) => v,
            None => XallocDie::fail(),
        };
        let mut result = Self::imalloc(alloc_size as isize);
        result[size] = 0;
        result[..size].copy_from_slice(&data[..size]);
        result
    }

    pub fn strdup(string: &str) -> String {
        string.to_owned()
    }

    pub fn check_nonnull<T>(value: T) -> T {
        value
    }

    fn idx_to_usize(value: isize) -> usize {
        match usize::try_from(value) {
            Ok(v) => v,
            Err(_) => XallocDie::fail(),
        }
    }
}
