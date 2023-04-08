use super::Memory;

struct Cpu {
    pub memory: [u8; 0xFFFF],
}

impl Memory for Cpu {
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
