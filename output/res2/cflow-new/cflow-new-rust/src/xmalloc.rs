use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    const DEFAULT_MXFAST: usize = 64 * std::mem::size_of::<usize>() / 4;

    pub fn gl_attribute_pure<T>(value: Option<T>) -> T {
        match value {
            Some(value) => value,
            None => XallocDie::die(),
        }
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

    pub fn char_alloc(size: usize) -> String {
        let bytes = Self::n_malloc(size, 1);
        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => XallocDie::die(),
        }
    }

    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut buffer) => {
                if size > buffer.len() {
                    let additional = size - buffer.len();
                    if buffer.try_reserve_exact(additional).is_err() {
                        XallocDie::die();
                    }
                }
                buffer.resize(size, 0);
                buffer
            }
            None => Self::malloc(size),
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        Self::realloc(buffer, size)
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let size = match n.checked_mul(s) {
            Some(size) => size,
            None => XallocDie::die(),
        };
        Self::realloc(buffer, size)
    }

    pub fn irealloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(buffer, n, s)
    }

    pub fn n_malloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(None, n, s)
    }

    pub fn in_malloc(n: usize, s: usize) -> Vec<u8> {
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
            new_n = match new_n.checked_add((new_n >> 1) + 1) {
                Some(value) => value,
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
        n_max: Option<usize>,
        s: usize,
    ) -> Vec<u8> {
        if s == 0 {
            XallocDie::die();
        }

        let n0 = *n;

        let mut new_n = n0.saturating_add(n0 >> 1);
        if let Some(max) = n_max {
            if max < new_n {
                new_n = max;
            }
        }

        let mut nbytes = match new_n.checked_mul(s) {
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
            let candidate = match n0.checked_add(n_incr_min) {
                Some(value) => value,
                None => XallocDie::die(),
            };
            if let Some(max) = n_max {
                if max < candidate {
                    XallocDie::die();
                }
            }
            nbytes = match candidate.checked_mul(s) {
                Some(value) => value,
                None => XallocDie::die(),
            };
            new_n = candidate;
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
        let size = match n.checked_mul(s) {
            Some(size) => size,
            None => XallocDie::die(),
        };
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            XallocDie::die();
        }
        buffer.resize(size, 0);
        buffer
    }

    pub fn icalloc(n: usize, s: usize) -> Vec<u8> {
        Self::calloc(n, s)
    }

    pub fn mem_dup(bytes: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }

    pub fn imem_dup(bytes: &[u8]) -> Vec<u8> {
        bytes.to_vec()
    }

    pub fn ximemdup_0(bytes: &[u8]) -> Vec<u8> {
        let new_len = match bytes.len().checked_add(1) {
            Some(value) => value,
            None => XallocDie::die(),
        };
        let mut result = Self::imalloc(new_len);
        result[..bytes.len()].copy_from_slice(bytes);
        result[bytes.len()] = 0;
        result
    }

    pub fn str_dup(string: &str) -> String {
        string.to_owned()
    }
}
