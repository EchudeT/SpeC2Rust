pub struct Getdtablesize;


impl Getdtablesize {
    pub fn setmaxstdio_nothrow(newmax: i32) -> Option<i32> {
        if newmax <= 0 {
            return None;
        }

        #[cfg(windows)]
        {
            const WINDOWS_UPPER_BOUND: i32 = 2048;
            if newmax > WINDOWS_UPPER_BOUND {
                None
            } else {
                Some(newmax)
            }
        }

        #[cfg(not(windows))]
        {
            Some(newmax)
        }
    }

    pub fn table_size() -> i32 {
        #[cfg(unix)]
        {
            return Self::module_gnu_rlimit_08();
        }

        #[cfg(windows)]
        {
            }

            let original_max_stdio = 512i32;
            let mut bound = 0x10000i32;
            while bound > 0 && Self::setmaxstdio_nothrow(bound).is_none() {
                bound /= 2;
            }

            if bound <= 0 {
                bound = original_max_stdio.max(1);
            }

            let _ = Self::setmaxstdio_nothrow(original_max_stdio);
            return bound;
        }

        #[cfg(not(any(unix, windows)))]
        {
            i32::MAX
        }
    }

    pub fn module_gnu_rlimit_08() -> i32 {
        i32::MAX
    }
