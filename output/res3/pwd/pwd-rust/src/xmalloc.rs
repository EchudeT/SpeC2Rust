use crate::xalloc_die::XallocDie;
use std::borrow::Cow;

pub struct Xmalloc;

impl Xmalloc {
    pub fn gl_attribute_pure() -> bool {
        true
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Vec::with_capacity(size))
    }

    pub fn imalloc(size: usize) -> Vec<u8> {
        Self::malloc(size)
    }

    pub fn char_alloc(size: usize) -> String {
        let bytes = Self::malloc(size);
        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }

    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut buf) => {
                if size > buf.len() {
                    buf.resize(size, 0);
                } else {
                    buf.truncate(size);
                }
                buf
            }
            None => {
                let mut buf = Self::malloc(size);
                buf.resize(size, 0);
                buf
            }
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        Self::realloc(buffer, size)
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let size = n.checked_mul(s).unwrap_or_else(|| XallocDie::fail());
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

    pub fn x_2_realloc(buffer: Option<Vec<u8>>, current_size: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, current_size, 1)
    }

    pub fn x_2_nrealloc(
        buffer: Option<Vec<u8>>,
        current_count: &mut usize,
        element_size: usize,
    ) -> Vec<u8> {
        if element_size == 0 {
            XallocDie::fail();
        }

        let mut n = *current_count;
        if buffer.is_none() {
            if n == 0 {
                let default_mxfast = 64 * std::mem::size_of::<usize>() / 4;
                n = default_mxfast / element_size;
                if n == 0 {
                    n = 1;
                }
            }
        } else {
            n = n
                .checked_add((n >> 1) + 1)
                .unwrap_or_else(|| XallocDie::fail());
        }

        let result = Self::realloc_array(buffer, n, element_size);
        *current_count = n;
        result
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        count: &mut usize,
        min_increase: usize,
        max_count: Option<usize>,
        element_size: usize,
    ) -> Vec<u8> {
        if element_size == 0 {
            XallocDie::fail();
        }

        let n0 = *count;
        let default_mxfast = 64 * std::mem::size_of::<usize>() / 4;

        let mut n = n0.saturating_add(n0 >> 1);
        if let Some(max) = max_count {
            if n > max {
                n = max;
            }
        }

        match n.checked_mul(element_size) {
            Some(bytes) if bytes < default_mxfast => {
                let adjusted = default_mxfast;
                n = adjusted / element_size;
            }
            Some(_) => {}
            None => n = usize::MAX / element_size,
        };

        let baseline = if buffer.is_none() {
            *count = 0;
            0
        } else {
            n0
        };

        if n.checked_sub(baseline).unwrap_or(0) < min_increase {
            n = baseline
                .checked_add(min_increase)
                .unwrap_or_else(|| XallocDie::fail());
            if let Some(max) = max_count {
                if n > max {
                    XallocDie::fail();
                }
            }
        }

        let final_nbytes = n.checked_mul(element_size).unwrap_or_else(|| XallocDie::fail());
        let result = Self::realloc(buffer, final_nbytes);
        *count = n;
        result
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: usize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let total = n.checked_mul(s).unwrap_or_else(|| XallocDie::fail());
        Self::check_nonnull(vec![0; total])
    }

    pub fn icalloc(n: usize, s: usize) -> Vec<u8> {
        Self::calloc(n, s)
    }

    pub fn memdup(data: &[u8], size: usize) -> Vec<u8> {
        if size > data.len() {
            XallocDie::fail();
        }
        data[..size].to_vec()
    }

    pub fn imemdup(data: &[u8], size: usize) -> Vec<u8> {
        Self::memdup(data, size)
    }

    pub fn ximemdup_0(data: &[u8], size: usize) -> Vec<u8> {
        if size > data.len() {
            XallocDie::fail();
        }
        let mut result = Self::imalloc(size.checked_add(1).unwrap_or_else(|| XallocDie::fail()));
        result.extend_from_slice(&data[..size]);
        result.push(0);
        result
    }

    pub fn strdup(string: &str) -> String {
        string.to_owned()
    }

    pub fn check_nonnull<T>(value: T) -> T {
        value
    }

    #[allow(dead_code)]
    fn _trace_name<'a>(name: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(name)
    }
}
