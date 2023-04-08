use std::borrow::Borrow;

use super::opcodes::AddressingMode;

#[cfg(test)]
mod test;

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
        return (hi << 8) | lo;
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
