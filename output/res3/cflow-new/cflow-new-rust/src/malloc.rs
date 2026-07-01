pub struct Malloc;

impl Malloc {
    pub fn rpl_malloc(n: usize) -> Option<Vec<u8>> {
        let size = if n == 0 { 1 } else { n };

        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(size).is_err() {
            return None;
        }
        buffer.resize(size, 0);
        Some(buffer)
    }
}
