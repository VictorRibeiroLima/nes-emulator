mod cpu;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.load_and_run(vec![0xA9, 110, 0xAA, 0xE8, 0x00]);
    println!("{}", cpu.register_x);
    println!("{}", cpu.register_a);
}
