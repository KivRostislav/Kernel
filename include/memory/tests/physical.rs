extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::ptr::addr_of_mut;
use crate::include::memory::{BLOCK_SIZE, MemoryRegion, PhysicalAddress, PhysicalMemoryManager};
use crate::include::memory::physical::{Allocate, Free};
use crate::include::memory::physical::bitmaps::Bitmaps;
use crate::include::memory::physical::State;

fn is_free_block(address: PhysicalAddress, regions: &[MemoryRegion]) -> bool {
    for region in regions {
        if region.state == State::USED { continue; }
        for x in 0..region.block_count {
            if region.address + (BLOCK_SIZE * x) == address {
                return true;
            }
        }
    }
    false
}

fn is_free_region(address: PhysicalAddress, block_count: usize, regions: &[MemoryRegion]) -> bool {
    for x in 0..block_count {
        if is_free_block(address + (BLOCK_SIZE * x), regions) == false { return false; }
    }
    true
}

pub fn PhysicalMemoryManager_init() -> Result<(), String> {
    let regions = [
        MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },
        MemoryRegion { address: 0x00005000, block_count: 5, state: State::USED },
        MemoryRegion { address: 0x0000A000, block_count: 5, state: State::UNUSED },
    ];

    let bitmaps_physical_memory_manager_result = PhysicalMemoryManager::init(&regions);
    if bitmaps_physical_memory_manager_result.is_none() { return Err(String::from("fail to init BitmapsPhysicalMemoryManager")); }
    Ok(())
}

pub fn PhysicalMemoryManager_allocate() -> Result<(), String> {
    let regions = [
        MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },
        MemoryRegion { address: 0x00005000, block_count: 5, state: State::USED },
        MemoryRegion { address: 0x0000A000, block_count: 5, state: State::UNUSED },
    ];

    let bitmaps_physical_memory_manager_result = PhysicalMemoryManager::init(&regions);
    if bitmaps_physical_memory_manager_result.is_none() { return Err(String::from("fail to init BitmapsPhysicalMemoryManager")); }
    let mut bitmaps_physical_memory_manager = bitmaps_physical_memory_manager_result.unwrap();

    let address_result = bitmaps_physical_memory_manager.allocate(BLOCK_SIZE + 1);
    if address_result.is_none() { return Err(String::from("fail to allocate block")); }
    let address = address_result.unwrap();
    if is_free_region(address, 2, &regions) { return Err(format!("return used memory region. Address: {:X} Block count: [{}]", address, 2))  }
    Ok(())
}

pub fn PhysicalMemoryManager_free() -> Result<(), String> {
    let regions = [
        MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },
        MemoryRegion { address: 0x00005000, block_count: 5, state: State::USED },
        MemoryRegion { address: 0x0000A000, block_count: 5, state: State::UNUSED },
    ];

    let bitmaps_physical_memory_manager_result = PhysicalMemoryManager::init(&regions);
    if bitmaps_physical_memory_manager_result.is_none() { return Err(String::from("fail to init BitmapsPhysicalMemoryManager")); }
    let mut bitmaps_physical_memory_manager = bitmaps_physical_memory_manager_result.unwrap();
    bitmaps_physical_memory_manager.free(regions[2].address, regions[2].block_count * BLOCK_SIZE);

    if is_free_region(regions[2].address, regions[2].block_count, &regions) == false {
        return Err(format!("don't mark region as unused. Address: [{:X}]. Block count: [{}]", regions[2].address, regions[2].block_count));
    }
    Ok(())
}

pub fn bitmaps_mark_block() -> Result<(), String> {
    let mut bitmaps_array: [u8; 2] = [0xFF, 0x00];
    let mut bitmaps = Bitmaps::init(addr_of_mut!(bitmaps_array[0]), 2);

    Bitmaps::mark_block(0, State::UNUSED);
    Bitmaps::mark_block(8, State::USED);

    if Bitmaps::block_state(0).unwrap() == State::UNUSED &&
        Bitmaps::block_state(8).unwrap() == State::USED { return Ok(()); }

    Err(String::from("don't change block state"))
}

pub fn bitmaps_block_state() -> Result<(), String> {
    let mut bitmaps_array: [u8; 2] = [0xFF, 0x00];
    let bitmaps = Bitmaps::init(addr_of_mut!(bitmaps_array[0]), 2);

    if Bitmaps::block_state(0).unwrap() == State::USED ||
        Bitmaps::block_state(8).unwrap() == State::UNUSED { return Ok(()); }

    Err(String::from("return wrong block state"))
}

pub fn bitmaps_first_free() -> Result<(), String> {
    let page_count = 2;
    let mut bitmaps_array: [u8; 3] = [0xFF, 0x00, 0xFF];
    let bitmaps = Bitmaps::init(addr_of_mut!(bitmaps_array[0]), 3);

    let page = Bitmaps::first_free(page_count);
    if page.is_none() { return Err(format!("fail to find the first free blocks. Page count: [{}]", page_count)); }
    let address = page.unwrap();
    if Bitmaps::block_state(Bitmaps::block(address)).unwrap() != State::USED || Bitmaps::block_state(Bitmaps::block(address + BLOCK_SIZE)).unwrap() != State::USED {
        return Err(format!("return address and don't mark it as used. Address: [{:X}] Page count: [{}]", address, page_count))
    }

    Ok(())
}