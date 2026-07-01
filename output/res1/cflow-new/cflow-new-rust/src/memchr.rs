pub struct Memchr;

impl Memchr {
    pub fn memchr(haystack: &[u8], needle: i32) -> Option<usize> {
        let target = needle as u8;
        haystack.iter().position(|&byte| byte == target)
    }
}
