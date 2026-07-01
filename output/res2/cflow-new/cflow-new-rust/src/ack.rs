pub struct Ack;

impl Ack {
    pub type u_long = u64;

    pub fn compute(a: Self::u_long, b: Self::u_long) -> Self::u_long {
        if a == 0 {
            b + 1
        } else if b == 0 {
            Self::compute(a - 1, 1)
        } else {
            Self::compute(a - 1, Self::compute(a, b - 1))
        }
    }
}
