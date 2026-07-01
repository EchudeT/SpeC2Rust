use crate::alignalloc::{Alignalloc, Allocation};
use crate::xalloc_die::XallocDie;

pub struct Xalignalloc;

impl Xalignalloc {
    pub fn allocate(alignment: usize, size: usize) -> Allocation {
        match Alignalloc::allocate_aligned(alignment, size) {
            Some(allocation) => allocation,
            None => XallocDie::xalloc_die(),
        }
    }
}
