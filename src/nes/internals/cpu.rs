use std::borrow::Borrow;

use bitflags::bitflags;

use super::{
    memory::Memory,
    opcodes::{AddressingMode, Opcodes},
};

bitflags!(
    pub struct StatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const UNUSED = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
);

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    /* 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction.
    (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
      0b0000_0000
        nvbb_dizc
    */
    pub status: StatusFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Memory for CPU {
    fn read_from_memory(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
    fn write_to_memory(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn get_memory_addr<T: Borrow<AddressingMode>>(&self, mode: T) -> u16 {
        match mode.borrow() {
            AddressingMode::IMMEDIATE => self.program_counter,

            AddressingMode::ZERO_PAGE => self.read_from_memory(self.program_counter) as u16,

            AddressingMode::ABSOLUTE => self.read_from_memory_le(self.program_counter),

            AddressingMode::ZERO_PAGE_X => {
                let pos = self.read_from_memory(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                return addr;
            }
            AddressingMode::ZERO_PAGE_Y => {
                let pos = self.read_from_memory(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                return addr;
            }

            AddressingMode::ABSOLUTE_X => {
                let base = self.read_from_memory_le(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                return addr;
            }
            AddressingMode::ABSOLUTE_Y => {
                let base = self.read_from_memory_le(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                return addr;
            }

            AddressingMode::INDIRECT_X => {
                let base = self.read_from_memory(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);

                let lo = self.read_from_memory(ptr as u16);
                let hi = self.read_from_memory(ptr.wrapping_add(1) as u16);
                let addr = (hi as u16) << 8 | (lo as u16);
                return addr;
            }
            AddressingMode::INDIRECT_Y => {
                let base = self.read_from_memory(self.program_counter);

                let lo = self.read_from_memory(base as u16);
                let hi = self.read_from_memory(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let addr = deref_base.wrapping_add(self.register_y as u16);
                return addr;
            }
            _ => panic!("Invalid addressing mode"),
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::empty(),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        //loads the program into memory from 0x8000 addr until the len of the program
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);

        //writes on the addr 0xfffc the addr of the beginning of the loaded program
        self.write_to_memory_le(0xfffc, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = StatusFlags::empty();

        //reads the addr of the beginning of the loaded program
        self.program_counter = self.read_from_memory_le(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        loop {
            let opcode_value = self.read_from_memory(self.program_counter);
            self.program_counter += 1;
            let opcode = Opcodes::from_u8(opcode_value).expect("Valid opcode");
            println!("opcode: {:?}", opcode);
            match opcode {
                Opcodes::ADC(_) => {
                    todo!();
                }
                Opcodes::AND(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    let result = self.register_a & value;
                    self.set_register_a(result);
                }
                Opcodes::ASL(addr_mode) => {
                    /*
                        "bit 7 is placed in the carry flag"
                        the logic for that is really interesting,any number with the 7th bit set will overflow after shifting left
                    */
                    self.status.set(
                        StatusFlags::CARRY,
                        self.status.contains(StatusFlags::NEGATIVE),
                    );

                    if addr_mode == AddressingMode::ACCUMULATOR {
                        let value = self.register_a;
                        let result = value << 1;
                        self.set_register_a(result);
                    } else {
                        let addr = self.get_memory_addr(&addr_mode);
                        let value = self.get_value_from_memory(addr_mode);
                        let result = value << 1;
                        self.write_to_memory(addr, result);
                        self.update_negative_flag(result);
                        self.update_zero_flag(result);
                    }
                }
                Opcodes::BCC => {
                    self.branch(!self.status.contains(StatusFlags::CARRY));
                }
                Opcodes::BCS => {
                    self.branch(self.status.contains(StatusFlags::CARRY));
                }
                Opcodes::BEQ => {
                    self.branch(self.status.contains(StatusFlags::ZERO));
                }
                Opcodes::BIT(_) => {
                    todo!()
                }
                Opcodes::BMI => {
                    self.branch(self.status.contains(StatusFlags::NEGATIVE));
                }
                Opcodes::BNE => {
                    self.branch(!self.status.contains(StatusFlags::ZERO));
                }
                Opcodes::BPL => {
                    self.branch(!self.status.contains(StatusFlags::NEGATIVE));
                }
                Opcodes::BVC => {
                    self.branch(!self.status.contains(StatusFlags::OVERFLOW));
                }
                Opcodes::BVS => {
                    self.branch(self.status.contains(StatusFlags::OVERFLOW));
                }
                Opcodes::LDA(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    self.set_register_a(value);
                }
                Opcodes::LDX(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    self.set_register_x(value);
                }
                Opcodes::LDY(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    self.set_register_y(value);
                }
                Opcodes::STA(addr_mode) => {
                    let mode_increment = addr_mode.get_counter_increment();
                    let addr = self.get_memory_addr(addr_mode);
                    self.write_to_memory(addr, self.register_a);
                    self.program_counter += mode_increment;
                }
                Opcodes::STX(addr_mode) => {
                    let mode_increment = addr_mode.get_counter_increment();
                    let addr = self.get_memory_addr(addr_mode);
                    self.write_to_memory(addr, self.register_x);
                    self.program_counter += mode_increment;
                }
                Opcodes::STY(addr_mode) => {
                    let mode_increment = addr_mode.get_counter_increment();
                    let addr = self.get_memory_addr(addr_mode);
                    self.write_to_memory(addr, self.register_y);
                    self.program_counter += mode_increment;
                }
                Opcodes::TAX => {
                    let result = self.register_a;
                    self.register_x = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::TAY => {
                    let result = self.register_a;
                    self.register_y = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::INX => {
                    let result = self.register_x.wrapping_add(1);
                    self.register_x = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::INY => {
                    let result = self.register_y.wrapping_add(1);
                    self.register_y = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::BRK => {
                    break;
                }
                _ => todo!(),
            }
        }
    }

    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.update_negative_flag(value);
        self.update_zero_flag(value);
    }

    fn set_register_x(&mut self, value: u8) {
        self.register_x = value;
        self.update_negative_flag(value);
        self.update_zero_flag(value);
    }

    fn set_register_y(&mut self, value: u8) {
        self.register_y = value;
        self.update_negative_flag(value);
        self.update_zero_flag(value);
    }

    fn get_value_from_memory(&mut self, addr_mode: AddressingMode) -> u8 {
        let mode_increment = addr_mode.get_counter_increment();
        let addr = self.get_memory_addr(addr_mode);
        let param = self.read_from_memory(addr);
        self.program_counter += mode_increment;
        return param;
    }

    fn branch(&mut self, condition: bool) {
        let offset = self.read_from_memory(self.program_counter);
        self.program_counter += 1;
        if condition {
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }

    //This method will see the result of an operation and set the Z flag accordantly
    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            //if the result is 0 the z flag should be set to true
            self.status.insert(StatusFlags::ZERO);
        } else {
            //if the result is not 0 the z flag should be set to false
            self.status.remove(StatusFlags::ZERO);
        }
    }

    //This method will see the result of an operation and set the N flag accordantly
    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            //if the result 7's bit is set than it's a negative value and the flag must be set
            self.status.insert(StatusFlags::NEGATIVE);
        } else {
            //if the result 7's bit not is set than it's a positive value and the flag must be unset
            self.status.remove(StatusFlags::NEGATIVE);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_memory_addr_immediate() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory(0x0000, 0x10);

        assert_eq!(cpu.get_memory_addr(AddressingMode::IMMEDIATE), 0x0000);
    }

    #[test]
    fn test_get_memory_addr_zero_page() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory(0x0000, 0x10);

        assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE), 0x0010);
    }

    #[test]
    fn test_get_memory_addr_absolute() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory_le(0x0000, 0x20);

        assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE), 0x20);
    }

    #[test]
    fn test_get_memory_addr_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory(0x0000, 0x20);
        cpu.register_x = 0x05;

        assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE_X), 0x0025);
    }

    #[test]
    fn test_get_memory_addr_zero_page_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory(0x0000, 0x20);
        cpu.register_y = 0x02;

        assert_eq!(cpu.get_memory_addr(AddressingMode::ZERO_PAGE_Y), 0x0022);
    }

    #[test]
    fn test_get_memory_addr_absolute_x() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory_le(0x0000, 0x30);
        cpu.register_x = 0xFF;

        assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE_X), 0x12f);
    }

    #[test]
    fn test_get_memory_addr_absolute_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.write_to_memory_le(0x0000, 0x40);
        cpu.register_y = 0xFF;

        assert_eq!(cpu.get_memory_addr(AddressingMode::ABSOLUTE_Y), 0x13F);
    }

    #[test]
    fn test_get_memory_addr_indirect_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xFF;
        cpu.program_counter = 0x15;
        cpu.write_to_memory(0x15, 0x60);
        cpu.write_to_memory(0x5F, 0x70);
        cpu.write_to_memory(0x60, 0x71);

        assert_eq!(cpu.get_memory_addr(AddressingMode::INDIRECT_X), 0x7170);
    }

    #[test]
    fn test_get_memory_addr_indirect_y() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x41;
        cpu.register_y = 0x31;
        cpu.write_to_memory(0x41, 0x80);
        cpu.write_to_memory(0x80, 0x90);
        cpu.write_to_memory(0x81, 0xFE);
        assert_eq!(cpu.get_memory_addr(AddressingMode::INDIRECT_Y), 0xFEC1);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.write_to_memory(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }
    #[test]
    fn test_0xa9_lda_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert_eq!(cpu.register_a, 0xff);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa2_ldx_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xa2_ldx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xff, 0x00]);
        assert_eq!(cpu.register_x, 0xff);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa0_ldy_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xa0_ldy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0x00]);
        assert_eq!(cpu.register_y, 0xff);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xaa_tax_copied_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xaa, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, cpu.register_x);
        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xaa, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xaa, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xa8_tay_copied_a_to_y() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xa8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, cpu.register_y);
        assert_eq!(cpu.register_y, 10);
    }

    #[test]
    fn test_0xa8_tay_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xa8, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xa8_tay_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xa8, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xe8_incremented_register_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x5d;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xe8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_x, 0x5e);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xe8_incremented_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xab;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xe8, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xe8_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_0xc8_incremented_register_y() {
        let mut cpu = CPU::new();
        cpu.register_y = 0x5d;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xc8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_y, 0x5e);
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_0xc8_incremented_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xab;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xc8, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_0xc8_overflow() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xff;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xc8, 0xc8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_y, 1)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_reset_should_reset_cpu() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.register_x = 11;
        cpu.register_y = 12;
        cpu.program_counter = 0xaab;
        cpu.load(vec![0x00]);
        cpu.reset();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_y, 0);
        assert_eq!(cpu.program_counter, 0x8000);
    }

    #[test]
    fn test_and_0x29() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.load(vec![0x29, 0b1100_1100, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x25() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.memory[0x00] = 0b1100_1100;
        cpu.load(vec![0x25, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x35() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.register_x = 0x01;
        cpu.memory[0x01] = 0b1100_1100;
        cpu.load(vec![0x35, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x2d() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.memory[0x1234] = 0b1100_1100;
        cpu.load(vec![0x2d, 0x34, 0x12, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x3d() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.register_x = 0x01;
        cpu.memory[0x1235] = 0b1100_1100;
        cpu.load(vec![0x3d, 0x34, 0x12, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x39() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.register_y = 0x01;
        cpu.memory[0x1235] = 0b1100_1100;
        cpu.load(vec![0x39, 0x34, 0x12, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x21() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.register_x = 0x01;
        cpu.memory[0x01] = 0x02;
        cpu.memory[0x02] = 0x03; //tive que modificar
        cpu.memory[0x0302] = 0b1100_1100; //tive que modificar
        cpu.load(vec![0x21, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_and_0x31() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010;
        cpu.register_y = 0x01;
        cpu.memory[0x01] = 0x02;
        cpu.memory[0x201] = 0b1100_1100; //tive que modificar
        cpu.load(vec![0x31, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_1000);
    }

    #[test]
    fn test_asl_0x0a_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b1010_1010; //170
        cpu.status.insert(StatusFlags::NEGATIVE);
        // cpu.load(vec![0xa9, 0xaa, 0x0a, 0x00]); we are basically doing this
        cpu.load(vec![0x0a, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b0101_0100); //84 instead of 340. the carry flag is set
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x0a_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_a = 0b0010_1010; //42

        // cpu.load(vec![0xa9, 0xaa, 0x0a, 0x00]); we are basically doing this
        cpu.load(vec![0x0a, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0b0101_0100); //84. this time the carry flag is not set so the result is "correct"
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x06_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xaa, 0x85, 0x00, 0x06, 0x00, 0x00]); //using load_and_run to avoid having to set the program counter
        assert_eq!(cpu.memory[0x00], 0b0101_0100); //84 instead of 340. the carry flag is set
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x06_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.memory[0x00] = 0b0010_1010; //42
        cpu.load(vec![0x06, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.memory[0x00], 0b0101_0100); //84
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x16_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0x01;
        cpu.memory[0x01] = 0b1010_1010; //170
        cpu.status.insert(StatusFlags::NEGATIVE);
        cpu.load(vec![0x16, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.memory[0x01], 0b0101_0100); //84 instead of 340. the carry flag is set
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x16_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0x01;
        cpu.memory[0x01] = 0b0010_1010; //42
        cpu.load(vec![0x16, 0x00, 0x00]);
        cpu.run();
        assert_eq!(cpu.memory[0x01], 0b0101_0100); //84
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x0e_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.memory[0x0200] = 0b1010_1010; //170
        cpu.status.insert(StatusFlags::NEGATIVE);
        cpu.load(vec![0x0e, 0x00, 0x02]);
        cpu.run();
        assert_eq!(cpu.memory[0x0200], 0b0101_0100); //84 instead of 340. the carry flag is set
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x0e_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.memory[0x0200] = 0b0010_1010; //42
        cpu.load(vec![0x0e, 0x00, 0x02]);
        cpu.run();
        assert_eq!(cpu.memory[0x0200], 0b0101_0100); //84
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x1e_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0x01;
        cpu.memory[0x0201] = 0b1010_1010; //170
        cpu.status.insert(StatusFlags::NEGATIVE);
        cpu.load(vec![0x1e, 0x00, 0x02]);
        cpu.run();
        assert_eq!(cpu.memory[0x0201], 0b0101_0100); //84 instead of 340. the carry flag is set
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_asl_0x1e_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.register_x = 0x01;
        cpu.memory[0x0201] = 0b0010_1010; //42
        cpu.load(vec![0x1e, 0x00, 0x02]);
        cpu.run();
        assert_eq!(cpu.memory[0x0201], 0b0101_0100); //84
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!cpu.status.contains(StatusFlags::ZERO));
    }

    #[test]
    fn test_bcc_0x90_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.insert(StatusFlags::CARRY);
        cpu.load(vec![0x90, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0xaa); //set. instruction was executed
    }

    #[test]
    fn test_bcc_0x90_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.remove(StatusFlags::CARRY);
        cpu.load(vec![0x90, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0x00); //not set. instruction jumped over
    }

    #[test]
    fn test_bcs_0xb0_carry_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.insert(StatusFlags::CARRY);
        cpu.load(vec![0xb0, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0x00); //not set. instruction jumped over
    }

    #[test]
    fn test_bcs_0xb0_carry_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.remove(StatusFlags::CARRY);
        cpu.load(vec![0xb0, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0xaa); //set. instruction was executed
    }

    #[test]
    fn test_beq_0xf0_zero_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.insert(StatusFlags::ZERO);
        cpu.load(vec![0xf0, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0x00); //not set. instruction jumped over
    }

    #[test]
    fn test_beq_0xf0_zero_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.remove(StatusFlags::ZERO);
        cpu.load(vec![0xf0, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0xaa); //set. instruction was executed
    }

    #[test]
    fn test_bmi_0x30_negative_flag_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.insert(StatusFlags::NEGATIVE);
        cpu.load(vec![0x30, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0x00); //not set. instruction jumped over
    }

    #[test]
    fn test_bmi_0x30_negative_flag_not_set() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x8000;
        cpu.status.remove(StatusFlags::NEGATIVE);
        cpu.load(vec![0x30, 0x02, 0xa9, 0xaa, 0x00]);
        cpu.run();
        assert!(cpu.register_a == 0xaa); //set. instruction was executed
    }
}
