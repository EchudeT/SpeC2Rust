pub struct Bitrotate;

impl Bitrotate {
    #[inline]
    pub fn left_u8(value: u8, shift: u32) -> u8 {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_u8(value: u8, shift: u32) -> u8 {
        value.rotate_right(shift)
    }

    #[inline]
    pub fn left_u16(value: u16, shift: u32) -> u16 {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_u16(value: u16, shift: u32) -> u16 {
        value.rotate_right(shift)
    }

    #[inline]
    pub fn left_u32(value: u32, shift: u32) -> u32 {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_u32(value: u32, shift: u32) -> u32 {
        value.rotate_right(shift)
    }

    #[inline]
    pub fn left_u64(value: u64, shift: u32) -> u64 {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_u64(value: u64, shift: u32) -> u64 {
        value.rotate_right(shift)
    }

    #[inline]
    pub fn left_u128(value: u128, shift: u32) -> u128 {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_u128(value: u128, shift: u32) -> u128 {
        value.rotate_right(shift)
    }

    #[inline]
    pub fn left_usize(value: usize, shift: u32) -> usize {
        value.rotate_left(shift)
    }

    #[inline]
    pub fn right_usize(value: usize, shift: u32) -> usize {
        value.rotate_right(shift)
    }
}
