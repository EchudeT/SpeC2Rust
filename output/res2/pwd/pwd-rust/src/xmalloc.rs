use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    #[inline]
    pub fn gl_attribute_pure() {}

    #[inline]
    pub fn check_nonnull<T>(value: Option<T>) -> T {
        match value {
            Some(value) => value,
            None => XallocDie::die(),
        }
    }

    #[inline]
    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Some(vec![0_u8; size]))
    }

    #[inline]
    pub fn imalloc(size: usize) -> Vec<u8> {
        Self::malloc(size)
    }

    #[inline]
    pub fn char_alloc(n: usize) -> String {
        let bytes = Self::nmalloc(n, 1);
        String::from_utf8(bytes).expect("xcharalloc produced invalid UTF-8")
    }

    #[inline]
    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut buffer) => {
                buffer.resize(size, 0);
                buffer
            }
            None => Self::malloc(size),
        }
    }

    #[inline]
    pub fn irealloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        Self::realloc(buffer, size)
    }

    #[inline]
    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let total = n.checked_mul(s).unwrap_or_else(|| XallocDie::die());
        Self::realloc(buffer, total)
    }

    #[inline]
    pub fn irealloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(buffer, n, s)
    }

    #[inline]
    pub fn nmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(None, n, s)
    }

    #[inline]
    pub fn inmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::irealloc_array(None, n, s)
    }

    #[inline]
    pub fn x_2_realloc(buffer: Option<Vec<u8>>, size: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, size, 1)
    }

    pub fn x_2_nrealloc(buffer: Option<Vec<u8>>, n: &mut usize, s: usize) -> Vec<u8> {
        if s == 0 {
            XallocDie::die();
        }

        const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;

        let new_n = if buffer.is_none() {
            if *n == 0 {
                let mut initial = DEFAULT_MXFAST / s;
                if initial == 0 {
                    initial = 1;
                }
                initial
            } else {
                *n
            }
        } else {
            n.checked_add((*n >> 1) + 1)
                .unwrap_or_else(|| XallocDie::die())
        };

        let result = Self::realloc_array(buffer, new_n, s);
        *n = new_n;
        result
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        n: &mut usize,
        n_incr_min: usize,
        n_max: isize,
        s: usize,
    ) -> Vec<u8> {
        if s == 0 {
            XallocDie::die();
        }

        const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;

        let n0 = *n;
        let mut new_n = n0.saturating_add(n0 >> 1);

        if n_max >= 0 {
            let max = n_max as usize;
            if max < new_n {
                new_n = max;
            }
        }

        let mut nbytes = match new_n.checked_mul(s) {
            Some(bytes) if bytes >= DEFAULT_MXFAST => bytes,
            Some(_) => DEFAULT_MXFAST - (DEFAULT_MXFAST % s),
            None => usize::MAX - (usize::MAX % s),
        };

        if nbytes == 0 && s > 0 {
            XallocDie::die();
        }

        new_n = nbytes / s;

        if buffer.is_none() {
            *n = 0;
        }

        if new_n.saturating_sub(n0) < n_incr_min {
            let required = n0
                .checked_add(n_incr_min)
                .unwrap_or_else(|| XallocDie::die());

            if n_max >= 0 && (n_max as usize) < required {
                XallocDie::die();
            }

            nbytes = required.checked_mul(s).unwrap_or_else(|| XallocDie::die());
            new_n = required;
        }

        let result = Self::realloc(buffer, nbytes);
        *n = new_n;
        result
    }

    #[inline]
    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    #[inline]
    pub fn izalloc(size: usize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    #[inline]
    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let total = n.checked_mul(s).unwrap_or_else(|| XallocDie::die());
        Self::check_nonnull(Some(vec![0_u8; total]))
    }

    #[inline]
    pub fn icalloc(n: usize, s: usize) -> Vec<u8> {
        Self::calloc(n, s)
    }

    #[inline]
    pub fn memdup(data: &[u8], s: usize) -> Vec<u8> {
        assert!(s <= data.len(), "xmemdup length exceeds source length");
        let mut result = Self::malloc(s);
        result[..s].copy_from_slice(&data[..s]);
        result
    }

    #[inline]
    pub fn imemdup(data: &[u8], s: usize) -> Vec<u8> {
        assert!(s <= data.len(), "ximemdup length exceeds source length");
        let mut result = Self::imalloc(s);
        result[..s].copy_from_slice(&data[..s]);
        result
    }

    #[inline]
    pub fn ximemdup_0(data: &[u8], s: usize) -> Vec<u8> {
        assert!(s <= data.len(), "ximemdup_0 length exceeds source length");
        let mut result = Self::imalloc(s.checked_add(1).unwrap_or_else(|| XallocDie::die()));
        result[..s].copy_from_slice(&data[..s]);
        result[s] = 0;
        result
    }

    #[inline]
    pub fn strdup(string: &str) -> String {
        string.to_owned()
    }
}
