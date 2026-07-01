pub struct Alignalloc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    bytes: Vec<u8>,
    start: usize,
    size: usize,
    alignment: usize,
}

impl Allocation {
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[self.start..self.start + self.size]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let end = self.start + self.size;
        &mut self.bytes[self.start..end]
    }

    pub fn aligned_offset(&self) -> usize {
        self.start
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn alignment(&self) -> usize {
        self.alignment
    }
}

impl Alignalloc {
    pub fn align_down(address: usize, alignment: usize) -> usize {
        if alignment == 0 {
            address
        } else {
            address & !(alignment - 1)
        }
    }

    pub fn address_of_pointer_to_malloced(aligned_address: usize) -> usize {
        let pointer_size = core::mem::size_of::<usize>();
        let pointer_align = core::mem::align_of::<usize>();
        let candidate = aligned_address.saturating_sub(pointer_size);
        Self::align_down(candidate, pointer_align)
    }

    pub fn allocate_aligned(alignment: usize, size: usize) -> Option<Allocation> {
        if alignment == 0 || !alignment.is_power_of_two() {
            return None;
        }

        let reserve = size.checked_add(alignment)?;
        let bytes = vec![0u8; reserve.max(1)];

        let base = bytes.as_ptr() as usize;
        let aligned = base.checked_add(alignment - 1).map(|n| n & !(alignment - 1))?;
        let start = aligned.checked_sub(base)?;

        if start.checked_add(size)? > bytes.len() {
            return None;
        }

        Some(Allocation {
            bytes,
            start,
            size,
            alignment,
        })
    }

    pub fn free_aligned(allocation: Option<Allocation>) {
        drop(allocation);
    }
}
