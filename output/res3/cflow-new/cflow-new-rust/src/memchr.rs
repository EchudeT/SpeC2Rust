pub struct Memchr;

impl Memchr {
    pub fn memchr(haystack: &[u8], needle: u8) -> Option<usize> {
        haystack.iter().position(|&byte| byte == needle)
    }
}
