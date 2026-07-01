pub struct BasenameLgpl;

impl BasenameLgpl {
    pub fn last_component(path: &str) -> &str {
        if path.is_empty() {
            return "";
        }

        let bytes = path.as_bytes();
        let mut end = bytes.len();

        while end > 1 && bytes[end - 1] == b'/' {
            end -= 1;
        }

        match bytes[..end].iter().rposition(|&b| b == b'/') {
            Some(pos) => &path[pos + 1..end],
            None => &path[..end],
        }
    }

    pub fn base_len(path: &str) -> usize {
        Self::last_component(path).len()
    }

    pub fn basename(path: &str) -> &str {
        Self::last_component(path)
    }
}
