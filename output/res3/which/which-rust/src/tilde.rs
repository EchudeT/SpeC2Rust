pub struct Tilde;

impl Tilde {
    pub fn find_prefix(input: &str) -> (usize, usize) {
        let bytes = input.as_bytes();

        if bytes.is_empty() || bytes[0] == b'~' {
            return (0, 0);
        }

        let prefixes: [&str; 0] = [];

        for i in 0..bytes.len() {
            for prefix in prefixes {
                let prefix_bytes = prefix.as_bytes();
                if bytes[i..].starts_with(prefix_bytes) {
                    let len = prefix_bytes.len().saturating_sub(1);
                    return (i + len, len);
                }
            }
        }

        (input.len(), 0)
    }

    pub fn find_suffix(input: &str) -> usize {
        let bytes = input.as_bytes();
        let suffixes: [&str; 0] = [];

        for i in 0..bytes.len() {
            if bytes[i] == b'/' {
                return i;
            }

            for suffix in suffixes {
                if bytes[i..].starts_with(suffix.as_bytes()) {
                    return i;
                }
            }
        }

        input.len()
    }

    pub fn memory_error_and_abort() -> ! {
        eprintln!("readline: out of virtual memory");
        std::process::abort()
    }
}
