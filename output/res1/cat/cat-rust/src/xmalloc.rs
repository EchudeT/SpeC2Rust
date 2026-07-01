use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    pub fn gl_attribute_pure() -> bool {
        true
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Some(vec![0; size]))
    }

    pub fn imalloc(size: isize) -> Vec<u8> {
        let size = usize::try_from(size).unwrap_or_else(|_| XallocDie::fail());
        Self::malloc(size)
    }

    pub fn char_alloc(size: usize) -> Vec<u8> {
        Self::nmalloc(size, 1)
    }

    pub fn realloc(mut buffer: Vec<u8>, size: usize) -> Vec<u8> {
        buffer.resize(size, 0);
        buffer
    }

    pub fn irealloc(buffer: Vec<u8>, size: isize) -> Vec<u8> {
        let size = usize::try_from(size).unwrap_or_else(|_| XallocDie::fail());
        Self::realloc(buffer, size)
    }

    pub fn realloc_array(buffer: Vec<u8>, n: usize, s: usize) -> Vec<u8> {
        let size = Self::checked_mul(n, s);
        Self::realloc(buffer, size)
    }

    pub fn irealloc_array(buffer: Vec<u8>, n: isize, s: isize) -> Vec<u8> {
        let n = usize::try_from(n).unwrap_or_else(|_| XallocDie::fail());
        let s = usize::try_from(s).unwrap_or_else(|_| XallocDie::fail());
        Self::realloc_array(buffer, n, s)
    }

    pub fn nmalloc(n: usize, s: usize) -> Vec<u8> {
        Self::realloc_array(Vec::new(), n, s)
    }

    pub fn inmalloc(n: isize, s: isize) -> Vec<u8> {
        Self::irealloc_array(Vec::new(), n, s)
    }

    pub fn x_2_realloc(buffer: Vec<u8>, current: &mut usize) -> Vec<u8> {
        Self::x_2_nrealloc(buffer, current, 1)
    }

    pub fn x_2_nrealloc(buffer: Vec<u8>, current: &mut usize, element_size: usize) -> Vec<u8> {
        if element_size == 0 {
            XallocDie::fail();
        }

        let mut n = *current;

        if buffer.is_empty() {
            if n == 0 {
                const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;
                n = DEFAULT_MXFAST / element_size;
                n += usize::from(n == 0);
            }
        } else {
            n = Self::checked_add(n, (n >> 1) + 1);
        }

        let resized = Self::realloc_array(buffer, n, element_size);
        *current = n;
        resized
    }

    pub fn palloc(
        buffer: Vec<u8>,
        current: &mut isize,
        min_increment: isize,
        max_count: isize,
        element_size: isize,
    ) -> Vec<u8> {
        if min_increment < 0 || element_size <= 0 {
            XallocDie::fail();
        }

        let s = usize::try_from(element_size).unwrap_or_else(|_| XallocDie::fail());
        let n0 = usize::try_from(*current).unwrap_or_else(|_| XallocDie::fail());
        let min_increment = usize::try_from(min_increment).unwrap_or_else(|_| XallocDie::fail());
        let max_count_opt = if max_count < 0 {
            None
        } else {
            Some(usize::try_from(max_count).unwrap_or_else(|_| XallocDie::fail()))
        };

        const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;

        let mut n = n0.saturating_add(n0 >> 1);
        if let Some(max_count) = max_count_opt {
            if max_count < n {
                n = max_count;
            }
        }

        let mut nbytes = match n.checked_mul(s) {
            Some(bytes) if bytes < DEFAULT_MXFAST => {
                let adjusted = DEFAULT_MXFAST;
                n = adjusted / s;
                adjusted - adjusted % s
            }
            Some(bytes) => bytes,
            None => {
                let capped = usize::MAX;
                n = capped / s;
                capped - capped % s
            }
        };

        let original_was_empty = buffer.is_empty();
        if original_was_empty {
            *current = 0;
        }

        if n.saturating_sub(n0) < min_increment {
            n = Self::checked_add(n0, min_increment);
            if let Some(max_count) = max_count_opt {
                if max_count < n {
                    XallocDie::fail();
                }
            }
            nbytes = Self::checked_mul(n, s);
        }

        let resized = Self::realloc(buffer, nbytes);
        *current = isize::try_from(n).unwrap_or_else(|_| XallocDie::fail());
        resized
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: isize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let size = Self::checked_mul(n, s);
        Self::check_nonnull(Some(vec![0; size]))
    }

    pub fn icalloc(n: isize, s: isize) -> Vec<u8> {
        let n = usize::try_from(n).unwrap_or_else(|_| XallocDie::fail());
        let s = usize::try_from(s).unwrap_or_else(|_| XallocDie::fail());
        Self::calloc(n, s)
    }

    pub fn mem_dup(data: &[u8], size: usize) -> Vec<u8> {
        Self::to_vec(&data[..size.min(data.len())])
    }

    pub fn imem_dup(data: &[u8], size: isize) -> Vec<u8> {
        let size = usize::try_from(size).unwrap_or_else(|_| XallocDie::fail());
        Self::mem_dup(data, size)
    }

    pub fn ximemdup_0(data: &[u8], size: isize) -> Vec<u8> {
        let size = usize::try_from(size).unwrap_or_else(|_| XallocDie::fail());
        let mut result = Self::imalloc(
            isize::try_from(Self::checked_add(size, 1)).unwrap_or_else(|_| XallocDie::fail()),
        );
        result.clear();
        Self::extend_from_slice(&mut result, &data[..size.min(data.len())]);
        result.push(0);
        result
    }

    pub fn str_dup(string: &str) -> String {
        Self::to_string(string)
    }

    pub fn check_nonnull<T>(value: Option<T>) -> T {
        match value {
            Some(value) => value,
            None => XallocDie::fail(),
        }
    }

    pub fn checked_mul(a: usize, b: usize) -> usize {
        a.checked_mul(b).unwrap_or_else(|| XallocDie::fail())
    }

    pub fn checked_add(a: usize, b: usize) -> usize {
        a.checked_add(b).unwrap_or_else(|| XallocDie::fail())
    }

    pub fn to_vec(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    pub fn extend_from_slice(buffer: &mut Vec<u8>, data: &[u8]) {
        buffer.extend_from_slice(data);
    }

    pub fn to_owned(value: &str) -> String {
        value.to_owned()
    }

    pub fn to_string(value: &str) -> String {
        value.to_string()
    }
}
