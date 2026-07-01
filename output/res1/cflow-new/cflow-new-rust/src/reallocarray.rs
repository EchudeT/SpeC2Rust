pub struct Reallocarray;

impl Reallocarray {
    pub fn resize_array(buffer: Option<Vec<u8>>, nmemb: usize, size: usize) -> Option<Vec<u8>> {
        let nbytes = nmemb.checked_mul(size)?;

        match buffer {
            Some(mut data) => {
                if nbytes == 0 {
                    data.clear();
                    data.shrink_to_fit();
                    Some(data)
                } else if nbytes > data.len() {
                    data.resize(nbytes, 0);
                    Some(data)
                } else {
                    data.truncate(nbytes);
                    Some(data)
                }
            }
            None => Some(vec![0; nbytes]),
        }
    }
}
