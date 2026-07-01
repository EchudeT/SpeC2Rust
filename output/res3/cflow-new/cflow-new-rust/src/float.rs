#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Float;

impl Float {
    pub const F32_MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    pub const F64_MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    pub const F32_DIGITS: u32 = f32::DIGITS;
    pub const F64_DIGITS: u32 = f64::DIGITS;
    pub const F32_EPSILON: f32 = f32::EPSILON;
    pub const F64_EPSILON: f64 = f64::EPSILON;
    pub const F32_MAX: f32 = f32::MAX;
    pub const F64_MAX: f64 = f64::MAX;

    pub fn new() -> Self {
        Self
    }

    pub fn long_double_max() -> f64 {
        f64::MAX
    }
}

impl Default for Float {
    fn default() -> Self {
        Self::new()
    }
}
