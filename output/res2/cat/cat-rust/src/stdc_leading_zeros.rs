pub struct StdcLeadingZeros;

impl StdcLeadingZeros {
    pub fn u8(value: u8) -> u32 {
        value.leading_zeros()
    }

    pub fn u16(value: u16) -> u32 {
        value.leading_zeros()
    }

    pub fn u32(value: u32) -> u32 {
        value.leading_zeros()
    }

    pub fn u64(value: u64) -> u32 {
        value.leading_zeros()
    }

    pub fn u128(value: u128) -> u32 {
        value.leading_zeros()
    }

    pub fn usize(value: usize) -> u32 {
        value.leading_zeros()
    }
}
