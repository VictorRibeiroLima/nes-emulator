enum Opcodes {
    LDA = 0xA9,
    TAX = 0xAA,
    INX = 0xE8,
    BRK = 0x00,
}

impl Opcodes {
    fn from_u8(value: u8) -> Result<Self, ()> {
        match value {
            0xA9 => Ok(Self::LDA),
            0xAA => Ok(Self::TAX),
            0xE8 => Ok(Self::INX),
            0x00 => Ok(Self::BRK),
            _ => Err(()),
        }
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    /* 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction.
    (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)
      0b0000_0000
        nvbb_dizc
    */
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opcode_value = program[self.get_instruction()];
            let opcode = Opcodes::from_u8(opcode_value).expect("Valid opcode");
            match opcode {
                Opcodes::LDA => {
                    let param = program[self.get_instruction()];
                    self.register_a = param;
                    self.update_negative_flag(param);
                    self.update_zero_flag(param);
                }
                Opcodes::TAX => {
                    let result = self.register_a;
                    self.register_x = result;
                    self.update_negative_flag(result);
                    self.update_zero_flag(result);
                }
                Opcodes::INX => {
                    let result = self.register_x.wrapping_add(1);
                    self.register_x = result;
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

    fn get_instruction(&mut self) -> usize {
        let current_value = self.program_counter as usize;
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xff, 0x00]);
        assert_eq!(cpu.register_a, 0xff);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xaa_tax_copied_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_a, cpu.register_x);
        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.interpret(vec![0xaa, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x80;
        cpu.interpret(vec![0xaa, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xe8_incremented_register_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x5d;
        cpu.interpret(vec![0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x5e);
        assert!(cpu.status & 0b0000_0010 == 0b0);
    }

    #[test]
    fn test_0xe8_incremented_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xab;
        cpu.interpret(vec![0xe8, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0x80);
    }

    #[test]
    fn test_0xe8_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }
}
