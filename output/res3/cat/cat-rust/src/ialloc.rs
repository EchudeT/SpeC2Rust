pub struct Ialloc;

impl Ialloc {
    pub fn fits_size(count: i64, element_size: usize) -> bool {
        usize::try_from(count)
            .ok()
            .and_then(|n| n.checked_mul(element_size))
            .is_some()
    }

    pub fn byte_len(count: i64, element_size: usize) -> Option<usize> {
        usize::try_from(count)
            .ok()
            .and_then(|n| n.checked_mul(element_size))
    }

    pub fn allocate_bytes(count: i64, element_size: usize) -> Option<Vec<u8>> {
        let len = Self::byte_len(count, element_size)?;
        Some(vec![0; len])
    }

    pub fn resize_bytes(buffer: &mut Vec<u8>, count: i64, element_size: usize) -> bool {
        match Self::byte_len(count, element_size) {
            Some(new_len) => {
                buffer.resize(new_len, 0);
                true
            }
            None => false,
        }
    }

    pub fn duplicate_bytes(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    pub fn duplicate_with_nul(data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len().saturating_add(1));
        out.extend_from_slice(data);
        out.push(0);
        out
    }
}
