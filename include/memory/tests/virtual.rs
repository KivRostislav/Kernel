extern crate alloc;

use alloc::string::String;
use core::ptr::eq;
use log::info;
use uefi::proto::device_path::build::end::Entire;
use crate::include::memory::{GLOBAL_PHYSICAL_MEMORY_MANAGER, MemoryRegion, PhysicalAddress, PhysicalMemoryManager, State, VirtualAddress};
use crate::include::memory::r#virtual::{Entry, Map, Unmap, VirtualMemoryManager};
use crate::include::memory::r#virtual::attributes::PageTableAttributes;
use crate::include::memory::r#virtual::entry::Entries;

pub fn VirtualMemoryManager_init() -> Result<(), String> {
    let regions = [MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },];

    let physical_memory_manager_option = PhysicalMemoryManager::init(&regions);
    if physical_memory_manager_option.is_none() { return Err(String::from("fail to init physical memory manager")); }
    let physical_memory_manager = physical_memory_manager_option.unwrap();
    unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER = &physical_memory_manager; }

    let virtual_memory_manager = VirtualMemoryManager::init();
    if virtual_memory_manager.is_none() { return Err(String::from("fail to init vitual memory manager. Out of memory")); }
    Ok(())
}

pub fn VirtualMemoryManager_map() -> Result<(), String> {
    let regions = [MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },];

    let physical_memory_manager_option = PhysicalMemoryManager::init(&regions);
    if physical_memory_manager_option.is_none() { return Err(String::from("fail to init physical memory manager")); }
    let physical_memory_manager = physical_memory_manager_option.unwrap();
    unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER = &physical_memory_manager; }

    let virtual_memory_manager_option = VirtualMemoryManager::init();
    if virtual_memory_manager_option.is_none() { return Err(String::from("fail to init virtual memory manager. Out of memory")); }
    let virtual_memory_manager = virtual_memory_manager_option.unwrap();

    let virtual_address = 0x00002000 as VirtualAddress;
    let physical_address = 0x00003000 as PhysicalAddress;
    let result = virtual_memory_manager.map(virtual_address, physical_address);
    if result.is_err() { return Err(String::from("fail to map addresses. Out of memory")); }

    let page_table = Entries::page_table(virtual_address as u64);
    let page_directory_table = Entries::page_directory_table(virtual_address as u64);
    let page_directory_pointer_table =Entries::page_directory_pointer_table(virtual_address as u64);
    let page_map_level_4 = Entries::page_map_level_4(virtual_address as u64);

    let page_map_level_4_entry = Entries::get_entry(virtual_memory_manager.pml4_pointer, page_map_level_4.try_into().unwrap());
    if Entries::test_attributes(*page_map_level_4_entry, PageTableAttributes::ACCESSED) == false { return Err(String::from("no accessible pml4 entry")); }

    let page_directory_pointer_table_entry = Entries::get_entry(Entries::get_address(*page_map_level_4_entry) as *mut Entry, page_directory_pointer_table.try_into().unwrap());
    if Entries::test_attributes(*page_directory_pointer_table_entry, PageTableAttributes::ACCESSED) == false { return Err(String::from("no accessible page directory pointer table entry")); }

    let page_directory_table_entry = Entries::get_entry(Entries::get_address(*page_directory_pointer_table_entry) as *mut Entry, page_directory_table.try_into().unwrap());
    if Entries::test_attributes(*page_directory_table_entry, PageTableAttributes::ACCESSED) == false { return Err(String::from("no accessible page directory table entry")); }

    let page_table_entry = Entries::get_entry(Entries::get_address(*page_directory_table_entry) as *mut Entry, page_table.try_into().unwrap());

    if Entries::test_attributes(*page_table_entry, PageTableAttributes::ACCESSED) == false { return Err(String::from("no accessible page table entry")); }
    if Entries::get_address(*page_table_entry) != (physical_address >> 12) as u64 { return Err(String::from("page table entry contains wrong physical address")); }

    Ok(())
}

pub fn VirtualMemoryManager_unmap() -> Result<(), String> {
    let regions = [MemoryRegion { address: 0x00000000, block_count: 5, state: State::UNUSED },];

    let physical_memory_manager_option = PhysicalMemoryManager::init(&regions);
    if physical_memory_manager_option.is_none() { return Err(String::from("fail to init physical memory manager")); }
    let physical_memory_manager = physical_memory_manager_option.unwrap();
    unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER = &physical_memory_manager }

    let virtual_memory_manager_option = VirtualMemoryManager::init();
    if virtual_memory_manager_option.is_none() { return Err(String::from("fail to init virtual memory manager. Out of memory")); }
    let virtual_memory_manager = virtual_memory_manager_option.unwrap();

    let virtual_address = 0x00002000 as VirtualAddress;
    let physical_address = 0x00003000 as PhysicalAddress;
    let map_result = virtual_memory_manager.map(virtual_address, physical_address);
    if map_result.is_err() { return Err(String::from("fail to map addresses. Out of memory")); }

    virtual_memory_manager.unmap(virtual_address);

    let page_table = Entries::page_table(virtual_address as u64);
    let page_directory_table = Entries::page_directory_table(virtual_address as u64);
    let page_directory_pointer_table =Entries::page_directory_pointer_table(virtual_address as u64);
    let page_map_level_4 = Entries::page_map_level_4(virtual_address as u64);

    let page_map_level_4_entry = Entries::get_entry(virtual_memory_manager.pml4_pointer, page_map_level_4.try_into().unwrap());
    if Entries::test_attributes(*page_map_level_4_entry, PageTableAttributes::ACCESSED) == false { return Ok(()); }

    let page_directory_pointer_table_entry = Entries::get_entry(Entries::get_address(*page_map_level_4_entry) as *mut Entry, page_directory_pointer_table.try_into().unwrap());
    if Entries::test_attributes(*page_directory_pointer_table_entry, PageTableAttributes::ACCESSED) == false { return Ok(()) }

    let page_directory_table_entry = Entries::get_entry(Entries::get_address(*page_directory_pointer_table_entry) as *mut Entry, page_directory_table.try_into().unwrap());
    if Entries::test_attributes(*page_directory_table_entry, PageTableAttributes::ACCESSED) == false { return Ok(()) }

    let page_table_entry = Entries::get_entry(Entries::get_address(*page_directory_table_entry) as *mut Entry, page_table.try_into().unwrap());
    if Entries::test_attributes(*page_table_entry, PageTableAttributes::ACCESSED) == false { return Ok(()) }

    Err(String::from("virtual memory manager contains addresses map"))
}

pub fn Entries_get_address() -> Result<(), String> {
    let address_1 = Entries::get_address(0xFFFFFFFFFFFFFFFF);
    let address_2 = Entries::get_address(0x0000000000000000);
    let address_3 = Entries::get_address(0x0000F0F0F0F0F000);
    if address_1 != 0xFFFFFFFFFF { return Err(String::from("return wrong address")); }
    if address_2 != 0x0000000000 { return Err(String::from("return wrong address")); }
    if address_3 != 0x0F0F0F0F0F { return Err(String::from("return wrong address")); }
    Ok(())
}

pub fn Entries_set_address() -> Result<(), String> {
    let mut entry = 0xFFFFFFFFFFFFFFFFu64;
    Entries::set_address(&mut entry, 0x00);
    if Entries::get_address(entry) != 0x00 { return Err(String::from("don't change address")); }
    Ok(())
}

pub fn Entries_get_attributes() -> Result<(), String> {
    let attributes_1 = Entries::get_attributes(0xFFF);
    let attributes_2 = Entries::get_attributes(0x000);
    let attributes_3 = Entries::get_attributes(0xF0F);
    if attributes_1 != 0xFFF { return Err(String::from("return wrong attributes")); }
    if attributes_2 != 0x000 { return Err(String::from("return wrong attributes")); }
    if attributes_3 != 0xF0F { return Err(String::from("return wrong attributes")); }
    Ok(())
}

pub fn Entries_set_attributes() -> Result<(), String> {
    let mut entry = 0xFFFu64;
    Entries::set_attributes(&mut entry, 0x000);
    if Entries::get_attributes(entry) != 0x00 { return Err(String::from("don't change attributes")); }
    Ok(())
}

pub fn Entries_unset_attributes() -> Result<(), String> {
    let mut entry = 0xFFFu64;
    Entries::unset_attributes(&mut entry, PageTableAttributes::ACCESSED);
    if Entries::test_attributes(entry, PageTableAttributes::ACCESSED) == true { return Err(String::from("don't unset attribute")); }
    Ok(())
}

pub fn Entries_test_attributes() -> Result<(), String> {
    let mut entry = PageTableAttributes::ACCESSED | PageTableAttributes::PAGE_ATTRIBUTES_TABLE;
    let result = Entries::test_attributes(entry, PageTableAttributes::ACCESSED) && Entries::test_attributes(entry, PageTableAttributes::PAGE_ATTRIBUTES_TABLE);
    if result == false { return Err(String::from("return wrong attributes status")); }
    Ok(())
}

pub fn Entries_offset() -> Result<(), String> {
    let offset_1 = Entries::offset(0xFFF);
    let offset_2 = Entries::offset(0x000);
    let offset_3 = Entries::offset(0xF0F);
    if offset_1 != 0xFFF { return Err(String::from("return wrong offset")); }
    if offset_2 != 0x000 { return Err(String::from("return wrong offset")); }
    if offset_3 != 0xF0F { return Err(String::from("return wrong offset")); }
    Ok(())
}

pub fn Entries_page_table() -> Result<(), String> {
    let page_table_1 = Entries::page_table(0xFFFFFFFFFFFFFFFF);
    let page_table_2 = Entries::page_table(0x0000000000000000);
    let page_table_3 = Entries::page_table(0x00000000000FF000);

    if page_table_1 != 0x1FF { return Err(String::from("return wrong page table index")); }
    if page_table_2 != 0x00 { return Err(String::from("return wrong page table index")); }
    if page_table_3 != 0xFF { return Err(String::from("return wrong page table index")); }
    Ok(())
}

pub fn Entries_page_directory_table() -> Result<(), String> {
    let page_directory_table_1 = Entries::page_directory_table(0xFFFFFFFFFFFFFFFF);
    let page_directory_table_2 = Entries::page_directory_table(0x0000000000000000);
    let page_directory_table_3 = Entries::page_directory_table(0x000000001FE00000);

    if page_directory_table_1 != 0x1FF { return Err(String::from("return wrong page table index")); }
    if page_directory_table_2 != 0x00 { return Err(String::from("return wrong page table index")); }
    if page_directory_table_3 != 0xFF { return Err(String::from("return wrong page table index")); }
    Ok(())
}

pub fn Entries_page_directory_pointer_table() -> Result<(), String> {
    let page_directory_pointer_table_1 = Entries::page_directory_pointer_table(0xFFFFFFFFFFFFFFFF);
    let page_directory_pointer_table_2 = Entries::page_directory_pointer_table(0x0000000000000000);
    let page_directory_pointer_table_3 = Entries::page_directory_pointer_table(0x0000003FC0000000);

    if page_directory_pointer_table_1 != 0x1FF { return Err(String::from("return wrong page table index")); }
    if page_directory_pointer_table_2 != 0x00 { return Err(String::from("return wrong page table index")); }
    if page_directory_pointer_table_3 != 0xFF { return Err(String::from("return wrong page table index")); }
    Ok(())
}

pub fn Entries_page_map_level_4() -> Result<(), String> {
    let page_map_level_4_1 = Entries::page_map_level_4(0xFFFFFFFFFFFFFFFF);
    let page_map_level_4_2 = Entries::page_map_level_4(0x0000000000000000);
    let page_map_level_4_3 = Entries::page_map_level_4(0x00007F8000000000);

    if page_map_level_4_1 != 0x1FF { return Err(String::from("return wrong page table index")); }
    if page_map_level_4_2 != 0x00 { return Err(String::from("return wrong page table index")); }
    if page_map_level_4_3 != 0xFF { return Err(String::from("return wrong page table index")); }
    Ok(())
}
