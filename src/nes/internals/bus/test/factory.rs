use std::collections::HashMap;

use crate::nes::internals::bus::Bus;

macro_rules! build_bus {
    ($($prop:ident[$addr:expr] = $value:expr ),*) => {
        {
            let mut cpu_ram: ::std::collections::HashMap<u16,u8> = ::std::collections::HashMap::new();
            let mut pg_room: ::std::collections::HashMap<u16,u8> = ::std::collections::HashMap::new();
            $(
                crate::nes::internals::bus::test::factory::check_prop!($prop);
                let prop = stringify!($prop);
                if prop == "cpu_ram" {
                    cpu_ram.insert($addr, $value);
                } else if prop == "pg_room" {
                    pg_room.insert($addr, $value);
                }
            )*
            crate::nes::internals::bus::test::factory::build_bus_with_values(cpu_ram, pg_room)
        }
    };
}

macro_rules! check_prop {
    (cpu_ram) => {};
    (pg_room) => {};
    ($prop:ident) => {
        compile_error!("Invalid property name");
    };
}

pub fn build_bus_with_values(cpu_ram: HashMap<u16, u8>, pg_room: HashMap<u16, u8>) -> Bus {
    let mut bus = Bus::new();

    for (addr, data) in cpu_ram.iter() {
        bus.cpu_ram[*addr as usize] = *data;
    }

    for (addr, data) in pg_room.iter() {
        bus.pg_room[*addr as usize] = *data;
    }

    return bus;
}

pub fn read_bus_cpu_ram(bus: &Bus, addr: u16) -> u8 {
    return bus.cpu_ram[addr as usize];
}

pub(crate) use build_bus;
pub(crate) use check_prop;
