use core::slice;
use crate::include::assembly::assembly_write_to_cr3_register;
use crate::include::memory::{BLOCK_SIZE, PhysicalAddress, PhysicalMemoryManager, VirtualAddress};
use crate::include::memory::physical::{Allocate, GLOBAL_PHYSICAL_MEMORY_MANAGER};

pub mod entry;
pub mod map;
pub mod unmap;
pub mod attributes;

pub static mut GLOBAL_VIRTUAL_MEMORY_MANAGER: *const VirtualMemoryManager = 0x00 as *const VirtualMemoryManager;

static PAGE_TABLE_SIZE: usize = 512;
pub type Entry = u64;

pub trait Map {
    fn map(&self, virtual_address: VirtualAddress, physical_address: PhysicalAddress) -> Result<(), &'static str>;
}

pub trait Unmap {
    fn unmap(&self, virtual_address: VirtualAddress);
}

pub struct VirtualMemoryManager{
    pub pml4_pointer: *mut Entry,
}

impl VirtualMemoryManager {
    pub fn init() -> Option<Self> {
        let page = unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER.as_ref().unwrap() }.allocate(1);
        if page.is_none() { return None; }
        unsafe { slice::from_raw_parts_mut(page.unwrap() as *mut u8, BLOCK_SIZE).fill(0x00) }
        Some(Self { pml4_pointer: page.unwrap() as *mut Entry })
    }

    pub fn enable(&self) {
        assembly_write_to_cr3_register(self.pml4_pointer as u64);
    }
}