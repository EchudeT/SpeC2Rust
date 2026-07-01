pub struct Malloc;

impl Malloc {
    pub fn rpl_malloc(n: usize) -> Option<Vec<u8>> {
        let size = if n == 0 { 1 } else { n };

        let mut result = Vec::<u8>::new();
        if result.try_reserve_exact(size).is_err() {
            return None;
        }
        result.resize(size, 0);
        Some(result)
    }
}
