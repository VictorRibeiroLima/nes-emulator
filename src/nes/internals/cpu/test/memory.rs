use crate::nes::internals::{bus::Bus, cpu::CPU, memory::Memory, opcodes::AddressingMode};

#[test]
fn test_get_memory_addr_immediate() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory(0x0000, 0x10);

    assert_eq!(cpu.get_memory_addr(AddressingMode::IMMEDIATE), 0x0000);
}

#[test]
fn test_get_memory_addr_zero_page() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory(0x0000, 0x10);

    assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE), 0x0010);
}

#[test]
fn test_get_memory_addr_absolute() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory_le(0x0000, 0x20);

    assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE), 0x20);
}

#[test]
fn test_get_memory_addr_zero_page_x() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory(0x0000, 0x20);
    cpu.register_x = 0x05;

    assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE_X), 0x0025);
}

#[test]
fn test_get_memory_addr_zero_page_y() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory(0x0000, 0x20);
    cpu.register_y = 0x02;

    assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE_Y), 0x0022);
}

#[test]
fn test_get_memory_addr_absolute_x() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory_le(0x0000, 0x30);
    cpu.register_x = 0xFF;

    assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE_X), 0x12f);
}

#[test]
fn test_get_memory_addr_absolute_y() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x0000;
    cpu.write_to_memory_le(0x0000, 0x40);
    cpu.register_y = 0xFF;

    assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE_Y), 0x13F);
}

#[test]
fn test_get_memory_addr_indirect_x() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.register_x = 0xFF;
    cpu.program_counter = 0x15;
    cpu.write_to_memory(0x15, 0x60);
    cpu.write_to_memory(0x5F, 0x70);
    cpu.write_to_memory(0x60, 0x71);

    assert_eq!(cpu.get_memory_addr(AddressingMode::INDIRECT_X), 0x7170);
}

#[test]
fn test_get_memory_addr_indirect_y() {
    let bus = Bus::new();
    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x41;
    cpu.register_y = 0x31;
    cpu.write_to_memory(0x41, 0x80);
    cpu.write_to_memory(0x80, 0x90);
    cpu.write_to_memory(0x81, 0xFE);
    assert_eq!(cpu.get_memory_addr(AddressingMode::INDIRECT_Y), 0xFEC1);
}
