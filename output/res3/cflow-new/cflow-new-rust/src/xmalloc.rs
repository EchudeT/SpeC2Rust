use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    const DEFAULT_MXFAST: usize = 64 * std::mem::size_of::<usize>() / 4;

    pub fn gl_attribute_pure() -> bool {
        true
    }

    pub fn alloc(size: usize) -> Vec<u8> {
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            XallocDie::die();
        }
        buffer.resize(size, 0);
        buffer
    }

    pub fn alloc_isize(size: isize) -> Vec<u8> {
        Self::alloc(Self::idx_to_usize(size))
    }

    pub fn char_alloc(n: usize) -> String {
        let bytes = Self::n_alloc(n, 1);
        String::from_utf8(bytes).unwrap_or_else(|_| XallocDie::die())
    }

    pub fn realloc_vec(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut existing) => {
                if size > existing.len() {
                    let additional = size - existing.len();
                    if existing.try_reserve_exact(additional).is_err() {
                        XallocDie::die();
                    }
                }
                existing.resize(size, 0);
                existing
            }
            None => Self::alloc(size),
        }
    }

    pub fn realloc_vec_isize(buffer: Option<Vec<u8>>, size: isize) -> Vec<u8> {
        Self::realloc_vec(buffer, Self::idx_to_usize(size))
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let size = Self::checked_mul(n, s).unwrap_or_else(|| XallocDie::die());
        Self::realloc_vec(buffer, size)
    }

    pub fn realloc_array_isize(buffer: Option<Vec<u8>>, n: isize, s: isize) -> Vec<u8> {
        let count = Self::idx_to_usize(n);
        let size = Self::idx_to_usize(s);
        Self::realloc_array(buffer, count, size)
    }

    pub fn n_alloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(None, n, s)
    }

    pub fn in_alloc(n: isize, s: isize) -> Vec<u8> {
        Self::realloc_array_isize(None, n, s)
    }

    pub fn x_2_realloc(buffer: Option<Vec<u8>>, ps: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, ps, 1)
    }

    pub fn x_2_nrealloc(buffer: Option<Vec<u8>>, pn: &mut usize, s: usize) -> Vec<u8> {
        if s == 0 {
            XallocDie::die();
        }

        let mut n = *pn;

        if buffer.is_none() {
            if n == 0 {
                n = Self::DEFAULT_MXFAST / s;
                n += usize::from(n == 0);
            }
        } else {
            n = Self::checked_add(n, (n >> 1).saturating_add(1)).unwrap_or_else(|| XallocDie::die());
        }

        let resized = Self::realloc_array(buffer, n, s);
        *pn = n;
        resized
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        pn: &mut isize,
        n_incr_min: isize,
        n_max: isize,
        s: isize,
    ) -> Vec<u8> {
        if n_incr_min < 0 || s <= 0 {
            XallocDie::die();
        }

        let n0 = *pn;
        let s_usize = Self::idx_to_usize(s);

        let mut n = match Self::checked_add_isize(n0, n0 >> 1) {
            Some(value) => value,
            None => isize::MAX,
        };

        if n_max >= 0 && n_max < n {
            n = n_max;
        }

        let mut nbytes = Self::checked_mul(Self::nonnegative_isize_to_usize(n), s_usize);
        let adjusted_nbytes = match nbytes {
            Some(bytes) if bytes < Self::DEFAULT_MXFAST => Self::DEFAULT_MXFAST,
            Some(_) => 0,
            None => usize::MAX,
        };

        if adjusted_nbytes != 0 {
            let adjusted_count = adjusted_nbytes / s_usize;
            n = isize::try_from(adjusted_count).unwrap_or_else(|_| XallocDie::die());
            let rounded_bytes = adjusted_nbytes - adjusted_nbytes % s_usize;
            nbytes = Some(rounded_bytes);
        }

        if buffer.is_none() {
            *pn = 0;
        }

        if n - n0 < n_incr_min {
            n = Self::checked_add_isize(n0, n_incr_min).unwrap_or_else(|| XallocDie::die());
            if (n_max >= 0 && n_max < n)
                || Self::checked_mul(Self::nonnegative_isize_to_usize(n), s_usize).is_none()
            {
                XallocDie::die();
            }
            nbytes = Self::checked_mul(Self::nonnegative_isize_to_usize(n), s_usize);
        }

        let resized = Self::realloc_vec(buffer, nbytes.unwrap_or_else(|| XallocDie::die()));
        *pn = n;
        resized
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc_vec(size, 1)
    }

    pub fn izalloc(size: isize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc_vec(n: usize, s: usize) -> Vec<u8> {
        let len = Self::checked_mul(n, s).unwrap_or_else(|| XallocDie::die());
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(len).is_err() {
            XallocDie::die();
        }
        buffer.resize(len, 0);
        buffer
    }

    pub fn icalloc(n: isize, s: isize) -> Vec<u8> {
        let count = Self::idx_to_usize(n);
        let size = Self::idx_to_usize(s);
        Self::calloc_vec(count, size)
    }

    pub fn mem_dup(data: &[u8], s: usize) -> Vec<u8> {
        if s > data.len() {
            XallocDie::die();
        }
        data[..s].to_vec()
    }

    pub fn mem_dup_isize(data: &[u8], s: isize) -> Vec<u8> {
        Self::mem_dup(data, Self::idx_to_usize(s))
    }

    pub fn ximemdup_0(data: &[u8], s: isize) -> Vec<u8> {
        let size = Self::idx_to_usize(s);
        if size > data.len() {
            XallocDie::die();
        }

        let total = Self::checked_add(size, 1).unwrap_or_else(|| XallocDie::die());
        let mut result = Self::alloc_isize(isize::try_from(total).unwrap_or_else(|_| XallocDie::die()));
        result[..size].copy_from_slice(&data[..size]);
        result[size] = 0;
        result
    }

    pub fn str_dup(string: &str) -> Vec<u8> {
        let bytes = string.as_bytes();
        let len_with_nul = Self::checked_add(bytes.len(), 1).unwrap_or_else(|| XallocDie::die());
        let mut duplicated =
            Self::mem_dup(bytes, bytes.len());
        if duplicated.try_reserve_exact(len_with_nul - duplicated.len()).is_err() {
            XallocDie::die();
        }
        duplicated.push(0);
        duplicated
    }

    pub fn checked_mul(a: usize, b: usize) -> Option<usize> {
        a.checked_mul(b)
    }

    pub fn checked_add(a: usize, b: usize) -> Option<usize> {
        a.checked_add(b)
    }

    fn idx_to_usize(value: isize) -> usize {
        usize::try_from(value).unwrap_or_else(|_| XallocDie::die())
    }

    fn nonnegative_isize_to_usize(value: isize) -> usize {
        usize::try_from(value).unwrap_or_else(|_| XallocDie::die())
    }

    fn checked_add_isize(a: isize, b: isize) -> Option<isize> {
        a.checked_add(b)
    }
}
