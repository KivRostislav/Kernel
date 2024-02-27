#![no_main]
#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate alloc;
extern crate log;
extern crate uefi;


#[path = "../include/mod.rs"] mod include;

mod tests;
mod memory;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use core::convert::TryInto;
use core::mem::size_of;
use core::ptr::{addr_of, addr_of_mut, NonNull};
use core::slice;
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use aml::{AmlContext, AmlHandle, AmlName, DebugVerbosity, Handler};
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::FrameBuffer;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::{guid, Identify};
use uefi::proto::device_path::media::PartitionSignature::Guid;
use crate::include::memory::{PhysicalMemoryManager, BLOCK_SIZE, MemoryRegion, State, VirtualMemoryManager, GLOBAL_PHYSICAL_MEMORY_MANAGER, GLOBAL_VIRTUAL_MEMORY_MANAGER, Allocate, Map, PhysicalAddress, VirtualAddress, Unmap};

#[derive(Clone, Copy)]
pub struct ACPI { }

impl AcpiHandler for ACPI {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        let virtual_memory_manager = unsafe { GLOBAL_VIRTUAL_MEMORY_MANAGER.as_ref().unwrap() };
        for x in 0..size.div_ceil(BLOCK_SIZE) {
            virtual_memory_manager.map(physical_address + (BLOCK_SIZE * x) as VirtualAddress, physical_address + (BLOCK_SIZE * x) as PhysicalAddress).unwrap();
        }

        PhysicalMapping::new(physical_address, NonNull::new(physical_address as *mut T).unwrap(), size, size.div_ceil(BLOCK_SIZE), *self)
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {
        let virtual_memory_manager = unsafe { GLOBAL_VIRTUAL_MEMORY_MANAGER.as_ref().unwrap() };
        for x in 0..region.mapped_length().div_ceil(BLOCK_SIZE) {
            virtual_memory_manager.unmap(region.physical_start() + (BLOCK_SIZE * x) as VirtualAddress);
        }
    }
}


#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    //tests::TestManager::run();
    //loop { }

    let boot_services = system_table.boot_services();
    let memory_map_size = boot_services.memory_map_size();
    let buffer_pointer = boot_services.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, memory_map_size.map_size.div_ceil(BLOCK_SIZE)).unwrap();
    let buffer = unsafe { slice::from_raw_parts_mut(buffer_pointer as *mut u8, memory_map_size.map_size) };
    let memory_map = boot_services.memory_map(buffer).unwrap();
    let entries = memory_map.entries();
    for entry in entries.clone() {
        info!("type: {:?}, address: {:X}, page count: {}", entry.ty, entry.phys_start, entry.page_count)
    }

    let entry_count = entries.clone().count();
    let page_count = (size_of::<MemoryRegion>() * entry_count).div_ceil(BLOCK_SIZE);
    let mut memory_page_count = 0usize;
    let region_buffer_pointer = boot_services.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, page_count).unwrap();
    let mut regions = unsafe { Vec::from_raw_parts(region_buffer_pointer as *mut MemoryRegion, 0, entry_count) };
    for entry in entries {
        memory_page_count = memory_page_count + <u64 as TryInto<usize>>::try_into(entry.page_count).unwrap();
        let status = if entry.ty == MemoryType::CONVENTIONAL { State::UNUSED } else { State::USED };
        regions.push(MemoryRegion { address: entry.phys_start.try_into().unwrap(), state: status, block_count: entry.page_count.try_into().unwrap() });
    }
    info!("---------------------------------------------------------------------------------------------");
    for region in &regions {
        info!("type: {:?}, address: {:X}, page count: {}", region.state, region.address, region.block_count);
    }

    let physical_memory_manager = PhysicalMemoryManager::init(regions.as_slice()).unwrap();
    unsafe { GLOBAL_PHYSICAL_MEMORY_MANAGER = &physical_memory_manager; }

    let virtual_memory_manager = VirtualMemoryManager::init().unwrap();
    unsafe { GLOBAL_VIRTUAL_MEMORY_MANAGER = &virtual_memory_manager; }
    let acpi_guid = guid!("8868e871-e4f1-11d3-bc22-0080c73c8881");
    let mut rsdp_address = 0x00;
    for x in system_table.config_table() {
        if x.guid == acpi_guid { rsdp_address = x.address as usize; }
    }

    info!("{:X}", rsdp_address);
    let handler = ACPI { };
    let result = unsafe { AcpiTables::from_rsdp(handler, rsdp_address) }.unwrap();
    let mut handlerx = HandlerX { };

    let mut context = AmlContext::new(unsafe { Box::from_raw(addr_of_mut!(handlerx)) }, DebugVerbosity::Scopes);
    context.parse_table(unsafe { slice::from_raw_parts(result.dsdt().unwrap().address as *const u8, result.dsdt().unwrap().length as usize) }).unwrap();
    info!("----------------------------------------------------------------");
    info!("{:?}", context.namespace);
    info!("{:?}", context.namespace.get_by_path(&AmlName::from_str("\\_SB_.PCI0").unwrap()).unwrap());

    loop {}
    Status::SUCCESS
}

pub fn print(buffer: &FrameBuffer, string: String) {

}

struct HandlerX { }
impl Handler for HandlerX {
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (address as *const u16).read() }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (address as *const u32).read() }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (address as *const u64).read() }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        todo!()
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        todo!()
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        todo!()
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        todo!()
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        todo!()
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        todo!()
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        todo!()
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        todo!()
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        todo!()
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        todo!()
    }

    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        todo!()
    }

    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        todo!()
    }

    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        todo!()
    }

    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        todo!()
    }

    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        todo!()
    }

    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        todo!()
    }
}


/***
let boot_services = system_table.boot_services();
        let graphics_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>();
        let open = OpenProtocolParams { agent: image_handle, handle: graphics_handle.unwrap(), controller: None };
        let mut graphics = unsafe { boot_services.open_protocol::<GraphicsOutput>(open, OpenProtocolAttributes::GetProtocol).unwrap() };
        frame_buffer_base = graphics.frame_buffer().as_mut_ptr();
        frame_buffer_size = graphics.frame_buffer().size();
        drop(graphics);
        let memory_map_length = boot_services.memory_map_size();
        let mut buffer = Vec::<u8>::with_capacity(memory_map_length.map_size);
        unsafe { buffer.set_len(memory_map_length.map_size); };

        let mut memory_map = boot_services.memory_map(&mut buffer).unwrap();

        for entry in memory_map.entries() {
            info!("type: {:?}, address: {:X}, page count: {}", entry.ty, entry.phys_start, entry.page_count)
        }

            let (system_tabe, memory_map) = system_table.exit_boot_services(MemoryType::LOADER_DATA);
    let mut count = 0;
    for x in memory_map.entries() {
        if x.ty == MemoryType::CONVENTIONAL { count = count + 1; }
    }

    let xd = if count > 1 { 0x00 } else { 0xFF };
    unsafe {
        let mut x = 0usize;
        loop {
            frame_buffer_base.offset(x.try_into().unwrap()).write(xd);
            frame_buffer_base.offset(<usize as TryInto<isize>>::try_into(x).unwrap() + 1isize).write(0);
            frame_buffer_base.offset(<usize as TryInto<isize>>::try_into(x).unwrap() + 2isize).write(0);
            x = x + 3;

            if x == frame_buffer_size - 3 {
                loop {}
            }
        }
    }

    loop { }
    Status::SUCCESS
***/

