pub struct Bitrotate;

impl Bitrotate {
    #[inline]
    pub fn rotl64(x: u64, n: u32) -> u64 {
        x.rotate_left(n % 64)
    }

    #[inline]
    pub fn rotr64(x: u64, n: u32) -> u64 {
        x.rotate_right(n % 64)
    }

    #[inline]
    pub fn rotl32(x: u32, n: u32) -> u32 {
        x.rotate_left(n % 32)
    }

    #[inline]
    pub fn rotr32(x: u32, n: u32) -> u32 {
        x.rotate_right(n % 32)
    }

    #[inline]
    pub fn rotl_usize(x: usize, n: u32) -> usize {
        x.rotate_left(n % usize::BITS)
    }

    #[inline]
    pub fn rotr_usize(x: usize, n: u32) -> usize {
        x.rotate_right(n % usize::BITS)
    }

    #[inline]
    pub fn rotl16(x: u16, n: u32) -> u16 {
        x.rotate_left(n % 16)
    }

    #[inline]
    pub fn rotr16(x: u16, n: u32) -> u16 {
        x.rotate_right(n % 16)
    }

    #[inline]
    pub fn rotl8(x: u8, n: u32) -> u8 {
        x.rotate_left(n % 8)
    }

    #[inline]
    pub fn rotr8(x: u8, n: u32) -> u8 {
        x.rotate_right(n % 8)
    }
}
