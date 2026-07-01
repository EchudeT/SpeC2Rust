pub struct Stdbit;

impl Stdbit {
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
    pub fn first_leading_one_u8(value: u8) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_one_u16(value: u16) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_one_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_one_u64(value: u64) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_one_u128(value: u128) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_zero_u8(value: u8) -> Option<u32> {
        if value == u8::MAX {
            None
        } else {
            Some((!value).leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_zero_u16(value: u16) -> Option<u32> {
        if value == u16::MAX {
            None
        } else {
            Some((!value).leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_zero_u32(value: u32) -> Option<u32> {
        if value == u32::MAX {
            None
        } else {
            Some((!value).leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_zero_u64(value: u64) -> Option<u32> {
        if value == u64::MAX {
            None
        } else {
            Some((!value).leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_leading_zero_u128(value: u128) -> Option<u32> {
        if value == u128::MAX {
            None
        } else {
            Some((!value).leading_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_one_u8(value: u8) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_one_u16(value: u16) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_one_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_one_u64(value: u64) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_one_u128(value: u128) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_zero_u8(value: u8) -> Option<u32> {
        if value == u8::MAX {
            None
        } else {
            Some((!value).trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_zero_u16(value: u16) -> Option<u32> {
        if value == u16::MAX {
            None
        } else {
            Some((!value).trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_zero_u32(value: u32) -> Option<u32> {
        if value == u32::MAX {
            None
        } else {
            Some((!value).trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_zero_u64(value: u64) -> Option<u32> {
        if value == u64::MAX {
            None
        } else {
            Some((!value).trailing_zeros() + 1)
        }
    }

    #[inline]
    pub fn first_trailing_zero_u128(value: u128) -> Option<u32> {
        if value == u128::MAX {
            None
        } else {
            Some((!value).trailing_zeros() + 1)
        }
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
    pub fn floor_pow2_u8(value: u8) -> Option<u8> {
        if value == 0 {
            None
        } else {
            Some(1u8 << (Self::bit_width_u8(value) - 1))
        }
    }

    #[inline]
    pub fn floor_pow2_u16(value: u16) -> Option<u16> {
        if value == 0 {
            None
        } else {
            Some(1u16 << (Self::bit_width_u16(value) - 1))
        }
    }

    #[inline]
    pub fn floor_pow2_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(1u32 << (Self::bit_width_u32(value) - 1))
        }
    }

    #[inline]
    pub fn floor_pow2_u64(value: u64) -> Option<u64> {
        if value == 0 {
            None
        } else {
            Some(1u64 << (Self::bit_width_u64(value) - 1))
        }
    }

    #[inline]
    pub fn floor_pow2_u128(value: u128) -> Option<u128> {
        if value == 0 {
            None
        } else {
            Some(1u128 << (Self::bit_width_u128(value) - 1))
        }
    }

    #[inline]
    pub fn ceil_pow2_u8(value: u8) -> Option<u8> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn ceil_pow2_u16(value: u16) -> Option<u16> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn ceil_pow2_u32(value: u32) -> Option<u32> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn ceil_pow2_u64(value: u64) -> Option<u64> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn ceil_pow2_u128(value: u128) -> Option<u128> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn byteswap_u16(value: u16) -> u16 {
        value.swap_bytes()
    }

    #[inline]
    pub fn byteswap_u32(value: u32) -> u32 {
        value.swap_bytes()
    }

    #[inline]
    pub fn byteswap_u64(value: u64) -> u64 {
        value.swap_bytes()
    }

    #[inline]
    pub fn byteswap_u128(value: u128) -> u128 {
        value.swap_bytes()
    }

    #[inline]
    pub fn bit_ceil_usize(value: usize) -> Option<usize> {
        value.checked_next_power_of_two()
    }

    #[inline]
    pub fn bit_floor_usize(value: usize) -> Option<usize> {
        if value == 0 {
            None
        } else {
            Some(1usize << (usize::BITS - value.leading_zeros() - 1))
        }
    }

    #[inline]
    pub fn bit_width_usize(value: usize) -> u32 {
        usize::BITS - value.leading_zeros()
    }
}
