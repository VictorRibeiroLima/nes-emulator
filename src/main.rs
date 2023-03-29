mod cpu;
fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.interpret(vec![0xA9, 10, 0xAA, 0xE8, 0x00]);
    println!("{}", cpu.register_x);
}
