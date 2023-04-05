use std::borrow::Borrow;

#[cfg(test)]
mod test;

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
                Opcodes::CLC => {
                    self.status.remove(StatusFlags::CARRY);
                }
                Opcodes::CLD => {
                    self.status.remove(StatusFlags::DECIMAL_MODE);
                }
                Opcodes::CLI => {
                    self.status.remove(StatusFlags::INTERRUPT_DISABLE);
                }
                Opcodes::CLV => {
                    self.status.remove(StatusFlags::OVERFLOW);
                }
                Opcodes::CMP(addr_mode) => self.compare(addr_mode, self.register_a),
                Opcodes::CPX(addr_mode) => self.compare(addr_mode, self.register_x),
                Opcodes::CPY(addr_mode) => self.compare(addr_mode, self.register_y),
                Opcodes::DEC(addr_mode) => {
                    let addr = self.get_memory_addr(&addr_mode);
                    let value = self.get_value_from_memory(addr_mode);
                    let result = value.wrapping_sub(1);
                    self.write_to_memory(addr, result);
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::DEX => {
                    let value = self.register_x;
                    let result = value.wrapping_sub(1);
                    self.set_register_x(result);
                }
                Opcodes::DEY => {
                    let value = self.register_y;
                    let result = value.wrapping_sub(1);
                    self.set_register_y(result);
                }
                Opcodes::EOR(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    let result = self.register_a ^ value;
                    self.set_register_a(result);
                }
                Opcodes::INC(addr_mode) => {
                    let addr = self.get_memory_addr(&addr_mode);
                    let value = self.get_value_from_memory(addr_mode);
                    let result = value.wrapping_add(1);
                    self.write_to_memory(addr, result);
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
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

    fn compare(&mut self, mode: AddressingMode, value: u8) {
        let mem_value = self.get_value_from_memory(mode);
        let result = value.wrapping_sub(mem_value);
        if value >= mem_value {
            self.status.insert(StatusFlags::CARRY);
        } else {
            self.status.remove(StatusFlags::CARRY);
        }
        self.update_negative_flag(result);
        self.update_zero_flag(result);
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
