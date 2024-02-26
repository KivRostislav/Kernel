pub mod allocator;
pub mod free;
pub mod bitmaps;

use core::alloc::{GlobalAlloc, Layout};
use crate::include::memory::{BLOCK_SIZE, PhysicalAddress};
use crate::include::memory::physical::bitmaps::Bitmaps;

pub static mut GLOBAL_PHYSICAL_MEMORY_MANAGER: *const PhysicalMemoryManager = 0x00 as *const PhysicalMemoryManager;

pub const BLOCKS_PER_BYTE: usize = 8;

pub trait Allocate {
    fn allocate(&self, size: usize) -> Option<PhysicalAddress>;
}

pub trait Free {
    fn free(&self, address: PhysicalAddress, size: usize);
}

pub struct PhysicalMemoryManager {  }
impl PhysicalMemoryManager {
    pub fn init(regions: &[MemoryRegion]) -> Option<Self> {
        let mut memory_block_count = 0usize;
        for region in regions { memory_block_count = memory_block_count + region.block_count; }

        let bitmaps_size = memory_block_count.div_ceil(BLOCKS_PER_BYTE);
        let mut free_region: Option<&MemoryRegion> = None;
        for region in regions {
            if region.state == State::UNUSED && region.block_count >= bitmaps_size.div_ceil(BLOCK_SIZE) { free_region = Some(region); }
        }

        if free_region.is_none() { return None; }
        Bitmaps::init(free_region.unwrap().address as *mut u8, bitmaps_size);
        for region in regions {
            Bitmaps::mark_region(Bitmaps::block(region.address), region.block_count, region.state);
        }

        Bitmaps::mark_region(free_region.unwrap().address, free_region.unwrap().block_count, State::UNUSED);
        Some(Self { })
    }
}

unsafe impl GlobalAlloc for PhysicalMemoryManager {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout.size()).unwrap() as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.free(ptr as PhysicalAddress, layout.size())
    }
}
pub struct MemoryRegion {
    pub address: PhysicalAddress,
    pub block_count: usize,
    pub state: State,
}

#[repr(transparent)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct State(u8);
impl State {
    pub const USED:   Self = Self(0b10000000);
    pub const UNUSED: Self = Self(0b00000000);

    pub fn try_from(value: u8) -> Option<Self> {
        return match State(value) {
            State::USED => Some(Self(value)),
            State::UNUSED => Some(Self(value)),
            _ => None,
        }
    }
}


