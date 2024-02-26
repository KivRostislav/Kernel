use crate::include::memory::tests::physical::{bitmaps_block_state, bitmaps_first_free, bitmaps_mark_block, PhysicalMemoryManager_allocate, PhysicalMemoryManager_free, PhysicalMemoryManager_init};
use crate::tests::{TestDescriptor};
use lazy_static::lazy_static;
use crate::include::memory::tests::r#virtual::{Entries_page_map_level_4, Entries_get_address, Entries_get_attributes, Entries_offset, Entries_page_directory_pointer_table, Entries_page_directory_table, Entries_page_table, Entries_set_address, Entries_set_attributes, Entries_test_attributes, Entries_unset_attributes, VirtualMemoryManager_init, VirtualMemoryManager_map, VirtualMemoryManager_unmap};

pub fn space() -> &'static[TestDescriptor] { &*TESTS }

lazy_static! {
    static ref TESTS: [TestDescriptor; 20] = [
        TestDescriptor { function: bitmaps_first_free, name: "physical_memory_bitmaps_first_free" },
        TestDescriptor { function: bitmaps_block_state, name: "physical_memory_bitmaps_block_state" },
        TestDescriptor { function: bitmaps_mark_block, name: "physical_memory_bitmaps_mark_block" },
        TestDescriptor { function: PhysicalMemoryManager_init, name: "PhysicalMemoryManager_init" },
        TestDescriptor { function: PhysicalMemoryManager_free, name: "PhysicalMemoryManager_free" },
        TestDescriptor { function: PhysicalMemoryManager_allocate, name: "PhysicalMemoryManager_allocate" },

        TestDescriptor { function: VirtualMemoryManager_init, name: "VirtualMemoryManager_init" },
        TestDescriptor { function: VirtualMemoryManager_map, name: "VirtualMemoryManager_map" },
        TestDescriptor { function: VirtualMemoryManager_unmap, name: "VirtualMemoryManager_unmap" },

        TestDescriptor { function: Entries_get_address, name: "Entries_get_address" },
        TestDescriptor { function: Entries_set_address, name: "Entries_set_address" },
        TestDescriptor { function: Entries_get_attributes, name: "Entries_get_attributes" },
        TestDescriptor { function: Entries_set_attributes, name: "Entries_set_attributes" },
        TestDescriptor { function: Entries_unset_attributes, name: "Entries_unset_attributes" },
        TestDescriptor { function: Entries_test_attributes, name: "Entries_test_attributes" },
        TestDescriptor { function: Entries_offset, name: "Entries_offset" },
        TestDescriptor { function: Entries_page_table, name: "Entries_page_table" },
        TestDescriptor { function: Entries_page_directory_table, name: "Entries_page_directory_table" },
        TestDescriptor { function: Entries_page_directory_pointer_table, name: "Entries_page_directory_pointer_table" },
        TestDescriptor { function: Entries_page_map_level_4, name: "Entries_page_map_level_4" },

    ];
}


pub mod physical;
pub mod r#virtual;