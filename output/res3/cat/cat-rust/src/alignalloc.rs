pub struct Alignalloc;

impl Alignalloc {
    pub fn align_down(address: usize, alignment: usize) -> usize {
        if alignment == 0 {
            address
        } else {
            address - (address % alignment)
        }
    }

    pub fn address_of_pointer_to_malloced(aligned_address: usize) -> usize {
        let pointer_bytes = std::mem::size_of::<usize>();
        aligned_address.saturating_sub(pointer_bytes)
    }

    pub fn allocate_aligned(alignment: usize, size: usize) -> Option<Vec<u8>> {
        if alignment == 0 {
            return None;
        }

        let total = size.checked_add(alignment)?;
        Some(vec![0; total])
    }

    pub fn free_aligned(block: Option<Vec<u8>>) {
        drop(block);
    }
}
