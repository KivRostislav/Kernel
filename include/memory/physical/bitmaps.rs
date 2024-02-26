extern crate alloc;

use crate::include::memory::{BLOCK_SIZE, PhysicalAddress, State};
use crate::include::memory::physical::BLOCKS_PER_BYTE;

static mut BITMAPS_POINTER: *mut u8 = 0x00 as *mut u8;
static mut BITMAPS_SIZE: usize = 0;

pub struct Bitmaps { }
impl Bitmaps {
    pub fn init(pointer: *mut u8, size: usize) {
        unsafe { BITMAPS_POINTER = pointer; }
        unsafe { BITMAPS_SIZE = size; }
    }

    pub fn address(block_index: usize) -> PhysicalAddress { block_index + BLOCK_SIZE }
    pub fn block(address: PhysicalAddress) -> usize { address.div_euclid(BLOCK_SIZE) }


    pub fn mark_block(block_index: usize, state: State) {
        let bitmap_index = block_index.div_euclid(BLOCKS_PER_BYTE);
        if bitmap_index >= unsafe { BITMAPS_SIZE } { return; }
        unsafe {
            match state {
                State::USED => BITMAPS_POINTER.add(bitmap_index).write(BITMAPS_POINTER.add(bitmap_index).read() | (0b10000000 >> (block_index % BLOCKS_PER_BYTE))),
                State::UNUSED => BITMAPS_POINTER.add(bitmap_index).write(BITMAPS_POINTER.add(bitmap_index).read() & !(0b10000000 >> (block_index % BLOCKS_PER_BYTE))),
                _ => return
            }
        }
    }

    pub fn mark_region(start_block_index: usize, block_count: usize, state: State) {
        for x in 0..block_count { // TODO: optimize this code
            Bitmaps::mark_block(start_block_index + (BLOCK_SIZE * x), state);
        }
    }

    pub fn block_state(block_index: usize) -> Option<State> {
        if block_index.div_euclid(8) >= unsafe { BITMAPS_SIZE } { return None; }
        let bitmap = unsafe { BITMAPS_POINTER.add(block_index.div_euclid(8)).read() };
        Some(State::try_from((bitmap & (0b10000000 >> (block_index % 8))) << (block_index % 8)).unwrap())

    }

    pub fn first_free(count: usize) -> Option<PhysicalAddress> {
        let mut start_index = usize::MAX;
        let mut total_count = 0usize;
        for bitmap_index in 0..unsafe { BITMAPS_SIZE } {
            if total_count >= count { return Some(Bitmaps::address(usize::from(bitmap_index * 8)))  }
            if unsafe { BITMAPS_POINTER.add(bitmap_index).read() } == 0xFF { continue; }
            if unsafe { BITMAPS_POINTER.add(bitmap_index).read() } == 0x00 {
                start_index = if start_index == usize::MAX { usize::from(bitmap_index * 8) } else { start_index };
                total_count = total_count + 8;
                continue;
            }

            for bit in 0..8 {
                if unsafe { BITMAPS_POINTER.add(bitmap_index).read() } & (0b10000000 >> bit) != 0x00 {
                    start_index = usize::MAX;
                    total_count = 0;
                }

                start_index = if start_index == usize::MAX { usize::from(bit + (bitmap_index * 8)) } else { start_index };
                total_count = total_count + 1;

                if total_count >= count { return Some(Bitmaps::address(usize::from(bit + (bitmap_index * 8))))  }
            }
        }

        if total_count >= count { return Some(Bitmaps::address(start_index))  }
        None
    }

}
