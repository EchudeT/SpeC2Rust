pub struct Unistd;

impl Unistd {
    pub fn stdin_fileno() -> i32 {
        0
    }

    pub fn stdout_fileno() -> i32 {
        1
    }

    pub fn stderr_fileno() -> i32 {
        2
    }

    pub fn standard_filenos() -> [i32; 3] {
        [
            Self::stdin_fileno(),
            Self::stdout_fileno(),
            Self::stderr_fileno(),
        ]
    }

    pub fn is_standard_fd(fd: i32) -> bool {
        matches!(fd, 0..=2)
    }

    pub fn sysconf_open_max_fallback() -> i32 {
        1024
    }

    pub fn page_size_fallback() -> usize {
        4096
    }
}
