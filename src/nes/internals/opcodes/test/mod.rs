macro_rules! test_opcodes {
    ($opcode:ident,$value:expr,$mode:ident,$test_name:ident) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                Opcodes::from_u8($value),
                Ok(Opcodes::$opcode(AddressingMode::$mode))
            );
        }
    };
    ($opcode:ident,$value:expr,$test_name:ident) => {
        #[test]
        fn $test_name() {
            assert_eq!(Opcodes::from_u8($value), Ok(Opcodes::$opcode));
        }
    };
}

use super::*;

test_opcodes!(ADC, 0x69, IMMEDIATE, test_from_u8_adc_immediate);
test_opcodes!(ADC, 0x65, ZERO_PAGE, test_from_u8_adc_zero_page);
test_opcodes!(ADC, 0x75, ZERO_PAGE_X, test_from_u8_adc_zero_page_x);
test_opcodes!(ADC, 0x6D, ABSOLUTE, test_from_u8_adc_absolute);
test_opcodes!(ADC, 0x7D, ABSOLUTE_X, test_from_u8_adc_absolute_x);
test_opcodes!(ADC, 0x79, ABSOLUTE_Y, test_from_u8_adc_absolute_y);
test_opcodes!(ADC, 0x61, INDIRECT_X, test_from_u8_adc_indirect_x);
test_opcodes!(ADC, 0x71, INDIRECT_Y, test_from_u8_adc_indirect_y);

test_opcodes!(AND, 0x29, IMMEDIATE, test_from_u8_and_immediate);
test_opcodes!(AND, 0x25, ZERO_PAGE, test_from_u8_and_zero_page);
test_opcodes!(AND, 0x35, ZERO_PAGE_X, test_from_u8_and_zero_page_x);
test_opcodes!(AND, 0x2D, ABSOLUTE, test_from_u8_and_absolute);
test_opcodes!(AND, 0x3D, ABSOLUTE_X, test_from_u8_and_absolute_x);
test_opcodes!(AND, 0x39, ABSOLUTE_Y, test_from_u8_and_absolute_y);
test_opcodes!(AND, 0x21, INDIRECT_X, test_from_u8_and_indirect_x);
test_opcodes!(AND, 0x31, INDIRECT_Y, test_from_u8_and_indirect_y);

test_opcodes!(ASL, 0x0A, ACCUMULATOR, test_from_u8_asl_accumulator);
test_opcodes!(ASL, 0x06, ZERO_PAGE, test_from_u8_asl_zero_page);
test_opcodes!(ASL, 0x16, ZERO_PAGE_X, test_from_u8_asl_zero_page_x);
test_opcodes!(ASL, 0x0E, ABSOLUTE, test_from_u8_asl_absolute);
test_opcodes!(ASL, 0x1E, ABSOLUTE_X, test_from_u8_asl_absolute_x);

test_opcodes!(BCC, 0x90, test_from_u8_bcc);

test_opcodes!(BCS, 0xB0, test_from_u8_bcs);

test_opcodes!(BEQ, 0xF0, test_from_u8_beq);

test_opcodes!(BIT, 0x24, ZERO_PAGE, test_from_u8_bit_zero_page);

test_opcodes!(BIT, 0x2C, ABSOLUTE, test_from_u8_bit_absolute);

test_opcodes!(BMI, 0x30, test_from_u8_bmi);

test_opcodes!(BNE, 0xD0, test_from_u8_bne);

test_opcodes!(BPL, 0x10, test_from_u8_bpl);

test_opcodes!(BRK, 0x00, test_from_u8_brk);

test_opcodes!(BVC, 0x50, test_from_u8_bvc);

test_opcodes!(BVS, 0x70, test_from_u8_bvs);

test_opcodes!(CLC, 0x18, test_from_u8_clc);

test_opcodes!(CLD, 0xD8, test_from_u8_cld);

test_opcodes!(CLI, 0x58, test_from_u8_cli);

test_opcodes!(CLV, 0xB8, test_from_u8_clv);

test_opcodes!(CMP, 0xC9, IMMEDIATE, test_from_u8_cmp_immediate);
test_opcodes!(CMP, 0xC5, ZERO_PAGE, test_from_u8_cmp_zero_page);
test_opcodes!(CMP, 0xD5, ZERO_PAGE_X, test_from_u8_cmp_zero_page_x);
test_opcodes!(CMP, 0xCD, ABSOLUTE, test_from_u8_cmp_absolute);
test_opcodes!(CMP, 0xDD, ABSOLUTE_X, test_from_u8_cmp_absolute_x);
test_opcodes!(CMP, 0xD9, ABSOLUTE_Y, test_from_u8_cmp_absolute_y);
test_opcodes!(CMP, 0xC1, INDIRECT_X, test_from_u8_cmp_indirect_x);
test_opcodes!(CMP, 0xD1, INDIRECT_Y, test_from_u8_cmp_indirect_y);

test_opcodes!(CPX, 0xE0, IMMEDIATE, test_from_u8_cpx_immediate);
test_opcodes!(CPX, 0xE4, ZERO_PAGE, test_from_u8_cpx_zero_page);
test_opcodes!(CPX, 0xEC, ABSOLUTE, test_from_u8_cpx_absolute);

test_opcodes!(CPY, 0xC0, IMMEDIATE, test_from_u8_cpy_immediate);
test_opcodes!(CPY, 0xC4, ZERO_PAGE, test_from_u8_cpy_zero_page);
test_opcodes!(CPY, 0xCC, ABSOLUTE, test_from_u8_cpy_absolute);

test_opcodes!(DEC, 0xC6, ZERO_PAGE, test_from_u8_dec_zero_page);
test_opcodes!(DEC, 0xD6, ZERO_PAGE_X, test_from_u8_dec_zero_page_x);
test_opcodes!(DEC, 0xCE, ABSOLUTE, test_from_u8_dec_absolute);
test_opcodes!(DEC, 0xDE, ABSOLUTE_X, test_from_u8_dec_absolute_x);

test_opcodes!(DEX, 0xCA, test_from_u8_dex);

test_opcodes!(DEY, 0x88, test_from_u8_dey);

test_opcodes!(EOR, 0x49, IMMEDIATE, test_from_u8_eor_immediate);
test_opcodes!(EOR, 0x45, ZERO_PAGE, test_from_u8_eor_zero_page);
test_opcodes!(EOR, 0x55, ZERO_PAGE_X, test_from_u8_eor_zero_page_x);
test_opcodes!(EOR, 0x4D, ABSOLUTE, test_from_u8_eor_absolute);
test_opcodes!(EOR, 0x5D, ABSOLUTE_X, test_from_u8_eor_absolute_x);
test_opcodes!(EOR, 0x59, ABSOLUTE_Y, test_from_u8_eor_absolute_y);
test_opcodes!(EOR, 0x41, INDIRECT_X, test_from_u8_eor_indirect_x);
test_opcodes!(EOR, 0x51, INDIRECT_Y, test_from_u8_eor_indirect_y);

test_opcodes!(INC, 0xE6, ZERO_PAGE, test_from_u8_inc_zero_page);
test_opcodes!(INC, 0xF6, ZERO_PAGE_X, test_from_u8_inc_zero_page_x);
test_opcodes!(INC, 0xEE, ABSOLUTE, test_from_u8_inc_absolute);
test_opcodes!(INC, 0xFE, ABSOLUTE_X, test_from_u8_inc_absolute_x);

test_opcodes!(INX, 0xE8, test_from_u8_inx);

test_opcodes!(INY, 0xC8, test_from_u8_iny);

test_opcodes!(JMP, 0x4C, ABSOLUTE, test_from_u8_jmp_absolute);
test_opcodes!(JMP, 0x6C, INDIRECT, test_from_u8_jmp_indirect);

test_opcodes!(JSR, 0x20, ABSOLUTE, test_from_u8_jsr_absolute);

test_opcodes!(LDA, 0xA9, IMMEDIATE, test_from_u8_lda_immediate);
test_opcodes!(LDA, 0xA5, ZERO_PAGE, test_from_u8_lda_zero_page);
test_opcodes!(LDA, 0xB5, ZERO_PAGE_X, test_from_u8_lda_zero_page_x);
test_opcodes!(LDA, 0xAD, ABSOLUTE, test_from_u8_lda_absolute);
test_opcodes!(LDA, 0xBD, ABSOLUTE_X, test_from_u8_lda_absolute_x);
test_opcodes!(LDA, 0xB9, ABSOLUTE_Y, test_from_u8_lda_absolute_y);
test_opcodes!(LDA, 0xA1, INDIRECT_X, test_from_u8_lda_indirect_x);
test_opcodes!(LDA, 0xB1, INDIRECT_Y, test_from_u8_lda_indirect_y);

test_opcodes!(LDX, 0xA2, IMMEDIATE, test_from_u8_ldx_immediate);
test_opcodes!(LDX, 0xA6, ZERO_PAGE, test_from_u8_ldx_zero_page);
test_opcodes!(LDX, 0xB6, ZERO_PAGE_Y, test_from_u8_ldx_zero_page_y);
test_opcodes!(LDX, 0xAE, ABSOLUTE, test_from_u8_ldx_absolute);
test_opcodes!(LDX, 0xBE, ABSOLUTE_Y, test_from_u8_ldx_absolute_y);

test_opcodes!(LDY, 0xA0, IMMEDIATE, test_from_u8_ldy_immediate);
test_opcodes!(LDY, 0xA4, ZERO_PAGE, test_from_u8_ldy_zero_page);
test_opcodes!(LDY, 0xB4, ZERO_PAGE_X, test_from_u8_ldy_zero_page_x);
test_opcodes!(LDY, 0xAC, ABSOLUTE, test_from_u8_ldy_absolute);
test_opcodes!(LDY, 0xBC, ABSOLUTE_X, test_from_u8_ldy_absolute_x);

test_opcodes!(LSR, 0x4A, ACCUMULATOR, test_from_u8_lsr_accumulator);
test_opcodes!(LSR, 0x46, ZERO_PAGE, test_from_u8_lsr_zero_page);
test_opcodes!(LSR, 0x56, ZERO_PAGE_X, test_from_u8_lsr_zero_page_x);
test_opcodes!(LSR, 0x4E, ABSOLUTE, test_from_u8_lsr_absolute);
test_opcodes!(LSR, 0x5E, ABSOLUTE_X, test_from_u8_lsr_absolute_x);

test_opcodes!(NOP, 0xEA, test_from_u8_nop);

test_opcodes!(ORA, 0x09, IMMEDIATE, test_from_u8_ora_immediate);
test_opcodes!(ORA, 0x05, ZERO_PAGE, test_from_u8_ora_zero_page);
test_opcodes!(ORA, 0x15, ZERO_PAGE_X, test_from_u8_ora_zero_page_x);
test_opcodes!(ORA, 0x0D, ABSOLUTE, test_from_u8_ora_absolute);
test_opcodes!(ORA, 0x1D, ABSOLUTE_X, test_from_u8_ora_absolute_x);
test_opcodes!(ORA, 0x19, ABSOLUTE_Y, test_from_u8_ora_absolute_y);
test_opcodes!(ORA, 0x01, INDIRECT_X, test_from_u8_ora_indirect_x);
test_opcodes!(ORA, 0x11, INDIRECT_Y, test_from_u8_ora_indirect_y);

test_opcodes!(PHA, 0x48, test_from_u8_pha);

test_opcodes!(PHP, 0x08, test_from_u8_php);

test_opcodes!(PLA, 0x68, test_from_u8_pla);

test_opcodes!(PLP, 0x28, test_from_u8_plp);

test_opcodes!(ROL, 0x2A, ACCUMULATOR, test_from_u8_rol_accumulator);
test_opcodes!(ROL, 0x26, ZERO_PAGE, test_from_u8_rol_zero_page);
test_opcodes!(ROL, 0x36, ZERO_PAGE_X, test_from_u8_rol_zero_page_x);
test_opcodes!(ROL, 0x2E, ABSOLUTE, test_from_u8_rol_absolute);
test_opcodes!(ROL, 0x3E, ABSOLUTE_X, test_from_u8_rol_absolute_x);

test_opcodes!(ROR, 0x6A, ACCUMULATOR, test_from_u8_ror_accumulator);
test_opcodes!(ROR, 0x66, ZERO_PAGE, test_from_u8_ror_zero_page);
test_opcodes!(ROR, 0x76, ZERO_PAGE_X, test_from_u8_ror_zero_page_x);
test_opcodes!(ROR, 0x6E, ABSOLUTE, test_from_u8_ror_absolute);
test_opcodes!(ROR, 0x7E, ABSOLUTE_X, test_from_u8_ror_absolute_x);

test_opcodes!(RTI, 0x40, test_from_u8_rti);

test_opcodes!(RTS, 0x60, test_from_u8_rts);

test_opcodes!(SBC, 0xE9, IMMEDIATE, test_from_u8_sbc_immediate);
test_opcodes!(SBC, 0xE5, ZERO_PAGE, test_from_u8_sbc_zero_page);
test_opcodes!(SBC, 0xF5, ZERO_PAGE_X, test_from_u8_sbc_zero_page_x);
test_opcodes!(SBC, 0xED, ABSOLUTE, test_from_u8_sbc_absolute);
test_opcodes!(SBC, 0xFD, ABSOLUTE_X, test_from_u8_sbc_absolute_x);
test_opcodes!(SBC, 0xF9, ABSOLUTE_Y, test_from_u8_sbc_absolute_y);
test_opcodes!(SBC, 0xE1, INDIRECT_X, test_from_u8_sbc_indirect_x);
test_opcodes!(SBC, 0xF1, INDIRECT_Y, test_from_u8_sbc_indirect_y);

test_opcodes!(SEC, 0x38, test_from_u8_sec);

test_opcodes!(SED, 0xF8, test_from_u8_sed);

test_opcodes!(SEI, 0x78, test_from_u8_sei);

test_opcodes!(STA, 0x85, ZERO_PAGE, test_from_u8_sta_zero_page);
test_opcodes!(STA, 0x95, ZERO_PAGE_X, test_from_u8_sta_zero_page_x);
test_opcodes!(STA, 0x8D, ABSOLUTE, test_from_u8_sta_absolute);
test_opcodes!(STA, 0x9D, ABSOLUTE_X, test_from_u8_sta_absolute_x);
test_opcodes!(STA, 0x99, ABSOLUTE_Y, test_from_u8_sta_absolute_y);
test_opcodes!(STA, 0x81, INDIRECT_X, test_from_u8_sta_indirect_x);
test_opcodes!(STA, 0x91, INDIRECT_Y, test_from_u8_sta_indirect_y);

test_opcodes!(STX, 0x86, ZERO_PAGE, test_from_u8_stx_zero_page);
test_opcodes!(STX, 0x96, ZERO_PAGE_Y, test_from_u8_stx_zero_page_y);
test_opcodes!(STX, 0x8E, ABSOLUTE, test_from_u8_stx_absolute);

test_opcodes!(STY, 0x84, ZERO_PAGE, test_from_u8_sty_zero_page);
test_opcodes!(STY, 0x94, ZERO_PAGE_X, test_from_u8_sty_zero_page_x);
test_opcodes!(STY, 0x8C, ABSOLUTE, test_from_u8_sty_absolute);

test_opcodes!(TAX, 0xAA, test_from_u8_tax);

test_opcodes!(TAY, 0xA8, test_from_u8_tay);

test_opcodes!(TSX, 0xBA, test_from_u8_tsx);

test_opcodes!(TXA, 0x8A, test_from_u8_txa);

test_opcodes!(TXS, 0x9A, test_from_u8_txs);

test_opcodes!(TYA, 0x98, test_from_u8_tya);

#[test]
fn test_invalid_opcode() {
    assert_eq!(Opcodes::from_u8(0xFF), Err(()));
}
