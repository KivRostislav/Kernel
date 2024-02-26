use core::arch::asm;

pub fn assembly_write_to_cr3_register(value: u64) {
    unsafe { asm!("mov cr3, {value}", value = in(reg) value) }
}

pub fn assembly_read_cr3_register() -> u64 {
    let mut value = 0u64;
    unsafe { asm!("mov {value}, cr3", value = out(reg) value) }
    value
}