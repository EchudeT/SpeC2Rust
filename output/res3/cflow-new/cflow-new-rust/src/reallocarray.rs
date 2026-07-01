use crate::realloc::Realloc;

pub struct Reallocarray;

impl Reallocarray {
    pub fn resize_array(buffer: Option<Vec<u8>>, nmemb: usize, size: usize) -> Option<Vec<u8>> {
        let nbytes = nmemb.checked_mul(size)?;
        Realloc::rpl_realloc(buffer, nbytes)
    }
}
