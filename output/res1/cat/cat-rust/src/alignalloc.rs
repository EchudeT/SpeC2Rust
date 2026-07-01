pub struct Alignalloc;

impl Alignalloc {
    pub fn align_down(value: usize, alignment: usize) -> usize {
        if alignment == 0 {
            value
        } else {
            value & !(alignment - 1)
        }
    }

    pub fn address_of_pointer_to_malloced(aligned_address: usize) -> usize {
        let pointer_size = core::mem::size_of::<usize>();
        let pointer_alignment = core::mem::align_of::<usize>();
        Self::align_down(
            aligned_address
                .saturating_sub(1)
                .saturating_sub(pointer_size),
            pointer_alignment,
        )
    }

    pub fn allocate_aligned(alignment: usize, size: usize) -> Option<Vec<u8>> {
        let effective_alignment = alignment.max(1);
        if !effective_alignment.is_power_of_two() {
            return None;
        }

        let extra = effective_alignment.checked_sub(1)?;
        let total = size
            .checked_add(extra)?
            .checked_add(core::mem::size_of::<usize>())?;

        Some(vec![0_u8; total])
    }

    pub fn free_aligned(allocation: Option<Vec<u8>>) {
        drop(allocation);
    }
}
