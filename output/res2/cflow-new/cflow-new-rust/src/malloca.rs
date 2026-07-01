pub struct Malloca;

impl Malloca {
    pub fn alloc(n: usize) -> Option<Vec<u8>> {
        let capacity = if n == 0 { 1 } else { n };
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(capacity).is_err() {
            return None;
        }
        buffer.resize(n, 0);
        Some(buffer)
    }

    pub fn free(buffer: Option<Vec<u8>>) {
        drop(buffer);
    }
}
