enum Opcodes {
    LDA = 0xA9,
    LDX = 0xA2,
    LDY = 0xA0,
    TAX = 0xAA,
    TAY = 0xA8,
    INX = 0xE8,
    INY = 0xC8,
    BRK = 0x00,
}

impl Opcodes {
    fn from_u8(value: u8) -> Result<Self, ()> {
        match value {
            0xA9 => Ok(Self::LDA),
            0xA2 => Ok(Self::LDX),
            0xA0 => Ok(Self::LDY),
            0xAA => Ok(Self::TAX),
            0xA8 => Ok(Self::TAY),
            0xE8 => Ok(Self::INX),
            0xC8 => Ok(Self::INY),
            0x00 => Ok(Self::BRK),
            _ => Err(()),
        }
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    /* 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction.
    (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
      0b0000_0000
        nvbb_dizc
    */
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

trait Memory {
    fn read_from_memory(&self, addr: u16) -> u8;

    fn write_to_memory(&mut self, addr: u16, data: u8);

    fn read_from_memory_le(&self, addr: u16) -> u16 {
        let lo = self.read_from_memory(addr) as u16;
        let hi = self.read_from_memory(addr + 1) as u16;
        /*hi << 8 moves the value of the first half of this 16bit data to the second half
          0b0000_0000_1111_1111 becomes 0b1111_1111_0000_0000
        */
        (hi << 8) | (lo as u16)
    }

    fn write_to_memory_le(&mut self, addr: u16, data: u16) {
        /*data >> 8 moves the value of the second half of this 16bit data to the first half
        0b1111_1111_0000_0000 becomes 0b0000_0000_1111_1111
        */
        let hi = (data >> 8) as u8;

        //data & 0xff (0b0000_0000_1111_1111) just unset's the second half to make sure this conversion does't break
        let lo = (data & 0xff) as u8;
        self.write_to_memory(addr, lo);
        self.write_to_memory(addr + 1, hi);
    }
}

impl Memory for CPU {
    fn read_from_memory(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
    fn write_to_memory(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
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
        self.status = 0;

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
            let opcode_value = self.next_instruction();
            let opcode = Opcodes::from_u8(opcode_value).expect("Valid opcode");
            match opcode {
                Opcodes::LDA => {
                    let param = self.next_instruction();
                    self.register_a = param;
                    self.update_negative_flag(param);
                    self.update_zero_flag(param);
                }
                Opcodes::LDX => {
                    let param = self.next_instruction();
                    self.register_x = param;
                    self.update_negative_flag(param);
                    self.update_zero_flag(param);
                }
                Opcodes::LDY => {
                    let param = self.next_instruction();
                    self.register_y = param;
                    self.update_negative_flag(param);
                    self.update_zero_flag(param);
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
            }
        }
    }

    //This method will see the result of an operation and set the Z flag accordantly
    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            //if the result is 0 the z flag should be set to true
            self.status = self.status | 0b0000_0010;
        } else {
            //if the result is not 0 the z flag should be set to false
            self.status = self.status & 0b1111_1101;
        }
    }

    //This method will see the result of an operation and set the N flag accordantly
    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            //if the result 7's bit is set than it's a negative value and the flag must be set
            self.status = self.status | 0b1000_0000;
        } else {
            //if the result 7's bit not is set than it's a positive value and the flag must be unset
            self.status = self.status & 0b0111_1111;
        }
    }

    fn next_instruction(&mut self) -> u8 {
        let current_value = self.read_from_memory(self.program_counter);
        self.program_counter += 1;
        return current_value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert_eq!(cpu.register_a, 0xff);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xa2_ldx_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa2_ldx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xff, 0x00]);
        assert_eq!(cpu.register_x, 0xff);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xa0_ldy_immediately_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa0_ldy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0x00]);
        assert_eq!(cpu.register_y, 0xff);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0x80);
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
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xaa, 0x00]);
        cpu.run();
        assert!(cpu.status & 0b1000_0000 == 0x80);
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
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa8_tay_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xa8, 0x00]);
        cpu.run();
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xe8_incremented_register_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x5d;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xe8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_x, 0x5e);
        assert!(cpu.status & 0b0000_0010 == 0b0);
    }

    #[test]
    fn test_0xe8_incremented_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xab;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xe8, 0x00]);
        cpu.run();
        assert!(cpu.status & 0b1000_0000 == 0x80);
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
        assert!(cpu.status & 0b0000_0010 == 0b0);
    }

    #[test]
    fn test_0xc8_incremented_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xab;
        cpu.program_counter = 0x8000;
        cpu.load(vec![0xc8, 0x00]);
        cpu.run();
        assert!(cpu.status & 0b1000_0000 == 0x80);
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
}
