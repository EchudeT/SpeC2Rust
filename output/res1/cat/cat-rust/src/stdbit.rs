pub struct Stdbit;

impl Stdbit {
    #[inline]
    pub const fn bit_width_u8() -> u32 {
        u8::BITS
    }

    #[inline]
    pub const fn bit_width_u16() -> u32 {
        u16::BITS
    }

    #[inline]
    pub const fn bit_width_u32() -> u32 {
        u32::BITS
    }

    #[inline]
    pub const fn bit_width_u64() -> u32 {
        u64::BITS
    }

    #[inline]
    pub const fn bit_width_u128() -> u32 {
        u128::BITS
    }

    #[inline]
    pub const fn bit_width_usize() -> u32 {
        usize::BITS
    }

    #[inline]
    pub const fn leading_zeros_u8(value: u8) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn leading_zeros_u16(value: u16) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn leading_zeros_u32(value: u32) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn leading_zeros_u64(value: u64) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn leading_zeros_u128(value: u128) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn leading_zeros_usize(value: usize) -> u32 {
        value.leading_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_u8(value: u8) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_u16(value: u16) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_u32(value: u32) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_u64(value: u64) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_u128(value: u128) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn trailing_zeros_usize(value: usize) -> u32 {
        value.trailing_zeros()
    }

    #[inline]
    pub const fn ones_u8(value: u8) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn ones_u16(value: u16) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn ones_u32(value: u32) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn ones_u64(value: u64) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn ones_u128(value: u128) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn ones_usize(value: usize) -> u32 {
        value.count_ones()
    }

    #[inline]
    pub const fn zeros_u8(value: u8) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn zeros_u16(value: u16) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn zeros_u32(value: u32) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn zeros_u64(value: u64) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn zeros_u128(value: u128) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn zeros_usize(value: usize) -> u32 {
        value.count_zeros()
    }

    #[inline]
    pub const fn has_single_bit_u8(value: u8) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn has_single_bit_u16(value: u16) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn has_single_bit_u32(value: u32) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn has_single_bit_u64(value: u64) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn has_single_bit_u128(value: u128) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn has_single_bit_usize(value: usize) -> bool {
        value.is_power_of_two()
    }

    #[inline]
    pub const fn bit_floor_u8(value: u8) -> u8 {
        if value == 0 {
            0
        } else {
            1u8 << (u8::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_floor_u16(value: u16) -> u16 {
        if value == 0 {
            0
        } else {
            1u16 << (u16::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_floor_u32(value: u32) -> u32 {
        if value == 0 {
            0
        } else {
            1u32 << (u32::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_floor_u64(value: u64) -> u64 {
        if value == 0 {
            0
        } else {
            1u64 << (u64::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_floor_u128(value: u128) -> u128 {
        if value == 0 {
            0
        } else {
            1u128 << (u128::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_floor_usize(value: usize) -> usize {
        if value == 0 {
            0
        } else {
            1usize << (usize::BITS - 1 - value.leading_zeros())
        }
    }

    #[inline]
    pub const fn bit_ceil_u8(value: u8) -> u8 {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_u8(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_ceil_u16(value: u16) -> u16 {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_u16(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_ceil_u32(value: u32) -> u32 {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_u32(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_ceil_u64(value: u64) -> u64 {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_u64(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_ceil_u128(value: u128) -> u128 {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_u128(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_ceil_usize(value: usize) -> usize {
        if value <= 1 {
            1
        } else {
            let floor = Self::bit_floor_usize(value - 1);
            floor << 1
        }
    }

    #[inline]
    pub const fn bit_width_value_u8(value: u8) -> u32 {
        if value == 0 {
            0
        } else {
            u8::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn bit_width_value_u16(value: u16) -> u32 {
        if value == 0 {
            0
        } else {
            u16::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn bit_width_value_u32(value: u32) -> u32 {
        if value == 0 {
            0
        } else {
            u32::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn bit_width_value_u64(value: u64) -> u32 {
        if value == 0 {
            0
        } else {
            u64::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn bit_width_value_u128(value: u128) -> u32 {
        if value == 0 {
            0
        } else {
            u128::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn bit_width_value_usize(value: usize) -> u32 {
        if value == 0 {
            0
        } else {
            usize::BITS - value.leading_zeros()
        }
    }

    #[inline]
    pub const fn first_leading_one_u8(value: u8) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_one_u16(value: u16) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_one_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_one_u64(value: u64) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_one_u128(value: u128) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_one_usize(value: usize) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_u8(value: u8) -> Option<u32> {
        if value == u8::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_u16(value: u16) -> Option<u32> {
        if value == u16::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_u32(value: u32) -> Option<u32> {
        if value == u32::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_u64(value: u64) -> Option<u32> {
        if value == u64::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_u128(value: u128) -> Option<u32> {
        if value == u128::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_leading_zero_usize(value: usize) -> Option<u32> {
        if value == usize::MAX {
            None
        } else {
            Some((!value).leading_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_u8(value: u8) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_u16(value: u16) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_u32(value: u32) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_u64(value: u64) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_u128(value: u128) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_one_usize(value: usize) -> Option<u32> {
        if value == 0 {
            None
        } else {
            Some(value.trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_u8(value: u8) -> Option<u32> {
        if value == u8::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_u16(value: u16) -> Option<u32> {
        if value == u16::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_u32(value: u32) -> Option<u32> {
        if value == u32::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_u64(value: u64) -> Option<u32> {
        if value == u64::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_u128(value: u128) -> Option<u32> {
        if value == u128::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn first_trailing_zero_usize(value: usize) -> Option<u32> {
        if value == usize::MAX {
            None
        } else {
            Some((!value).trailing_zeros())
        }
    }

    #[inline]
    pub const fn count_leading_ones_u8(value: u8) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_leading_ones_u16(value: u16) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_leading_ones_u32(value: u32) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_leading_ones_u64(value: u64) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_leading_ones_u128(value: u128) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_leading_ones_usize(value: usize) -> u32 {
        value.leading_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_u8(value: u8) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_u16(value: u16) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_u32(value: u32) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_u64(value: u64) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_u128(value: u128) -> u32 {
        value.trailing_ones()
    }

    #[inline]
    pub const fn count_trailing_ones_usize(value: usize) -> u32 {
        value.trailing_ones()
    }
}
