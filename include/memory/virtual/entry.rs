use core::ops::Add;
use crate::include::memory::r#virtual::Entry;
use crate::include::memory::PhysicalAddress;

pub struct Entries { }
impl Entries {
    pub fn get_address(entry: Entry) -> u64 { (entry & 0x000FFFFFFFFFF000) >> 12 }
    pub fn set_address(entry: &mut Entry, address: PhysicalAddress) { *entry = (*entry & !0x7FFFFFFFFFFFF000) | <usize as TryInto<u64>>::try_into(address & 0x7FFFFFFFFFFFF000).unwrap(); }

    pub fn get_attributes(entry: Entry) -> u64 { entry & 0x00000FFF }
    pub fn set_attributes(entry: &mut Entry, attributes: u64) { *entry = (*entry & 0xFFFFFFFFFFFFF000) | attributes }
    pub fn unset_attributes(entry: &mut Entry, attributes: u64) { *entry = !((*entry & !0x7FFFFFFFFFFFF000) & attributes) & *entry }
    pub fn test_attributes(entry: Entry, attributes: u64) -> bool { ((entry & 0x00000FFF) & attributes) == attributes }

    pub fn offset(entry: Entry) -> u64 { entry & 0x00000FFF }
    pub fn page_table(entry: Entry) -> u64 { (entry & 0x001FF000) >> 12}
    pub fn page_directory_table(entry: Entry) -> u64 { (entry & 0x3FE00000) >> 21 }
    pub fn page_directory_pointer_table(entry: Entry) -> u64 { (entry & 0x7FC0000000) >> 30 }
    pub fn page_map_level_4(entry: Entry) -> u64 { (entry & 0xFF8000000000) >> 39 }

    pub fn get_entry(table: *mut Entry, index: usize) -> &'static mut Entry { unsafe { table.add(index).as_mut().unwrap() } }
}