pub struct Tilde {
    additional_prefixes: Vec<String>,
    additional_suffixes: Vec<String>,
}

impl Default for Tilde {
    fn default() -> Self {
        Self {
            additional_prefixes: Vec::new(),
            additional_suffixes: Vec::new(),
        }
    }
}

impl Tilde {
    pub fn find_prefix(&self, input: &str) -> (usize, usize) {
        let bytes = input.as_bytes();
        let string_len = bytes.len();

        if bytes.is_empty() || bytes[0] == b'~' {
            return (0, 0);
        }

        for i in 0..string_len {
            for prefix in &self.additional_prefixes {
                let prefix_bytes = prefix.as_bytes();
                if bytes[i..].starts_with(prefix_bytes) {
                    let len = prefix_bytes.len().saturating_sub(1);
                    return (i + len, len);
                }
            }
        }

        (string_len, 0)
    }

    pub fn find_suffix(&self, input: &str) -> usize {
        let bytes = input.as_bytes();
        let string_len = bytes.len();

        for i in 0..string_len {
            if bytes[i] == b'/' {
                return i;
            }

            for suffix in &self.additional_suffixes {
                let suffix_bytes = suffix.as_bytes();
                if bytes[i..].starts_with(suffix_bytes) {
                    return i;
                }
            }
        }

        string_len
    }

    pub fn memory_error_and_abort() -> ! {
        eprintln!("readline: out of virtual memory");
        std::process::abort()
    }
}
