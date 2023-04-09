use super::memory::Memory;

pub const RAM_START: u16 = 0x0000;
pub const RAM_END: u16 = 0x0800;
pub const RAM_SIZE: u16 = RAM_END - RAM_START;
pub const RAM_MIRRORS_END: u16 = 0x1FFF;

pub const PG_ROOM_START: u16 = 0x8000;
pub const PG_ROOM_END: u16 = 0xFFFF;
pub const PG_ROOM_SIZE: u16 = PG_ROOM_END - PG_ROOM_START;

pub const PPU_REGISTERS: u16 = 0x2000;
pub const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

#[cfg(test)]
pub mod test;

pub struct Bus {
    cpu_ram: [u8; RAM_SIZE as usize],
    pg_room: [u8; PG_ROOM_SIZE as usize],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_ram: [0; RAM_SIZE as usize],
            pg_room: [0; PG_ROOM_SIZE as usize],
        }
    }
}

impl Memory for Bus {
    fn read_from_memory(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror = addr & 0x7FF; // 0x7FF = 0b11111111111 we need to reduce the address to 11 bits;
                return self.cpu_ram[mirror as usize];
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            PG_ROOM_START..=PG_ROOM_END => {
                return self.read_from_pg_rom(addr);
            }
            _ => {
                println!("Read Address: {:X} ignored", addr);
                return 0;
            }
        }
    }

    fn write_to_memory(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_MIRRORS_END => {
                let mirror = addr & 0x7FF; // 0x7FF = 0b11111111111 we need to reduce the address to 11 bits;
                self.cpu_ram[mirror as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            PG_ROOM_START..=PG_ROOM_END => {
                panic!("Write to ROM is not supported yet");
            }
            _ => {
                println!("Write Address: {:X} ignored", addr);
            }
        }
    }
}

impl Bus {
    fn read_from_pg_rom(&self, addr: u16) -> u8 {
        let room_addr = addr - PG_ROOM_START; // get the address relative to the start of the room
        self.pg_room[room_addr as usize]
    }
}
