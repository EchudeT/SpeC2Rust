pub struct Float;

impl Float {
    pub const F64_MAX: f64 = f64::MAX;
    pub const F64_MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    pub const F64_DIGITS: u32 = f64::DIGITS;
    pub const F64_EPSILON: f64 = f64::EPSILON;
    pub const F64_MIN: f64 = f64::MIN_POSITIVE;

    pub fn long_double_max_parts() -> [f64; 2] {
        [f64::MAX, f64::MAX / 134_217_728.0 / 134_217_728.0]
    }

    pub fn long_double_max() -> f64 {
        let [hi, lo] = Self::long_double_max_parts();
        hi + lo
    }
}
