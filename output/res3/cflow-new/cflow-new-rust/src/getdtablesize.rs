pub struct Getdtablesize;


impl Getdtablesize {
    pub fn setmaxstdio_nothrow(newmax: i32) -> Option<i32> {
        #[cfg(windows)]
        {
            if newmax < 0 {
                return None;
            }

            let max_supported = 0x8000_i32;
            if newmax > max_supported {
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
        #[cfg(windows)]
        {

            let original_max_stdio = 512_i32;
            let mut bound = 0x10000_i32;

            while Self::setmaxstdio_nothrow(bound).is_none() && bound > 0 {
                bound /= 2;
            }

            let _ = Self::setmaxstdio_nothrow(original_max_stdio);

            let result = if bound > 0 { bound } else { i32::MAX };
            result
        }

        #[cfg(not(windows))]
        {
            Self::module_gnu_rlimit_08()
        }
    }

    pub fn module_gnu_rlimit_08() -> i32 {
        i32::MAX
    }
}
