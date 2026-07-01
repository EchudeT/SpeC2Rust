pub struct Memchr;

impl Memchr {
    pub fn memchr(buffer: &[u8], byte: i32) -> Option<usize> {
        let target = byte as u8;
        let word_bytes = core::mem::size_of::<usize>();

        if buffer.is_empty() {
            return None;
        }

        let mut index = 0usize;
        let base_addr = buffer.as_ptr() as usize;

        while index < buffer.len() && (base_addr + index) % word_bytes != 0 {
            if buffer[index] == target {
                return Some(index);
            }
            index += 1;
        }

        let repeated_one = Self::repeated_one();
        let repeated_c = Self::repeated_byte(target);
        let high_bits = repeated_one << 7;

        while index + word_bytes <= buffer.len() {
            let chunk = &buffer[index..index + word_bytes];
            let longword = usize::from_ne_bytes(chunk.try_into().expect("chunk size matches usize"));
            let xored = longword ^ repeated_c;

            if ((xored.wrapping_sub(repeated_one)) & !xored & high_bits) != 0 {
                break;
            }

            index += word_bytes;
        }

        while index < buffer.len() {
            if buffer[index] == target {
                return Some(index);
            }
            index += 1;
        }

        None
    }

    fn repeated_one() -> usize {
        let mut value = 0usize;
        let mut shift = 0usize;

        while shift < usize::BITS as usize {
            value |= 0x01usize << shift;
            shift += 8;
        }

        value
    }

    fn repeated_byte(byte: u8) -> usize {
        let mut value = 0usize;
        let mut shift = 0usize;

        while shift < usize::BITS as usize {
            value |= (byte as usize) << shift;
            shift += 8;
        }

        value
    }
}
