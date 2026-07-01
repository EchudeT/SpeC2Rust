pub struct Float;

impl Float {
    pub const F32_MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    pub const F64_MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    pub const F32_DIGITS: u32 = f32::DIGITS;
    pub const F64_DIGITS: u32 = f64::DIGITS;
    pub const F32_EPSILON: f32 = f32::EPSILON;
    pub const F64_EPSILON: f64 = f64::EPSILON;
    pub const F32_MIN: f32 = f32::MIN;
    pub const F32_MAX: f32 = f32::MAX;
    pub const F64_MIN: f64 = f64::MIN;
    pub const F64_MAX: f64 = f64::MAX;

    pub fn max_long_double_like() -> f64 {
        f64::MAX
    }

    pub fn split_long_double_like_max() -> (f64, f64) {
        #[cfg(all(
            any(target_arch = "powerpc", target_arch = "powerpc64"),
            any(target_os = "aix", target_os = "linux")
        ))]
        {
            let hi = f64::MAX;
            let lo = f64::MAX / 134_217_728.0 / 134_217_728.0;
            return (hi, lo);
        }

        (f64::MAX, 0.0)
    }
}
