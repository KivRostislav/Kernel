use crate::include::memory::r#virtual::{Entry, Map, VirtualMemoryManager};
use crate::include::memory::{PhysicalAddress, VirtualAddress};
use crate::include::memory::physical::{Allocate, GLOBAL_PHYSICAL_MEMORY_MANAGER};
use crate::include::memory::r#virtual::attributes::PageTableAttributes;
use crate::include::memory::r#virtual::entry::Entries;

impl Map for VirtualMemoryManager {
    fn map(&self, virtual_address: VirtualAddress, physical_address: PhysicalAddress) -> Result<(), &'static str> {
        let page_table = Entries::page_table(virtual_address as u64);
        let page_directory_table = Entries::page_directory_table(virtual_address as u64);
        let page_directory_pointer_table = Entries::page_directory_pointer_table(virtual_address as u64);
        let page_map_level_4 = Entries::page_map_level_4(virtual_address as u64);

        let page_map_level_4_entry = get_or_create(self.pml4_pointer, page_map_level_4.try_into().unwrap())?;
        let page_directory_pointer_table_entry = get_or_create(Entries::get_address(*page_map_level_4_entry) as *mut Entry, page_directory_pointer_table.try_into().unwrap())?;
        let page_directory_table_entry = get_or_create(Entries::get_address(*page_directory_pointer_table_entry) as *mut Entry, page_directory_table.try_into().unwrap())?;
        let page_table_entry = get_or_create(Entries::get_address(*page_directory_table_entry) as *mut Entry, page_table.try_into().unwrap())?;
        Entries::set_address(page_table_entry, physical_address);
        Entries::set_attributes(page_table_entry, PageTableAttributes::ACCESSED);

        Ok(())
    }
}

fn get_or_create(table_pointer: *mut Entry, index: usize) -> Result<&'static mut Entry, &'static str> {
    let entry = Entries::get_entry(table_pointer, index);
    if Entries::test_attributes(*entry, PageTableAttributes::ACCESSED) == false {
        let page = unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER.as_ref().unwrap() }.allocate(1);
        if page.is_none() { return Err("Out of memory") }
        Entries::set_address(entry, page.unwrap() as PhysicalAddress);
        Entries::set_attributes(entry, PageTableAttributes::ACCESSED);
    }
    Ok(entry)
}
