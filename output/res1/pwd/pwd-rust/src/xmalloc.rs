use crate::xalloc_die::XallocDie;

pub struct Xmalloc;

impl Xmalloc {
    const DEFAULT_MXFAST: usize = 64 * core::mem::size_of::<usize>() / 4;

    pub fn gl_attribute_pure() -> bool {
        true
    }

    pub fn malloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Self::try_uninitialized_allocation(size))
    }

    pub fn imalloc(size: usize) -> Vec<u8> {
        Self::check_nonnull(Self::try_uninitialized_allocation(size))
    }

    pub fn char_alloc(n: usize) -> Vec<u8> {
        Self::nmalloc(n, 1)
    }

    pub fn realloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut existing) => {
                existing.resize(size, 0);
                existing
            }
            None => Self::malloc(size),
        }
    }

    pub fn irealloc(buffer: Option<Vec<u8>>, size: usize) -> Vec<u8> {
        match buffer {
            Some(mut existing) => {
                existing.resize(size, 0);
                existing
            }
            None => {
                if size > isize::MAX as usize {
                    Self::handle_alloc_error();
                }
                Self::imalloc(size)
            }
        }
    }

    pub fn realloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        let total = n.checked_mul(s).unwrap_or_else(Self::allocation_overflow);
        Self::realloc(buffer, total)
    }

    pub fn irealloc_array(buffer: Option<Vec<u8>>, n: usize, s: usize) -> Vec<u8> {
        if n > isize::MAX as usize && s != 0 {
            Self::handle_alloc_error();
        }
        if s > isize::MAX as usize && n != 0 {
            Self::handle_alloc_error();
        }

        let requested = if n == 0 || s == 0 {
            0
        } else {
            n.checked_mul(s).unwrap_or_else(Self::allocation_overflow)
        };

        match buffer {
            Some(mut existing) => {
                existing.resize(requested, 0);
                existing
            }
            None => Self::imalloc(requested),
        }
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

    pub fn x_2_nrealloc(buffer: Option<Vec<u8>>, count: &mut usize, elem_size: usize) -> Vec<u8> {
        if elem_size == 0 {
            Self::handle_alloc_error();
        }

        let mut n = *count;

        if buffer.is_none() {
            if n == 0 {
                n = Self::DEFAULT_MXFAST / elem_size;
                if n == 0 {
                    n = 1;
                }
            }
        } else {
            n = n
                .checked_add((n >> 1) + 1)
                .unwrap_or_else(Self::allocation_overflow);
        }

        let result = Self::realloc_array(buffer, n, elem_size);
        *count = n;
        result
    }

    pub fn palloc(
        buffer: Option<Vec<u8>>,
        count: &mut usize,
        min_increment: usize,
        max_count: Option<usize>,
        elem_size: usize,
    ) -> Vec<u8> {
        if elem_size == 0 {
            Self::handle_alloc_error();
        }

        let original_count = *count;

        let mut grown = original_count
            .checked_add(original_count >> 1)
            .unwrap_or(usize::MAX);

        if let Some(limit) = max_count {
            if limit < grown {
                grown = limit;
            }
        }

        let mut byte_count = match grown.checked_mul(elem_size) {
            Some(bytes) if bytes >= Self::DEFAULT_MXFAST => bytes,
            Some(_) => Self::DEFAULT_MXFAST,
            None => usize::MAX,
        };

        if byte_count != 0 && byte_count != usize::MAX {
            grown = byte_count / elem_size;
            byte_count -= byte_count % elem_size;
        } else if byte_count == usize::MAX {
            grown = usize::MAX;
        }

        if buffer.is_none() {
            *count = 0;
        }

        if grown.saturating_sub(original_count) < min_increment {
            grown = original_count
                .checked_add(min_increment)
                .unwrap_or_else(Self::allocation_overflow);

            if let Some(limit) = max_count {
                if limit < grown {
                    Self::handle_alloc_error();
                }
            }

            byte_count = grown
                .checked_mul(elem_size)
                .unwrap_or_else(Self::allocation_overflow);
        }

        let result = Self::realloc(buffer, byte_count);
        *count = grown;
        result
    }

    pub fn zalloc(size: usize) -> Vec<u8> {
        Self::calloc(size, 1)
    }

    pub fn izalloc(size: usize) -> Vec<u8> {
        Self::icalloc(size, 1)
    }

    pub fn calloc(n: usize, s: usize) -> Vec<u8> {
        let total = n.checked_mul(s).unwrap_or_else(Self::allocation_overflow);
        Self::check_nonnull(vec![0; total])
    }

    pub fn icalloc(n: usize, s: usize) -> Vec<u8> {
        if n > isize::MAX as usize && s != 0 {
            Self::handle_alloc_error();
        }
        if s > isize::MAX as usize && n != 0 {
            Self::handle_alloc_error();
        }

        let total = if n == 0 || s == 0 {
            0
        } else {
            n.checked_mul(s).unwrap_or_else(Self::allocation_overflow)
        };

        Self::check_nonnull(vec![0; total])
    }

    pub fn memdup(data: &[u8]) -> Vec<u8> {
        let result = Self::malloc(data.len());
        Self::copy_into(result, data)
    }

    pub fn imemdup(data: &[u8]) -> Vec<u8> {
        let result = Self::imalloc(data.len());
        Self::copy_into(result, data)
    }

    pub fn ximemdup_0(data: &[u8]) -> Vec<u8> {
        let new_len = data.len().checked_add(1).unwrap_or_else(Self::allocation_overflow);
        let mut result = Self::imalloc(new_len);
        result[..data.len()].copy_from_slice(data);
        result[data.len()] = 0;
        result
    }

    pub fn strdup(string: &str) -> String {
        string.to_owned()
    }

    pub fn check_nonnull<T>(value: T) -> T {
        value
    }

    pub fn handle_alloc_error() -> ! {
        XallocDie::fail()
    }

    fn try_uninitialized_allocation(size: usize) -> Vec<u8> {
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            Self::handle_alloc_error();
        }
        buffer.resize(size, 0);
        buffer
    }

    fn copy_into(mut destination: Vec<u8>, source: &[u8]) -> Vec<u8> {
        destination[..source.len()].copy_from_slice(source);
        destination
    }

    fn allocation_overflow(_: core::num::TryFromIntError) -> ! {
        Self::handle_alloc_error()
    }
}

trait OverflowAbort<T> {
    fn unwrap_or_else(self, f: fn(T) -> !) -> usize;
}

impl OverflowAbort<core::num::TryFromIntError> for Result<usize, core::num::TryFromIntError> {
    fn unwrap_or_else(self, f: fn(core::num::TryFromIntError) -> !) -> usize {
        match self {
            Ok(value) => value,
            Err(error) => f(error),
        }
    }
}

trait CheckedMulAbort {
    fn unwrap_or_else(self, f: fn(core::num::TryFromIntError) -> !) -> usize;
}

impl CheckedMulAbort for Option<usize> {
    fn unwrap_or_else(self, f: fn(core::num::TryFromIntError) -> !) -> usize {
        match self {
            Some(value) => value,
            None => {
                let error = usize::try_from(u128::MAX).err().expect("overflow sentinel");
                f(error)
            }
        }
    }
}
