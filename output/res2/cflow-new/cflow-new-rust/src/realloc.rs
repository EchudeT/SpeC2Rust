pub struct Realloc;

impl Realloc {
    pub fn rpl_realloc(buffer: Option<Vec<u8>>, new_size: usize) -> Option<Vec<u8>> {
        match buffer {
            None => Some(vec![0; new_size]),
            Some(mut existing) => {
                if new_size == 0 {
                    return None;
                }

                if new_size > isize::MAX as usize {
                    return None;
                }

                existing.resize(new_size, 0);
                Some(existing)
            }
        }
    }
}
