use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    pub fn gl_attribute_pure() {}

    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Vec::with_capacity(size))
    }

    pub fn imalloc(size: isize) -> Vec<u8> {
        let size = Self::idx_to_usize(size);
        Self::malloc(size)
    }

    pub fn char_alloc(n: usize) -> Vec<u8> {
        Self::nmalloc(n, 1)
    }

    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut bytes) => {
                if size > bytes.len() {
                    bytes.resize(size, 0);
                } else {
                    bytes.truncate(size);
                }
                bytes
            }
            None => vec![0; size],
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, size: isize) -> Vec<u8> {
        Self::realloc(buffer, Self::idx_to_usize(size))
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let size = Self::checked_mul(n, s);
        Self::realloc(buffer, size)
    }

    pub fn irealloc_array(buffer: Option<Vec<u8>>, n: isize, s: isize) -> Vec<u8> {
        let size = Self::checked_mul(Self::idx_to_usize(n), Self::idx_to_usize(s));
        Self::realloc(buffer, size)
    }

    pub fn nmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(None, n, s)
    }

    pub fn inmalloc(n: isize, s: isize) -> Vec<u8> {
        Self::irealloc_array(None, n, s)
    }

    pub fn x_2_realloc(buffer: Option<Vec<u8>>, current_size: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, current_size, 1)
    }

    pub fn x_2_nrealloc(buffer: Option<Vec<u8>>, current_count: &mut usize, element_size: usize) -> Vec<u8> {
        if element_size == 0 {
            XallocDie::fail();
        }

        let mut n = *current_count;

        if buffer.is_none() {
            if n == 0 {
                const DEFAULT_MXFAST: usize = 64 * (core::mem::size_of::<usize>()) / 4;
                n = DEFAULT_MXFAST / element_size;
                if n == 0 {
                    n = 1;
                }
            }
        } else {
            n = Self::checked_add(n, (n >> 1) + 1);
        }

        let result = Self::realloc_array(buffer, n, element_size);
        *current_count = n;
        result
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        n: &mut isize,
        n_incr_min: isize,
        n_max: isize,
        s: isize,
    ) -> Vec<u8> {
        let n0 = *n;
        let s_usize = Self::idx_to_usize(s);
        let n0_usize = Self::idx_to_usize(n0);
        let n_incr_min_usize = Self::idx_to_usize(n_incr_min);

        if s_usize == 0 {
            XallocDie::fail();
        }

        const DEFAULT_MXFAST: usize = 64 * (core::mem::size_of::<usize>()) / 4;

        let mut new_n = n0_usize.saturating_add(n0_usize >> 1);
        if n_max >= 0 {
            let n_max_usize = Self::idx_to_usize(n_max);
            if n_max_usize < new_n {
                new_n = n_max_usize;
            }
        }

        let adjusted_nbytes = match new_n.checked_mul(s_usize) {
            None => usize::MAX,
            Some(nbytes) if nbytes < DEFAULT_MXFAST => DEFAULT_MXFAST,
            Some(_) => 0,
        };

        let mut nbytes = new_n.saturating_mul(s_usize);

        if adjusted_nbytes != 0 {
            new_n = adjusted_nbytes / s_usize;
            nbytes = adjusted_nbytes - adjusted_nbytes % s_usize;
        }

        if buffer.is_none() {
            *n = 0;
        }

        if new_n.saturating_sub(n0_usize) < n_incr_min_usize {
            let candidate = match n0_usize.checked_add(n_incr_min_usize) {
                Some(v) => v,
                None => XallocDie::fail(),
            };
            if n_max >= 0 && Self::idx_to_usize(n_max) < candidate {
                XallocDie::fail();
            }
            nbytes = match candidate.checked_mul(s_usize) {
                Some(v) => v,
                None => XallocDie::fail(),
            };
            new_n = candidate;
        }

        let result = Self::realloc(buffer, nbytes);
        *n = match isize::try_from(new_n) {
            Ok(v) => v,
            Err(_) => XallocDie::fail(),
        };
        result
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: isize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let len = Self::checked_mul(n, s);
        Self::check_nonnull(vec![0; len])
    }

    pub fn icalloc(n: isize, s: isize) -> Vec<u8> {
        let len = Self::checked_mul(Self::idx_to_usize(n), Self::idx_to_usize(s));
        Self::check_nonnull(vec![0; len])
    }

    pub fn memdup(data: &[u8], s: usize) -> Vec<u8> {
        if s > data.len() {
            XallocDie::fail();
        }
        data[..s].to_vec()
    }

    pub fn imemdup(data: &[u8], s: isize) -> Vec<u8> {
        Self::memdup(data, Self::idx_to_usize(s))
    }

    pub fn ximemdup_0(data: &[u8], s: isize) -> Vec<u8> {
        let len = Self::idx_to_usize(s);
        if len > data.len() {
            XallocDie::fail();
        }
        let total = Self::checked_add(len, 1);
        let mut result = Self::imalloc(
            isize::try_from(total).unwrap_or_else(|_| XallocDie::fail()),
        );
        result.resize(total, 0);
        result[..len].copy_from_slice(&data[..len]);
        result
    }

    pub fn strdup(string: &str) -> Vec<u8> {
        let bytes = string.as_bytes();
        let len_with_nul = Self::checked_add(bytes.len(), 1);
        let mut result = Vec::with_capacity(len_with_nul);
        result.extend_from_slice(bytes);
        result.push(0);
        result
    }

    pub fn check_nonnull<T>(value: T) -> T {
        value
    }

    pub fn checked_mul(a: usize, b: usize) -> usize {
        match a.checked_mul(b) {
            Some(v) => v,
            None => XallocDie::fail(),
        }
    }

    pub fn checked_add(a: usize, b: usize) -> usize {
        match a.checked_add(b) {
            Some(v) => v,
            None => XallocDie::fail(),
        }
    }

    fn idx_to_usize(value: isize) -> usize {
        match usize::try_from(value) {
            Ok(v) => v,
            Err(_) => XallocDie::fail(),
        }
    }
}
