pub struct Tilde;

impl Tilde {
    pub fn find_prefix(string: &str) -> (usize, usize) {
        let string_len = string.len();

        if string.is_empty() || string.as_bytes()[0] == b'~' {
            return (0, 0);
        }

        let prefixes: [&str; 0] = [];

        for i in 0..string_len {
            for prefix in prefixes {
                if string[i..].starts_with(prefix) {
                    let len = prefix.len().saturating_sub(1);
                    return (i + len, len);
                }
            }
        }

        (string_len, 0)
    }

    pub fn find_suffix(string: &str) -> usize {
        let bytes = string.as_bytes();
        let suffixes: [&str; 0] = [];

        for i in 0..bytes.len() {
            if bytes[i] == b'/' {
                return i;
            }

            for suffix in suffixes {
                if string[i..].starts_with(suffix) {
                    return i;
                }
            }
        }

        bytes.len()
    }

    pub fn memory_error_and_abort() -> ! {
        eprintln!("readline: out of virtual memory");
        std::process::abort()
    }
}
