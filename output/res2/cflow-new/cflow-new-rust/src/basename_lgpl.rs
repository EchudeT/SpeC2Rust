pub struct BasenameLgpl;

impl BasenameLgpl {
    pub fn base_len(name: &str) -> usize {
        let bytes = name.as_bytes();
        let mut len = bytes.len();
        while len > 0 && bytes[len - 1] == b'/' {
            len -= 1;
        }
        len
    }

    pub fn last_component(name: &str) -> &str {
        let bytes = name.as_bytes();
        let mut start = 0usize;
        let mut i = 0usize;

        while i < bytes.len() {
            if bytes[i] == b'/' {
                while i < bytes.len() && bytes[i] == b'/' {
                    i += 1;
                }
                if i < bytes.len() {
                    start = i;
                }
            } else {
                i += 1;
            }
        }

        &name[start..]
    }

    pub fn basename(name: &str) -> &str {
        let component = Self::last_component(name);
        let trimmed_len = Self::base_len(component);
        &component[..trimmed_len]
    }
}
