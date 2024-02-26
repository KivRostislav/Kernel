mod physical;
mod r#virtual;

pub mod tests;

pub use crate::include::memory::physical::{GLOBAL_PHYSICAL_MEMORY_MANAGER, PhysicalMemoryManager, MemoryRegion, State, Allocate, Free};

pub use crate::include::memory::r#virtual::{GLOBAL_VIRTUAL_MEMORY_MANAGER, VirtualMemoryManager, Map, Unmap};
pub use crate::include::memory::r#virtual::attributes::PageTableAttributes;


pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub const BLOCK_SIZE: usize = 4096; // 4KiB




