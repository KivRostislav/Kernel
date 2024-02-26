use crate::include::memory::r#virtual::{Entry, Unmap, VirtualMemoryManager};
use crate::include::memory::r#virtual::attributes::PageTableAttributes;
use crate::include::memory::r#virtual::entry::Entries;
use crate::include::memory::VirtualAddress;

impl Unmap for VirtualMemoryManager {
    fn unmap(&self, virtual_address: VirtualAddress) {
        let page_table = Entries::page_table(virtual_address as u64);
        let page_directory_table = Entries::page_directory_table(virtual_address as u64);
        let page_directory_pointer_table = Entries::page_directory_pointer_table(virtual_address as u64);
        let page_map_level_4 = Entries::page_map_level_4(virtual_address as u64);

        let page_map_level_4_entry = Entries::get_entry(self.pml4_pointer, page_map_level_4.try_into().unwrap());
        if Entries::test_attributes(*page_map_level_4_entry, PageTableAttributes::ACCESSED) == false { return; }

        let page_directory_pointer_table_entry = Entries::get_entry(Entries::get_address(*page_map_level_4_entry) as *mut Entry, page_directory_pointer_table.try_into().unwrap());
        if Entries::test_attributes(*page_directory_pointer_table_entry, PageTableAttributes::ACCESSED) == false { return; }

        let page_directory_table_entry = Entries::get_entry(Entries::get_address(*page_directory_pointer_table_entry) as *mut Entry, page_directory_table.try_into().unwrap());
        if Entries::test_attributes(*page_directory_table_entry, PageTableAttributes::ACCESSED) == false { return; }

        let page_table_entry = Entries::get_entry(Entries::get_address(*page_directory_table_entry) as *mut Entry, page_table.try_into().unwrap());
        Entries::unset_attributes(page_table_entry, PageTableAttributes::ACCESSED);
    }
}