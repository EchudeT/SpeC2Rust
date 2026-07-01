pub struct Xsize;

impl Xsize {
    pub const SIZE_MAX: usize = usize::MAX;

    pub fn add(a: usize, b: usize) -> Option<usize> {
        a.checked_add(b)
    }

    pub fn mul(a: usize, b: usize) -> Option<usize> {
        a.checked_mul(b)
    }

    pub fn mul_add(a: usize, b: usize, c: usize) -> Option<usize> {
        a.checked_mul(b)?.checked_add(c)
    }

    pub fn saturating_add(a: usize, b: usize) -> usize {
        a.saturating_add(b)
    }

    pub fn saturating_mul(a: usize, b: usize) -> usize {
        a.saturating_mul(b)
    }

    pub fn saturating_mul_add(a: usize, b: usize, c: usize) -> usize {
        a.saturating_mul(b).saturating_add(c)
    }

    pub fn is_add_safe(a: usize, b: usize) -> bool {
        a.checked_add(b).is_some()
    }

    pub fn is_mul_safe(a: usize, b: usize) -> bool {
        a.checked_mul(b).is_some()
    }

    pub fn is_mul_add_safe(a: usize, b: usize, c: usize) -> bool {
        a.checked_mul(b).and_then(|v| v.checked_add(c)).is_some()
    }

    pub fn excess_for_add(a: usize, b: usize) -> usize {
        a.checked_add(b)
            .map(|_| 0)
            .unwrap_or_else(|| a.saturating_add(b).wrapping_sub(Self::SIZE_MAX))
    }

    pub fn excess_for_mul(a: usize, b: usize) -> usize {
        match a.checked_mul(b) {
            Some(_) => 0,
            None => {
                if a == 0 || b == 0 {
                    0
                } else {
                    let limit = Self::SIZE_MAX / a;
                    b.saturating_sub(limit)
                }
            }
        }
    }
}
