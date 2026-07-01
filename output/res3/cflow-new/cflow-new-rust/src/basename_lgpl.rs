pub struct BasenameLgpl;

impl BasenameLgpl {
    pub fn base_len(name: &str) -> usize {
        let bytes = name.as_bytes();
        let mut len = bytes.len();

        while len > 1 && bytes[len - 1] == b'/' {
            len -= 1;
        }

        let trimmed = &bytes[..len];

        match trimmed.iter().rposition(|&b| b == b'/') {
            Some(pos) => trimmed[pos + 1..].len(),
            None => trimmed.len(),
        }
    }

    pub fn last_component(name: &str) -> &str {
        let bytes = name.as_bytes();
        let mut start = 0;
        let mut i = 0;

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
        let len = Self::base_len(component);
        &component[..len]
    }
}
