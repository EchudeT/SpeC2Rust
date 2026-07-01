pub struct Ialloc;

impl Ialloc {
    pub fn checked_add(a: isize, b: isize) -> Option<isize> {
        a.checked_add(b)
    }

    pub fn checked_mul(a: isize, b: isize) -> Option<isize> {
        a.checked_mul(b)
    }

    pub fn nonnegative_len(len: isize) -> Option<usize> {
        usize::try_from(len).ok()
    }

    pub fn is_valid_len(len: isize) -> bool {
        Self::nonnegative_len(len).is_some()
    }

    pub fn empty() -> Vec<u8> {
        Vec::new()
    }

    pub fn with_len(len: isize) -> Option<Vec<u8>> {
        let len = Self::nonnegative_len(len)?;
        Some(vec![0; len])
    }

    pub fn with_capacity(len: isize) -> Option<Vec<u8>> {
        let len = Self::nonnegative_len(len)?;
        let mut bytes = Vec::new();
        if bytes.try_reserve_exact(len).is_err() {
            return None;
        }
        Some(bytes)
    }

    pub fn resize(mut bytes: Vec<u8>, new_len: isize) -> Option<Vec<u8>> {
        let new_len = Self::nonnegative_len(new_len)?;
        if new_len > bytes.len() {
            let additional = new_len - bytes.len();
            if bytes.try_reserve_exact(additional).is_err() {
                return None;
            }
        }
        bytes.resize(new_len, 0);
        Some(bytes)
    }

    pub fn duplicate(data: &[u8]) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        if out.try_reserve_exact(data.len()).is_err() {
            return None;
        }
        out.extend_from_slice(data);
        Some(out)
    }

    pub fn duplicate_with_nul(data: &[u8]) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        if out.try_reserve_exact(data.len().checked_add(1)?).is_err() {
            return None;
        }
        out.extend_from_slice(data);
        out.push(0);
        Some(out)
    }

    pub fn duplicate_str(text: &str) -> Option<String> {
        let mut out = String::new();
        if out.try_reserve(text.len()).is_err() {
            return None;
        }
        out.push_str(text);
        Some(out)
    }
}
