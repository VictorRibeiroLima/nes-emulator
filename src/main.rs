mod nes;

fn main() {
    let mut cpu = nes::internals::cpu::CPU::new();
    cpu.load_and_run(vec![0xA9, 110, 0xAA, 0xE8, 0x00]);
}
