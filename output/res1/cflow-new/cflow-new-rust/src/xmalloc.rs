use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    const DEFAULT_MXFAST: usize = 64 * std::mem::size_of::<usize>() / 4;

    pub fn gl_attribute_pure<T>(value: Option<T>) -> T {
        match value {
            Some(v) => v,
            None => XallocDie::die(),
        }
    }

    pub fn checked_mul(a: usize, b: usize) -> Option<usize> {
        a.checked_mul(b)
    }

    pub fn checked_add(a: usize, b: usize) -> Option<usize> {
        a.checked_add(b)
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            XallocDie::die();
        }
        buffer.resize(size, 0);
        buffer
    }

    pub fn imalloc(size: usize) -> Vec<u8> {
        Self::malloc(size)
    }

    pub fn char_alloc(count: usize) -> Vec<u8> {
        Self::nmalloc(count, 1)
    }

    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut v) => {
                if size > v.len() {
                    let additional = size - v.len();
                    if v.try_reserve_exact(additional).is_err() {
                        XallocDie::die();
                    }
                }
                v.resize(size, 0);
                v
            }
            None => Self::malloc(size),
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        Self::realloc(buffer, size)
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let size = match Self::checked_mul(n, s) {
            Some(v) => v,
            None => XallocDie::die(),
        };
        Self::realloc(buffer, size)
    }

    pub fn irealloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(buffer, n, s)
    }

    pub fn nmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(None, n, s)
    }

    pub fn inmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::irealloc_array(None, n, s)
    }

    pub fn x_2_realloc(buffer: Option<Vec<u8>>, size: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, size, 1)
    }

    pub fn x_2_nrealloc(buffer: Option<Vec<u8>>, n: &mut usize, s: usize) -> Vec<u8> {
        if s == 0 {
            XallocDie::die();
        }

        let mut new_n = *n;

        if buffer.is_none() {
            if new_n == 0 {
                new_n = Self::DEFAULT_MXFAST / s;
                if new_n == 0 {
                    new_n = 1;
                }
            }
        } else {
            new_n = match Self::checked_add(new_n, (new_n >> 1) + 1) {
                Some(v) => v,
                None => XallocDie::die(),
            };
        }

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

        let n0 = *n;
        let mut new_n = match Self::checked_add(n0, n0 >> 1) {
            Some(v) => v,
            None => usize::MAX,
        };

        if n_max >= 0 {
            let max_n = n_max as usize;
            if max_n < new_n {
                new_n = max_n;
            }
        }

        let mut nbytes = match Self::checked_mul(new_n, s) {
            Some(bytes) => {
                if bytes < Self::DEFAULT_MXFAST {
                    Self::DEFAULT_MXFAST
                } else {
                    bytes
                }
            }
            None => usize::MAX,
        };

        if nbytes == usize::MAX {
            new_n = usize::MAX / s;
            nbytes = new_n.saturating_mul(s);
        } else if nbytes == Self::DEFAULT_MXFAST {
            new_n = nbytes / s;
            nbytes -= nbytes % s;
        }

        if buffer.is_none() {
            *n = 0;
        }

        if new_n.saturating_sub(n0) < n_incr_min {
            new_n = match Self::checked_add(n0, n_incr_min) {
                Some(v) => v,
                None => XallocDie::die(),
            };
            if n_max >= 0 && (n_max as usize) < new_n {
                XallocDie::die();
            }
            nbytes = match Self::checked_mul(new_n, s) {
                Some(v) => v,
                None => XallocDie::die(),
            };
        }

        let result = Self::realloc(buffer, nbytes);
        *n = new_n;
        result
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: usize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let size = match Self::checked_mul(n, s) {
            Some(v) => v,
            None => XallocDie::die(),
        };
        vec![0; size]
    }

    pub fn icalloc(n: usize, s: usize) -> Vec<u8> {
        Self::calloc(n, s)
    }

    pub fn memdup(data: &[u8], size: usize) -> Vec<u8> {
        data[..size].to_vec()
    }

    pub fn imemdup(data: &[u8], size: usize) -> Vec<u8> {
        Self::memdup(data, size)
    }

    pub fn ximemdup_0(data: &[u8], size: usize) -> Vec<u8> {
        let total = match Self::checked_add(size, 1) {
            Some(v) => v,
            None => XallocDie::die(),
        };
        let mut result = Self::imalloc(total);
        result[size] = 0;
        result[..size].copy_from_slice(&data[..size]);
        result
    }

    pub fn strdup(string: &str) -> String {
        String::from(string)
    }
}
