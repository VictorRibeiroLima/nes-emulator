use std::borrow::Borrow;

#[cfg(test)]
mod test;

use bitflags::bitflags;

use super::{
    bus::Bus,
    memory::Memory,
    opcodes::{AddressingMode, Opcodes},
};

bitflags!(
    #[derive(Clone)]
    pub struct StatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const BREAK2 = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
);

const STACK_BASE: u16 = 0x0100;
const STACK_SIZE: u8 = 0x00FF;

pub struct CPU {
    register_a: u8,
    register_x: u8,
    register_y: u8,
    /* 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction.
    (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
      0b0000_0000
        nvbb_dizc
    */
    status: StatusFlags,
    program_counter: u16,
    stack_pointer: u8,
    bus: Bus,
}

impl Memory for CPU {
    fn read_from_memory(&self, addr: u16) -> u8 {
        self.bus.read_from_memory(addr)
    }

    fn write_to_memory(&mut self, addr: u16, data: u8) {
        self.bus.write_to_memory(addr, data);
    }
}

impl CPU {
    pub fn new(bus: Bus) -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: StatusFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            stack_pointer: STACK_SIZE, //0x0100 - 0x01ff is used for the stack
            bus,
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        //loads the program into ram from 0x600 addr until the len of the program
        for i in 0..(program.len() as u16) {
            self.write_to_memory(0x0600 + i, program[i as usize]);
        }

        //writes on the addr 0xfffc the addr of the beginning of the loaded program
        //self.write_to_memory_le(0xfffc, 0x0600);
        self.program_counter = 0x0600;
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = StatusFlags::from_bits_truncate(0b100100);
        self.stack_pointer = STACK_SIZE;

        //reads the addr of the beginning of the loaded program
        self.program_counter = self.read_from_memory_le(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.program_counter = 0x0600;
        self.run();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F: FnMut(&mut CPU)>(&mut self, mut callback: F) {
        loop {
            let opcode_value = self.read_from_memory(self.program_counter);
            self.program_counter += 1;
            let opcode = Opcodes::from_u8(opcode_value).expect("Valid opcode");
            match opcode {
                Opcodes::ADC(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    self.add_to_register_a(value);
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
                Opcodes::BIT(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    let result = self.register_a & value;
                    self.update_negative_flag(value);
                    self.update_overflow_flag(value);

                    self.update_zero_flag(result);
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
                Opcodes::INX => {
                    let value = self.register_x;
                    let result = value.wrapping_add(1);
                    self.set_register_x(result);
                }
                Opcodes::INY => {
                    let result = self.register_y.wrapping_add(1);
                    self.register_y = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::JMP(addr_mode) => {
                    if addr_mode == AddressingMode::ABSOLUTE {
                        let addr = self.get_memory_addr(&addr_mode);
                        self.program_counter = addr;
                    } else {
                        let mem_address = self.read_from_memory_le(self.program_counter);
                        //6502 bug mode with with page boundary:
                        //  if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
                        // the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
                        // i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000

                        let indirect_ref;

                        // any address ending in 0xFF will be affected by the bug
                        if mem_address & 0x00FF == 0x00FF {
                            /*
                                the bug is that the 6502 takes the low byte from the correct address.
                                However, it takes the high byte from 0x**00 (where * is any value) instead of 0x**ff + 1
                            */
                            let lo = self.read_from_memory(mem_address);
                            let hi = self.read_from_memory(mem_address & 0xFF00);
                            indirect_ref = (hi as u16) << 8 | (lo as u16);
                        } else {
                            indirect_ref = self.read_from_memory_le(mem_address);
                        };

                        self.program_counter = indirect_ref;
                    }
                }
                Opcodes::JSR(addr_mode) => {
                    let mode_increment = addr_mode.get_counter_increment();
                    let addr = self.get_memory_addr(&addr_mode);
                    let return_addr = self.program_counter + mode_increment - 1;
                    self.stack_push_le(return_addr);
                    self.program_counter = addr;
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
                Opcodes::LSR(addr_mode) => {
                    let value: u8;
                    if addr_mode == AddressingMode::ACCUMULATOR {
                        value = self.register_a;
                        let result = value >> 1;
                        self.set_register_a(result);
                    } else {
                        let addr = self.get_memory_addr(&addr_mode);
                        value = self.get_value_from_memory(addr_mode);
                        let result = value >> 1;
                        self.write_to_memory(addr, result);
                        self.update_negative_flag(result);
                        self.update_zero_flag(result);
                    }
                    if value & 0x01 == 0x01 {
                        self.status.insert(StatusFlags::CARRY);
                    } else {
                        self.status.remove(StatusFlags::CARRY);
                    }
                }
                Opcodes::NOP => {}
                Opcodes::ORA(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    let result = self.register_a | value;
                    self.set_register_a(result);
                }
                Opcodes::PHA => {
                    let value = self.register_a;
                    self.stack_push(value);
                }
                Opcodes::PHP => {
                    //http://wiki.nesdev.com/w/index.php/CPU_status_flag_behavior
                    let mut flags = self.status.clone();
                    flags.insert(StatusFlags::BREAK);
                    flags.insert(StatusFlags::BREAK2);
                    let value = flags.bits();
                    self.stack_push(value);
                }
                Opcodes::PLA => {
                    let value = self.stack_pop();
                    self.set_register_a(value);
                }
                Opcodes::PLP => {
                    let value = self.stack_pop();
                    self.status = StatusFlags::from_bits_truncate(value);
                    self.status.remove(StatusFlags::BREAK);
                    self.status.insert(StatusFlags::BREAK2);
                }
                Opcodes::ROL(addr_mode) => {
                    let value: u8;
                    // true turns into 0x1, false turns into 0x0
                    let carry_bit = self.status.contains(StatusFlags::CARRY) as u8;
                    if addr_mode == AddressingMode::ACCUMULATOR {
                        value = self.register_a;
                        // a shift left always leaves a 0 on the 0 bit, so we can just OR it with the carry bit
                        let result = (value << 1) | carry_bit;
                        self.set_register_a(result);
                    } else {
                        let addr = self.get_memory_addr(&addr_mode);
                        value = self.get_value_from_memory(addr_mode);
                        let result = (value << 1) | carry_bit;
                        self.write_to_memory(addr, result);
                        self.update_negative_flag(result);
                        self.update_zero_flag(result);
                    }
                    if value & 0x80 == 0x80 {
                        self.status.insert(StatusFlags::CARRY);
                    } else {
                        self.status.remove(StatusFlags::CARRY);
                    }
                }
                Opcodes::ROR(addr_mode) => {
                    let value: u8;
                    // true turns into 0x1, false turns into 0x0,so we shift it to the 7 bit to get the carry bit on the right spot
                    let carry_bit = (self.status.contains(StatusFlags::CARRY) as u8) << 7;
                    if addr_mode == AddressingMode::ACCUMULATOR {
                        value = self.register_a;
                        // a shift right always leaves a 0 on the 7 bit, so we can just OR it with the carry bit
                        let result = (value >> 1) | carry_bit;
                        self.set_register_a(result);
                    } else {
                        let addr = self.get_memory_addr(&addr_mode);
                        value = self.get_value_from_memory(addr_mode);
                        let result = (value >> 1) | carry_bit;
                        self.write_to_memory(addr, result);
                        self.update_negative_flag(result);
                        self.update_zero_flag(result);
                    }
                    if value & 0x01 == 0x01 {
                        self.status.insert(StatusFlags::CARRY);
                    } else {
                        self.status.remove(StatusFlags::CARRY);
                    }
                }
                Opcodes::RTI => {
                    let status = self.stack_pop();
                    let addr = self.stack_pop_le();
                    self.status = StatusFlags::from_bits_truncate(status);
                    self.status.remove(StatusFlags::BREAK);
                    self.status.insert(StatusFlags::BREAK2);
                    self.program_counter = addr;
                }
                Opcodes::RTS => {
                    let addr = self.stack_pop_le();
                    self.program_counter = addr + 1;
                }
                Opcodes::SBC(addr_mode) => {
                    let value = self.get_value_from_memory(addr_mode);
                    let sub_value = (value as i8).wrapping_neg().wrapping_sub(1);
                    self.add_to_register_a(sub_value as u8);
                }
                Opcodes::SEC => {
                    self.status.insert(StatusFlags::CARRY);
                }
                Opcodes::SED => {
                    self.status.insert(StatusFlags::DECIMAL_MODE);
                }
                Opcodes::SEI => {
                    self.status.insert(StatusFlags::INTERRUPT_DISABLE);
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
                Opcodes::TSX => {
                    let result = self.stack_pointer;
                    self.set_register_x(result);
                }
                Opcodes::TXA => {
                    let result = self.register_x;
                    self.register_a = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::TXS => {
                    let result = self.register_x;
                    self.stack_pointer = result;
                }
                Opcodes::TYA => {
                    let result = self.register_y;
                    self.register_a = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::BRK => {
                    break;
                }
            }
            callback(self);
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

    fn get_value_from_memory(&mut self, addr_mode: AddressingMode) -> u8 {
        let mode_increment = addr_mode.get_counter_increment();
        let addr = self.get_memory_addr(addr_mode);
        let param = self.read_from_memory(addr);
        self.program_counter += mode_increment;
        return param;
    }

    fn branch(&mut self, condition: bool) {
        let offset = self.read_from_memory(self.program_counter) as i8;
        self.program_counter = self.program_counter.wrapping_add(1);
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

    //This method will see the result of an operation and set the V flag accordantly
    fn update_overflow_flag(&mut self, result: u8) {
        if result & 0b0100_0000 != 0 {
            //if the result 6's bit is set than it's a negative value and the flag must be set
            self.status.insert(StatusFlags::OVERFLOW);
        } else {
            //if the result 6's bit not is set than it's a positive value and the flag must be unset
            self.status.remove(StatusFlags::OVERFLOW);
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

    fn stack_push(&mut self, value: u8) {
        /*  on an empty stack the pointer is at 0x00FF.
           on this case adding base to the pointer will result in 0x01FF which is the first address of the stack
        */
        let addr = STACK_BASE + (self.stack_pointer) as u16;
        self.write_to_memory(addr, value);

        /*when the pointer reaches 0x00 it will overflow and start at 0xFF.
        this is not necessarily a problem.
        because the pointer starts at 0xff and not on 0x01ff.
        the stack access logic for pushing is to write then decrement the pointer,so we can effectively write at 0x1ff.
        this causes the behavior that when we write at 0x0100, the pointer will overflow and start at 0xFF.

        but nonetheless, we need to check for overflow and warn the user if it happens just for debugging weird behavior.
         */
        let (pointer, overflowed) = self.stack_pointer.overflowing_sub(1);
        self.stack_pointer = pointer;
        if overflowed {
            println!("Warning: Stack overflow on next push");
        }
    }

    fn stack_push_le(&mut self, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0x00FF) as u8;
        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop(&mut self) -> u8 {
        /*when the pointer reaches 0xFF it will underflow and start at 0x00.
        this is not necessarily a problem.
        after writhing at 0x0100, the pointer will overflow and start at 0xFF.
        we need to compensate for this by incrementing the pointer before reading from the stack so we can read from addr 0x0100.

        but nonetheless, we need to check for underflow and warn the user if it happens just for debugging weird behavior.
         */
        let (pointer, overflowed) = self.stack_pointer.overflowing_add(1);
        self.stack_pointer = pointer;
        if overflowed {
            println!("Warning: Stack underflow on next pop");
        }

        let addr = STACK_BASE + (self.stack_pointer) as u16;
        let result = self.read_from_memory(addr);
        return result;
    }

    fn stack_pop_le(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        return (high << 8) | low;
    }

    fn add_to_register_a(&mut self, value: u8) {
        let carry_bit = self.status.contains(StatusFlags::CARRY) as u16;
        let sum = self.register_a as u16 + value as u16 + carry_bit;

        // 0xff = 255.
        // 255 = 0b1111_1111 the maximum amount in 8 bits.
        let carry = sum > 0xff;

        if carry {
            self.status.insert(StatusFlags::CARRY);
        } else {
            self.status.remove(StatusFlags::CARRY);
        }

        let result = sum as u8;

        // weird overflow check
        if (value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status.insert(StatusFlags::OVERFLOW);
        } else {
            self.status.remove(StatusFlags::OVERFLOW)
        }

        self.set_register_a(result);
    }
}
