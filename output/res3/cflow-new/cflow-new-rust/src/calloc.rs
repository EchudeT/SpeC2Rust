pub struct Calloc;

impl Calloc {
    pub fn rpl_calloc(n: usize, s: usize) -> Option<Vec<u8>> {
        let (n, s) = if n == 0 || s == 0 { (1, 1) } else { (n, s) };
        let total = n.checked_mul(s)?;
        Some(vec![0; total])
    }
}
