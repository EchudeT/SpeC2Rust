pub struct Realloc;

impl Realloc {
    pub fn rpl_realloc(buffer: Option<Vec<u8>>, new_size: usize) -> Option<Vec<u8>> {
        if buffer.is_none() {
            return if new_size == 0 {
                Some(Vec::new())
            } else {
                let mut new_buffer = Vec::new();
                if new_buffer.try_reserve_exact(new_size).is_err() {
                    None
                } else {
                    new_buffer.resize(new_size, 0);
                    Some(new_buffer)
                }
            };
        }

        if new_size == 0 {
            return None;
        }

        let mut buffer = buffer?;
        if buffer.len() == new_size {
            return Some(buffer);
        }

        if new_size > buffer.len() {
            let additional = new_size - buffer.len();
            if buffer.try_reserve_exact(additional).is_err() {
                return None;
            }
        }

        buffer.resize(new_size, 0);
        Some(buffer)
    }
}
