use crate::include::memory::{BLOCK_SIZE, PhysicalMemoryManager, State};
use crate::include::memory::physical::{Allocate, PhysicalAddress};
use crate::include::memory::physical::bitmaps::Bitmaps;

impl Allocate for PhysicalMemoryManager {
    fn allocate(&self, size: usize) -> Option<PhysicalAddress> {
        let address = Bitmaps::first_free(size.div_ceil(BLOCK_SIZE))?;
        Bitmaps::mark_block(Bitmaps::block(address), State::USED);
        Some(address)
    }
}