pub struct Stdbit;

impl Stdbit {
    #[inline]
    pub fn has_single_bit_u8(value: u8) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub fn has_single_bit_u16(value: u16) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub fn has_single_bit_u32(value: u32) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub fn has_single_bit_u64(value: u64) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub fn has_single_bit_u128(value: u128) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub fn bit_width_u8(value: u8) -> u32 {
        u8::BITS - value.leading_zeros()
    }

    #[inline]
    pub fn bit_width_u16(value: u16) -> u32 {
        u16::BITS - value.leading_zeros()
    }

    #[inline]
    pub fn bit_width_u32(value: u32) -> u32 {
        u32::BITS - value.leading_zeros()
    }

    #[inline]
    pub fn bit_width_u64(value: u64) -> u32 {
        u64::BITS - value.leading_zeros()
    }

    #[inline]
    pub fn bit_width_u128(value: u128) -> u32 {
        u128::BITS - value.leading_zeros()
    }

    #[inline]
    pub fn leading_zeros_u8(value: u8) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub fn leading_zeros_u16(value: u16) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub fn leading_zeros_u32(value: u32) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub fn leading_zeros_u64(value: u64) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub fn leading_zeros_u128(value: u128) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub fn leading_ones_u8(value: u8) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub fn leading_ones_u16(value: u16) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub fn leading_ones_u32(value: u32) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub fn leading_ones_u64(value: u64) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub fn leading_ones_u128(value: u128) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub fn trailing_zeros_u8(value: u8) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub fn trailing_zeros_u16(value: u16) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub fn trailing_zeros_u32(value: u32) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub fn trailing_zeros_u64(value: u64) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub fn trailing_zeros_u128(value: u128) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub fn trailing_ones_u8(value: u8) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub fn trailing_ones_u16(value: u16) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub fn trailing_ones_u32(value: u32) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub fn trailing_ones_u64(value: u64) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub fn trailing_ones_u128(value: u128) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub fn first_leading_zero_u8(value: u8) -> Option<u32> {
        let n = value.leading_ones();
        (n < u8::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_zero_u16(value: u16) -> Option<u32> {
        let n = value.leading_ones();
        (n < u16::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_zero_u32(value: u32) -> Option<u32> {
        let n = value.leading_ones();
        (n < u32::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_zero_u64(value: u64) -> Option<u32> {
        let n = value.leading_ones();
        (n < u64::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_zero_u128(value: u128) -> Option<u32> {
        let n = value.leading_ones();
        (n < u128::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_one_u8(value: u8) -> Option<u32> {
        let n = value.leading_zeros();
        (n < u8::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_one_u16(value: u16) -> Option<u32> {
        let n = value.leading_zeros();
        (n < u16::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_one_u32(value: u32) -> Option<u32> {
        let n = value.leading_zeros();
        (n < u32::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_one_u64(value: u64) -> Option<u32> {
        let n = value.leading_zeros();
        (n < u64::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_leading_one_u128(value: u128) -> Option<u32> {
        let n = value.leading_zeros();
        (n < u128::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_zero_u8(value: u8) -> Option<u32> {
        let n = value.trailing_ones();
        (n < u8::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_zero_u16(value: u16) -> Option<u32> {
        let n = value.trailing_ones();
        (n < u16::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_zero_u32(value: u32) -> Option<u32> {
        let n = value.trailing_ones();
        (n < u32::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_zero_u64(value: u64) -> Option<u32> {
        let n = value.trailing_ones();
        (n < u64::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_zero_u128(value: u128) -> Option<u32> {
        let n = value.trailing_ones();
        (n < u128::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_one_u8(value: u8) -> Option<u32> {
        let n = value.trailing_zeros();
        (n < u8::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_one_u16(value: u16) -> Option<u32> {
        let n = value.trailing_zeros();
        (n < u16::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_one_u32(value: u32) -> Option<u32> {
        let n = value.trailing_zeros();
        (n < u32::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_one_u64(value: u64) -> Option<u32> {
        let n = value.trailing_zeros();
        (n < u64::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn first_trailing_one_u128(value: u128) -> Option<u32> {
        let n = value.trailing_zeros();
        (n < u128::BITS).then_some(n + 1)
    }

    #[inline]
    pub fn count_zeros_u8(value: u8) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub fn count_zeros_u16(value: u16) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub fn count_zeros_u32(value: u32) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub fn count_zeros_u64(value: u64) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub fn count_zeros_u128(value: u128) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub fn count_ones_u8(value: u8) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub fn count_ones_u16(value: u16) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub fn count_ones_u32(value: u32) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub fn count_ones_u64(value: u64) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub fn count_ones_u128(value: u128) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub fn ceil_power_of_two_u8(value: u8) -> Option<u8> {
        if value <= 1 {
            Some(1)
        } else {
            1u8.checked_shl(Self::bit_width_u8(value - 1))
        }
    }

    #[inline]
    pub fn ceil_power_of_two_u16(value: u16) -> Option<u16> {
        if value <= 1 {
            Some(1)
        } else {
            1u16.checked_shl(Self::bit_width_u16(value - 1))
        }
    }

    #[inline]
    pub fn ceil_power_of_two_u32(value: u32) -> Option<u32> {
        if value <= 1 {
            Some(1)
        } else {
            1u32.checked_shl(Self::bit_width_u32(value - 1))
        }
    }

    #[inline]
    pub fn ceil_power_of_two_u64(value: u64) -> Option<u64> {
        if value <= 1 {
            Some(1)
        } else {
            1u64.checked_shl(Self::bit_width_u64(value - 1))
        }
    }

    #[inline]
    pub fn ceil_power_of_two_u128(value: u128) -> Option<u128> {
        if value <= 1 {
            Some(1)
        } else {
            1u128.checked_shl(Self::bit_width_u128(value - 1))
        }
    }

    #[inline]
    pub fn floor_power_of_two_u8(value: u8) -> Option<u8> {
        if value == 0 {
            None
        } else {
            Some(1u8 << (Self::bit_width_u8(value) - 1))
        }
    }

    #[inline]
    pub fn floor_power_of_two_u16(value: u16) -> Option<u16> {
        if value == 0 {
            None
        } else {
            Some(1u16 << (Self::bit_width_u16(value) - 1))
        }
    }

    #[inline]
    pub fn floor_power_of_two_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(1u32 << (Self::bit_width_u32(value) - 1))
        }
    }

    #[inline]
    pub fn floor_power_of_two_u64(value: u64) -> Option<u64> {
        if value == 0 {
            None
        } else {
            Some(1u64 << (Self::bit_width_u64(value) - 1))
        }
    }

    #[inline]
    pub fn floor_power_of_two_u128(value: u128) -> Option<u128> {
        if value == 0 {
            None
        } else {
            Some(1u128 << (Self::bit_width_u128(value) - 1))
        }
    }

    #[inline]
    pub fn byte_count_u8(_: u8) -> usize {
        core::mem::size_of::<u8>()
    }

    #[inline]
    pub fn byte_count_u16(_: u16) -> usize {
        core::mem::size_of::<u16>()
    }

    #[inline]
    pub fn byte_count_u32(_: u32) -> usize {
        core::mem::size_of::<u32>()
    }

    #[inline]
    pub fn byte_count_u64(_: u64) -> usize {
        core::mem::size_of::<u64>()
    }

    #[inline]
    pub fn byte_count_u128(_: u128) -> usize {
        core::mem::size_of::<u128>()
    }

    #[inline]
    pub fn bit_count_u8(_: u8) -> u32 {
        u8::BITS
    }

    #[inline]
    pub fn bit_count_u16(_: u16) -> u32 {
        u16::BITS
    }

    #[inline]
    pub fn bit_count_u32(_: u32) -> u32 {
        u32::BITS
    }

    #[inline]
    pub fn bit_count_u64(_: u64) -> u32 {
        u64::BITS
    }

    #[inline]
    pub fn bit_count_u128(_: u128) -> u32 {
        u128::BITS
    }
}
