pub struct Xsize;

impl Xsize {
    pub const SIZE_MAX: usize = usize::MAX;

    pub fn checked_add(a: usize, b: usize) -> Option<usize> {
        a.checked_add(b)
    }

    pub fn checked_mul(a: usize, b: usize) -> Option<usize> {
        a.checked_mul(b)
    }

    pub fn checked_sub(a: usize, b: usize) -> Option<usize> {
        a.checked_sub(b)
    }

    pub fn sum3(a: usize, b: usize, c: usize) -> Option<usize> {
        Self::checked_add(a, b).and_then(|ab| Self::checked_add(ab, c))
    }

    pub fn product2(a: usize, b: usize) -> Option<usize> {
        Self::checked_mul(a, b)
    }

    pub fn product3(a: usize, b: usize, c: usize) -> Option<usize> {
        Self::checked_mul(a, b).and_then(|ab| Self::checked_mul(ab, c))
    }

    pub fn add_mul(base: usize, factor_a: usize, factor_b: usize) -> Option<usize> {
        Self::checked_mul(factor_a, factor_b).and_then(|prod| Self::checked_add(base, prod))
    }

    pub fn mul_add(a: usize, b: usize, addend: usize) -> Option<usize> {
        Self::checked_mul(a, b).and_then(|prod| Self::checked_add(prod, addend))
    }

    pub fn is_valid_size(value: Option<usize>) -> bool {
        value.is_some()
    }
}
