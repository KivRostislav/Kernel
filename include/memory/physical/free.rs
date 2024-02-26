use crate::include::memory::{BLOCK_SIZE, PhysicalMemoryManager, State};
use crate::include::memory::physical::{Free, PhysicalAddress};
use crate::include::memory::physical::bitmaps::Bitmaps;

impl Free for PhysicalMemoryManager {
    fn free(&self, address: PhysicalAddress, size: usize) {
        // TODO maybe delete this code.
        // TODO because perhaps memory manager mustn't fill all free memory with 0x00.
        let slice = unsafe { core::slice::from_raw_parts_mut(address as *mut u8, size) };
        slice.fill(0x00);

        Bitmaps::mark_region(address, size.div_ceil(BLOCK_SIZE), State::UNUSED);
    }
}