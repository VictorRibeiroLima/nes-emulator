use std::borrow::Borrow;

use super::opcodes::AddressingMode;

pub trait Memory {
    fn get_memory_addr<T: Borrow<AddressingMode>>(&self, mode: T) -> u16;

    fn read_from_memory(&self, addr: u16) -> u8;

    fn write_to_memory(&mut self, addr: u16, data: u8);

    fn read_from_memory_le(&self, addr: u16) -> u16 {
        let lo = self.read_from_memory(addr) as u16;
        let hi = self.read_from_memory(addr + 1) as u16;
        /*hi << 8 moves the value of the first half of this 16bit data to the second half
          0b0000_0000_1111_1111 becomes 0b1111_1111_0000_0000
        */
        return (hi << 8) | (lo as u16);
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

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use super::Memory;

    struct Cpu {
        pub memory: [u8; 0xFFFF],
    }

    impl Memory for Cpu {
        fn get_memory_addr<T: Borrow<crate::nes::internals::opcodes::AddressingMode>>(
            &self,
            _mode: T,
        ) -> u16 {
            return 0;
        }
        fn read_from_memory(&self, addr: u16) -> u8 {
            return self.memory[addr as usize];
        }
        fn write_to_memory(&mut self, addr: u16, data: u8) {
            self.memory[addr as usize] = data;
        }
    }

    #[test]
    fn test_read_from_memory_le() {
        let mut memory: [u8; 0xFFFF] = [0; 0xFFFF];
        memory[0x1234] = 0x56;
        memory[0x1235] = 0x78;
        let cpu = Cpu { memory };
        assert_eq!(cpu.read_from_memory_le(0x1234), 0x7856);
    }

    #[test]
    fn test_write_to_memory_le() {
        let mut cpu = Cpu {
            memory: [0; 0xFFFF],
        };

        let addr: u16 = 0x1234;
        let data: u16 = 0xABCD;
        cpu.write_to_memory_le(addr, data);

        assert_eq!(cpu.memory[addr as usize], 0xCD);
        assert_eq!(cpu.memory[(addr + 1) as usize], 0xAB);
    }
}
