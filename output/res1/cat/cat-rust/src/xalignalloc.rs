use crate::alignalloc::Alignalloc;
use crate::xalloc_die::XallocDie;

pub struct Xalignalloc;

impl Xalignalloc {
    pub fn allocate(alignment: usize, size: usize) -> Vec<u8> {
        match Alignalloc::allocate_aligned(alignment, size) {
            Some(allocation) => allocation,
            None => XallocDie::xalloc_die(),
        }
    }
}
