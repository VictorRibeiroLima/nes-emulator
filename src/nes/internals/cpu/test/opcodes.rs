use crate::nes::internals::{
    cpu::{StatusFlags, CPU, STACK_SIZE},
    memory::Memory,
};

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.write_to_memory(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn test_0xa9_lda_immediately_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
    assert_eq!(cpu.register_a, 0xff);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xa2_ldx_immediately_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
    assert_eq!(cpu.register_x, 0x05);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xa2_ldx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xa2_ldx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0xff, 0x00]);
    assert_eq!(cpu.register_x, 0xff);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xa0_ldy_immediately_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
    assert_eq!(cpu.register_y, 0x05);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xa0_ldy_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xa0_ldy_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0xff, 0x00]);
    assert_eq!(cpu.register_y, 0xff);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
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
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xaa_tax_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x80;
    cpu.program_counter = 0x8000;
    cpu.load(vec![0xaa, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
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
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xa8_tay_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x80;
    cpu.program_counter = 0x8000;
    cpu.load(vec![0xa8, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_0xe8_incremented_register_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x5d;
    cpu.program_counter = 0x8000;
    cpu.load(vec![0xe8, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_x, 0x5e);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xe8_incremented_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xab;
    cpu.program_counter = 0x8000;
    cpu.load(vec![0xe8, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
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
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xc8_incremented_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0xab;
    cpu.program_counter = 0x8000;
    cpu.load(vec![0xc8, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
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

#[test]
fn test_and_0x29() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.load(vec![0x29, 0b1100_1100, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x25() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.memory[0x00] = 0b1100_1100;
    cpu.load(vec![0x25, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x35() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0b1100_1100;
    cpu.load(vec![0x35, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x2d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.memory[0x1234] = 0b1100_1100;
    cpu.load(vec![0x2d, 0x34, 0x12, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x3d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.register_x = 0x01;
    cpu.memory[0x1235] = 0b1100_1100;
    cpu.load(vec![0x3d, 0x34, 0x12, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x39() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.register_y = 0x01;
    cpu.memory[0x1235] = 0b1100_1100;
    cpu.load(vec![0x39, 0x34, 0x12, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x21() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.memory[0x02] = 0x03;
    cpu.memory[0x0302] = 0b1100_1100;
    cpu.load(vec![0x21, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_and_0x31() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.memory[0x201] = 0b1100_1100;
    cpu.load(vec![0x31, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_1000);
}

#[test]
fn test_asl_0x0a_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b1010_1010; //170
    cpu.status.insert(StatusFlags::NEGATIVE);
    // cpu.load(vec![0xa9, 0xaa, 0x0a, 0x00]); we are basically doing this
    cpu.load(vec![0x0a, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b0101_0100); //84 instead of 340. the carry flag is set
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x0a_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0b0010_1010; //42

    // cpu.load(vec![0xa9, 0xaa, 0x0a, 0x00]); we are basically doing this
    cpu.load(vec![0x0a, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0b0101_0100); //84. this time the carry flag is not set so the result is "correct"
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x06_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xaa, 0x85, 0x00, 0x06, 0x00, 0x00]); //using load_and_run to avoid having to set the program counter
    assert_eq!(cpu.memory[0x00], 0b0101_0100); //84 instead of 340. the carry flag is set
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x06_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x00] = 0b0010_1010; //42
    cpu.load(vec![0x06, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x00], 0b0101_0100); //84
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x16_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0b1010_1010; //170
    cpu.status.insert(StatusFlags::NEGATIVE);
    cpu.load(vec![0x16, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0b0101_0100); //84 instead of 340. the carry flag is set
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x16_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0b0010_1010; //42
    cpu.load(vec![0x16, 0x00, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0b0101_0100); //84
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x0e_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x0200] = 0b1010_1010; //170
    cpu.status.insert(StatusFlags::NEGATIVE);
    cpu.load(vec![0x0e, 0x00, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0200], 0b0101_0100); //84 instead of 340. the carry flag is set
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x0e_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x0200] = 0b0010_1010; //42
    cpu.load(vec![0x0e, 0x00, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0200], 0b0101_0100); //84
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x1e_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0201] = 0b1010_1010; //170
    cpu.status.insert(StatusFlags::NEGATIVE);
    cpu.load(vec![0x1e, 0x00, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0b0101_0100); //84 instead of 340. the carry flag is set
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_0x1e_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0201] = 0b0010_1010; //42
    cpu.load(vec![0x1e, 0x00, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0b0101_0100); //84
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_bcc_0x90_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::CARRY);
    cpu.load(vec![0x90, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bcc_0x90_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::CARRY);
    cpu.load(vec![0x90, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bcs_0xb0_carry_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::CARRY);
    cpu.load(vec![0xb0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bcs_0xb0_carry_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::CARRY);
    cpu.load(vec![0xb0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_beq_0xf0_zero_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::ZERO);
    cpu.load(vec![0xf0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_beq_0xf0_zero_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::ZERO);
    cpu.load(vec![0xf0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bmi_0x30_negative_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::NEGATIVE);
    cpu.load(vec![0x30, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bmi_0x30_negative_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::NEGATIVE);
    cpu.load(vec![0x30, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bne_0xd0_zero_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::ZERO);
    cpu.load(vec![0xd0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bne_0xd0_zero_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::ZERO);
    cpu.load(vec![0xd0, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bpl_0x10_negative_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::NEGATIVE);
    cpu.load(vec![0x10, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bpl_0x10_negative_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::NEGATIVE);
    cpu.load(vec![0x10, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bvc_0x50_overflow_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::OVERFLOW);
    cpu.load(vec![0x50, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_bvc_0x50_overflow_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::OVERFLOW);
    cpu.load(vec![0x50, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bvs_0x70_overflow_flag_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::OVERFLOW);
    cpu.load(vec![0x70, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0x00); //not set. instruction jumped over
}

#[test]
fn test_bvs_0x70_overflow_flag_not_set() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.remove(StatusFlags::OVERFLOW);
    cpu.load(vec![0x70, 0x02, 0xa9, 0xaa, 0x00]);
    cpu.run();
    assert!(cpu.register_a == 0xaa); //set. instruction was executed
}

#[test]
fn test_clc_0x18() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::CARRY);
    cpu.load(vec![0x18, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cld_0xd8() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::DECIMAL_MODE);
    cpu.load(vec![0xd8, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::DECIMAL_MODE));
}

#[test]
fn test_cli_0x58() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::INTERRUPT_DISABLE);
    cpu.load(vec![0x58, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_clv_0xb8() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.status.insert(StatusFlags::OVERFLOW);
    cpu.load(vec![0xb8, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_cmp_0xc9_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.load(vec![0xc9, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc9_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.load(vec![0xc9, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc9_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.load(vec![0xc9, 0x02, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc5_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x00] = 0x01;
    cpu.load(vec![0xc5, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc5_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.memory[0x00] = 0x01;
    cpu.load(vec![0xc5, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc5_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x00] = 0x02;
    cpu.load(vec![0xc5, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd5_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xd5, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd5_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xd5, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd5_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.load(vec![0xd5, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xcd_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x0000] = 0x01;
    cpu.load(vec![0xcd, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xcd_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.memory[0x0000] = 0x01;
    cpu.load(vec![0xcd, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xcd_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x0000] = 0x02;
    cpu.load(vec![0xcd, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xdd_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x0001] = 0x01;
    cpu.load(vec![0xdd, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xdd_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.register_x = 0x01;
    cpu.memory[0x0001] = 0x01;
    cpu.load(vec![0xdd, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xdd_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x0001] = 0x02;
    cpu.load(vec![0xdd, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd9_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x0001] = 0x01;
    cpu.load(vec![0xd9, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd9_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.register_y = 0x01;
    cpu.memory[0x0001] = 0x01;
    cpu.load(vec![0xd9, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd9_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x0001] = 0x02;
    cpu.load(vec![0xd9, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc1_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xc1, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc1_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.register_x = 0x01;
    cpu.memory[0x0001] = 0x01;
    cpu.load(vec![0xc1, 0x00, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xc1_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x0001] = 0x02;
    cpu.memory[0x0002] = 0x03;
    cpu.memory[0x0302] = 0x02;
    cpu.load(vec![0xc1, 0x00, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd1_value_equal_to_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.memory[0x02] = 0x03;
    cpu.memory[0x0303] = 0x01;
    cpu.load(vec![0xd1, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd1_value_less_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.register_y = 0x01;
    cpu.memory[0x0001] = 0x02;
    cpu.memory[0x0002] = 0x03;
    cpu.memory[0x0303] = 0x01;
    cpu.load(vec![0xd1, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_0xd1_value_greater_than_accumulator() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x0001] = 0x02;
    cpu.memory[0x0002] = 0x03;
    cpu.memory[0x0303] = 0x02;
    cpu.load(vec![0xd1, 0x01, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe0_value_equal_to_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.load(vec![0xe0, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe0_value_less_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.load(vec![0xe0, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe0_value_greater_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.load(vec![0xe0, 0x02, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe4_value_equal_to_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xe4, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe4_value_less_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xe4, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xe4_value_greater_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.load(vec![0xe4, 0x01, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xec_value_equal_to_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xec, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xec_value_less_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x02;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xec, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpx_0xec_value_greater_than_x() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xec, 0x01, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc0_value_equal_to_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.load(vec![0xc0, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc0_value_less_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x02;
    cpu.load(vec![0xc0, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc0_value_greater_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.load(vec![0xc0, 0x02, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc4_value_equal_to_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xc4, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc4_value_less_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x02;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xc4, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xc4_value_greater_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.load(vec![0xc4, 0x01, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xcc_value_equal_to_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xcc, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xcc_value_less_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x02;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xcc, 0x01, 0x00]);
    cpu.run();
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_0xcc_value_greater_than_y() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.memory[0x01] = 0x02;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0xcc, 0x01, 0x00]);
    cpu.run();
    assert!(!cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dec_0xc6() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xc6, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dec_0xd6() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.load(vec![0xd6, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dec_0xce() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xce, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dec_0xde() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.load(vec![0xde, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dex_0xca() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.load(vec![0xca]);
    cpu.run();
    assert_eq!(cpu.register_x, 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dey_0x88() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_y = 0x01;
    cpu.load(vec![0x88]);
    cpu.run();
    assert_eq!(cpu.register_y, 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x49() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.load(vec![0x49, 0x01]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x45() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0x45, 0x01]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x55() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.load(vec![0x55, 0x01]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x4d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.memory[0x0201] = 0x01;
    cpu.load(vec![0x4d, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x5d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x01;
    cpu.load(vec![0x5d, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x59() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.register_y = 0x01;
    cpu.memory[0x0202] = 0x01;
    cpu.load(vec![0x59, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x41() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x0201] = 0x01;
    cpu.load(vec![0x41, 0x01]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_0x51() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.register_x = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.memory[0x01] = 0x01;
    cpu.memory[0x0102] = 0x01;
    cpu.load(vec![0x51, 0x01]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_0xe6() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0xe6, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_0xf6() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.load(vec![0xf6, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_0xee() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x0201] = 0x01;
    cpu.load(vec![0xee, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_0xfe() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x01;
    cpu.load(vec![0xfe, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0202], 0x02);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_lsr_0x4a_set_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x03;
    cpu.load(vec![0x4a]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x4a_set_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.load(vec![0x4a]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x4a_unset_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(StatusFlags::CARRY);
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x02;
    cpu.load(vec![0x4a]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x46_set_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x03;
    cpu.load(vec![0x46, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x46_set_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x01;
    cpu.load(vec![0x46, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x46_unset_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(StatusFlags::CARRY);
    cpu.program_counter = 0x8000;
    cpu.memory[0x01] = 0x02;
    cpu.load(vec![0x46, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x01], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x56_set_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x03;
    cpu.load(vec![0x56, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x56_set_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x01;
    cpu.load(vec![0x56, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x56_unset_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(StatusFlags::CARRY);
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x02;
    cpu.load(vec![0x56, 0x01]);
    cpu.run();
    assert_eq!(cpu.memory[0x02], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x4e_set_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x0201] = 0x03;
    cpu.load(vec![0x4e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x4e_set_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.memory[0x0201] = 0x01;
    cpu.load(vec![0x4e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x4e_unset_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(StatusFlags::CARRY);
    cpu.program_counter = 0x8000;
    cpu.memory[0x0201] = 0x02;
    cpu.load(vec![0x4e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0201], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x5e_set_carry() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x03;
    cpu.load(vec![0x5e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0202], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x5e_set_zero() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x01;
    cpu.load(vec![0x5e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0202], 0x00);
    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_0x5e_unset_carry() {
    let mut cpu = CPU::new();
    cpu.status.insert(StatusFlags::CARRY);
    cpu.program_counter = 0x8000;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x02;
    cpu.load(vec![0x5e, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.memory[0x0202], 0x01);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_nop_0xea() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x02;
    cpu.register_y = 0x03;
    cpu.load(vec![0xea, 0xea, 0xea]);
    cpu.run();
    assert_eq!(cpu.program_counter, 0x8004);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_x, 0x02);
    assert_eq!(cpu.register_y, 0x03);
}

#[test]
fn test_ora_0x09() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.load(vec![0x09, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x05() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x02] = 0x02;
    cpu.load(vec![0x05, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x15() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x03] = 0x02;
    cpu.load(vec![0x15, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x0d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.memory[0x0201] = 0x02;
    cpu.load(vec![0x0d, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x1d() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x0202] = 0x02;
    cpu.load(vec![0x1d, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x19() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x0202] = 0x02;
    cpu.load(vec![0x19, 0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x01() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_x = 0x01;
    cpu.memory[0x02] = 0x02;
    cpu.memory[0x03] = 0x03;
    cpu.memory[0x04] = 0x02;
    cpu.memory[0x0203] = 0x02;
    cpu.load(vec![0x01, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_0x11() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.register_a = 0x01;
    cpu.register_y = 0x01;
    cpu.memory[0x02] = 0x02;
    cpu.memory[0x03] = 0x03;
    cpu.memory[0x0303] = 0x02;
    cpu.load(vec![0x11, 0x02]);
    cpu.run();
    assert_eq!(cpu.register_a, 0x03);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_pha_0x48_empty_stack() {
    let mut cpu = CPU::new();
    assert!(cpu.stack_pointer == STACK_SIZE);
    cpu.load_and_run(vec![
        0xA9, 0xe0, 0x48, 0xA0, 0xbb, 0x98, 0x48, 0xA2, 0x01, 0x8A, 0x48,
    ]);
    /*
       LDA #$e0
       PHA
       LDY #$bb
       TYA
       PHA
       LDX #$01
       TXA
       PHA
    */
    assert!(cpu.stack_pointer == 0xfc);
    assert!(cpu.memory[0x01ff] == 0xe0);
    assert!(cpu.memory[0x01fe] == 0xbb);
    assert!(cpu.memory[0x01fd] == 0x01);
}

#[test]
fn test_pha_0x48_full_stack() {
    let mut cpu = CPU::new();
    cpu.program_counter = 0x8000;
    cpu.stack_pointer = 0x00;
    cpu.load(vec![0xA9, 0xe0, 0x48]);
    /*
        LDA #$e0
        PHA
    */
    cpu.run();
    assert!(cpu.stack_pointer == 0xff);
    assert!(cpu.memory[0x0100] == 0xe0);
}
