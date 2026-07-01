pub struct Ack;

impl Ack {
    pub fn compute(a: u64, b: u64) -> u64 {
        if a == 0 {
            b + 1
        } else if b == 0 {
            Self::compute(a - 1, 1)
        } else {
            Self::compute(a - 1, Self::compute(a, b - 1))
        }
    }
}
