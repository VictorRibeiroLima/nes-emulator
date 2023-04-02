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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    ACCUMULATOR,
    IMMEDIATE,
    ZERO_PAGE,
    ZERO_PAGE_X,
    ZERO_PAGE_Y,
    ABSOLUTE,
    ABSOLUTE_X,
    ABSOLUTE_Y,
    INDIRECT,
    INDIRECT_X,
    INDIRECT_Y,
}

impl AddressingMode {
    pub fn get_counter_increment(&self) -> u16 {
        match *self {
            Self::IMMEDIATE => 1,
            Self::ZERO_PAGE => 1,
            Self::ZERO_PAGE_X => 1,
            Self::ZERO_PAGE_Y => 1,
            Self::ABSOLUTE => 2,
            Self::ABSOLUTE_X => 2,
            Self::ABSOLUTE_Y => 2,
            Self::INDIRECT_X => 1,
            Self::INDIRECT_Y => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcodes {
    ADC(AddressingMode),
    AND(AddressingMode),
    ASL(AddressingMode),
    BCC,
    BCS,
    BEQ,
    BIT(AddressingMode),
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    DEX,
    DEY,
    EOR(AddressingMode),
    INC(AddressingMode),
    INX,
    INY,
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP,
    ORA(AddressingMode),
    PHA,
    PHP,
    PLA,
    PLP,
    ROL(AddressingMode),
    ROR(AddressingMode),
    RTI,
    RTS,
    SBC(AddressingMode),
    SEC,
    SED,
    SEI,
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl Opcodes {
    pub fn from_u8(value: u8) -> Result<Self, ()> {
        match value {
            0x69 => Ok(Self::ADC(AddressingMode::IMMEDIATE)),
            0x65 => Ok(Self::ADC(AddressingMode::ZERO_PAGE)),
            0x75 => Ok(Self::ADC(AddressingMode::ZERO_PAGE_X)),
            0x6D => Ok(Self::ADC(AddressingMode::ABSOLUTE)),
            0x7D => Ok(Self::ADC(AddressingMode::ABSOLUTE_X)),
            0x79 => Ok(Self::ADC(AddressingMode::ABSOLUTE_Y)),
            0x61 => Ok(Self::ADC(AddressingMode::INDIRECT_X)),
            0x71 => Ok(Self::ADC(AddressingMode::INDIRECT_Y)),

            0x29 => Ok(Self::AND(AddressingMode::IMMEDIATE)),
            0x25 => Ok(Self::AND(AddressingMode::ZERO_PAGE)),
            0x35 => Ok(Self::AND(AddressingMode::ZERO_PAGE_X)),
            0x2D => Ok(Self::AND(AddressingMode::ABSOLUTE)),
            0x3D => Ok(Self::AND(AddressingMode::ABSOLUTE_X)),
            0x39 => Ok(Self::AND(AddressingMode::ABSOLUTE_Y)),
            0x21 => Ok(Self::AND(AddressingMode::INDIRECT_X)),
            0x31 => Ok(Self::AND(AddressingMode::INDIRECT_Y)),

            0x0A => Ok(Self::ASL(AddressingMode::ACCUMULATOR)),
            0x06 => Ok(Self::ASL(AddressingMode::ZERO_PAGE)),
            0x16 => Ok(Self::ASL(AddressingMode::ZERO_PAGE_X)),
            0x0E => Ok(Self::ASL(AddressingMode::ABSOLUTE)),
            0x1E => Ok(Self::ASL(AddressingMode::ABSOLUTE_X)),

            0x90 => Ok(Self::BCC),

            0xB0 => Ok(Self::BCS),

            0xF0 => Ok(Self::BEQ),

            0x24 => Ok(Self::BIT(AddressingMode::ZERO_PAGE)),
            0x2C => Ok(Self::BIT(AddressingMode::ABSOLUTE)),

            0x30 => Ok(Self::BMI),

            0xD0 => Ok(Self::BNE),

            0x10 => Ok(Self::BPL),

            0x00 => Ok(Self::BRK),

            0x50 => Ok(Self::BVC),

            0x70 => Ok(Self::BVS),

            0x18 => Ok(Self::CLC),

            0xD8 => Ok(Self::CLD),

            0x58 => Ok(Self::CLI),

            0xB8 => Ok(Self::CLV),

            0xC9 => Ok(Self::CMP(AddressingMode::IMMEDIATE)),
            0xC5 => Ok(Self::CMP(AddressingMode::ZERO_PAGE)),
            0xD5 => Ok(Self::CMP(AddressingMode::ZERO_PAGE_X)),
            0xCD => Ok(Self::CMP(AddressingMode::ABSOLUTE)),
            0xDD => Ok(Self::CMP(AddressingMode::ABSOLUTE_X)),
            0xD9 => Ok(Self::CMP(AddressingMode::ABSOLUTE_Y)),
            0xC1 => Ok(Self::CMP(AddressingMode::INDIRECT_X)),
            0xD1 => Ok(Self::CMP(AddressingMode::INDIRECT_Y)),

            0xE0 => Ok(Self::CPX(AddressingMode::IMMEDIATE)),
            0xE4 => Ok(Self::CPX(AddressingMode::ZERO_PAGE)),
            0xEC => Ok(Self::CPX(AddressingMode::ABSOLUTE)),

            0xC0 => Ok(Self::CPY(AddressingMode::IMMEDIATE)),
            0xC4 => Ok(Self::CPY(AddressingMode::ZERO_PAGE)),
            0xCC => Ok(Self::CPY(AddressingMode::ABSOLUTE)),

            0xC6 => Ok(Self::DEC(AddressingMode::ZERO_PAGE)),
            0xD6 => Ok(Self::DEC(AddressingMode::ZERO_PAGE_X)),
            0xCE => Ok(Self::DEC(AddressingMode::ABSOLUTE)),
            0xDE => Ok(Self::DEC(AddressingMode::ABSOLUTE_X)),

            0xCA => Ok(Self::DEX),

            0x88 => Ok(Self::DEY),

            0x49 => Ok(Self::EOR(AddressingMode::IMMEDIATE)),
            0x45 => Ok(Self::EOR(AddressingMode::ZERO_PAGE)),
            0x55 => Ok(Self::EOR(AddressingMode::ZERO_PAGE_X)),
            0x4D => Ok(Self::EOR(AddressingMode::ABSOLUTE)),
            0x5D => Ok(Self::EOR(AddressingMode::ABSOLUTE_X)),
            0x59 => Ok(Self::EOR(AddressingMode::ABSOLUTE_Y)),
            0x41 => Ok(Self::EOR(AddressingMode::INDIRECT_X)),
            0x51 => Ok(Self::EOR(AddressingMode::INDIRECT_Y)),

            0xE6 => Ok(Self::INC(AddressingMode::ZERO_PAGE)),
            0xF6 => Ok(Self::INC(AddressingMode::ZERO_PAGE_X)),
            0xEE => Ok(Self::INC(AddressingMode::ABSOLUTE)),
            0xFE => Ok(Self::INC(AddressingMode::ABSOLUTE_X)),

            0xE8 => Ok(Self::INX),

            0xC8 => Ok(Self::INY),

            0x4C => Ok(Self::JMP(AddressingMode::ABSOLUTE)),
            0x6C => Ok(Self::JMP(AddressingMode::INDIRECT)),

            0x20 => Ok(Self::JSR(AddressingMode::ABSOLUTE)),

            0xA2 => Ok(Self::LDX(AddressingMode::IMMEDIATE)),
            0xA6 => Ok(Self::LDX(AddressingMode::ZERO_PAGE)),
            0xB6 => Ok(Self::LDX(AddressingMode::ZERO_PAGE_Y)),
            0xAE => Ok(Self::LDX(AddressingMode::ABSOLUTE)),
            0xBE => Ok(Self::LDX(AddressingMode::ABSOLUTE_Y)),

            0xA0 => Ok(Self::LDY(AddressingMode::IMMEDIATE)),
            0xA4 => Ok(Self::LDY(AddressingMode::ZERO_PAGE)),
            0xB4 => Ok(Self::LDY(AddressingMode::ZERO_PAGE_X)),
            0xAC => Ok(Self::LDY(AddressingMode::ABSOLUTE)),
            0xBC => Ok(Self::LDY(AddressingMode::ABSOLUTE_X)),

            0x4A => Ok(Self::LSR(AddressingMode::ACCUMULATOR)),
            0x46 => Ok(Self::LSR(AddressingMode::ZERO_PAGE)),
            0x56 => Ok(Self::LSR(AddressingMode::ZERO_PAGE_X)),
            0x4E => Ok(Self::LSR(AddressingMode::ABSOLUTE)),
            0x5E => Ok(Self::LSR(AddressingMode::ABSOLUTE_X)),

            0xEA => Ok(Self::NOP),

            0x09 => Ok(Self::ORA(AddressingMode::IMMEDIATE)),
            0x05 => Ok(Self::ORA(AddressingMode::ZERO_PAGE)),
            0x15 => Ok(Self::ORA(AddressingMode::ZERO_PAGE_X)),
            0x0D => Ok(Self::ORA(AddressingMode::ABSOLUTE)),
            0x1D => Ok(Self::ORA(AddressingMode::ABSOLUTE_X)),
            0x19 => Ok(Self::ORA(AddressingMode::ABSOLUTE_Y)),
            0x01 => Ok(Self::ORA(AddressingMode::INDIRECT_X)),
            0x11 => Ok(Self::ORA(AddressingMode::INDIRECT_Y)),

            0x48 => Ok(Self::PHA),

            0x08 => Ok(Self::PHP),

            0x68 => Ok(Self::PLA),

            0x28 => Ok(Self::PLP),

            0x2A => Ok(Self::ROL(AddressingMode::ACCUMULATOR)),
            0x26 => Ok(Self::ROL(AddressingMode::ZERO_PAGE)),
            0x36 => Ok(Self::ROL(AddressingMode::ZERO_PAGE_X)),
            0x2E => Ok(Self::ROL(AddressingMode::ABSOLUTE)),
            0x3E => Ok(Self::ROL(AddressingMode::ABSOLUTE_X)),

            0x6A => Ok(Self::ROR(AddressingMode::ACCUMULATOR)),
            0x66 => Ok(Self::ROR(AddressingMode::ZERO_PAGE)),
            0x76 => Ok(Self::ROR(AddressingMode::ZERO_PAGE_X)),
            0x6E => Ok(Self::ROR(AddressingMode::ABSOLUTE)),
            0x7E => Ok(Self::ROR(AddressingMode::ABSOLUTE_X)),

            0x40 => Ok(Self::RTI),

            0x60 => Ok(Self::RTS),

            0xE9 => Ok(Self::SBC(AddressingMode::IMMEDIATE)),
            0xE5 => Ok(Self::SBC(AddressingMode::ZERO_PAGE)),
            0xF5 => Ok(Self::SBC(AddressingMode::ZERO_PAGE_X)),
            0xED => Ok(Self::SBC(AddressingMode::ABSOLUTE)),
            0xFD => Ok(Self::SBC(AddressingMode::ABSOLUTE_X)),
            0xF9 => Ok(Self::SBC(AddressingMode::ABSOLUTE_Y)),
            0xE1 => Ok(Self::SBC(AddressingMode::INDIRECT_X)),
            0xF1 => Ok(Self::SBC(AddressingMode::INDIRECT_Y)),

            0x38 => Ok(Self::SEC),

            0xF8 => Ok(Self::SED),

            0x78 => Ok(Self::SEI),

            0x85 => Ok(Self::STA(AddressingMode::ZERO_PAGE)),
            0x95 => Ok(Self::STA(AddressingMode::ZERO_PAGE_X)),
            0x8D => Ok(Self::STA(AddressingMode::ABSOLUTE)),
            0x9D => Ok(Self::STA(AddressingMode::ABSOLUTE_X)),
            0x99 => Ok(Self::STA(AddressingMode::ABSOLUTE_Y)),
            0x81 => Ok(Self::STA(AddressingMode::INDIRECT_X)),
            0x91 => Ok(Self::STA(AddressingMode::INDIRECT_Y)),

            0xA9 => Ok(Self::LDA(AddressingMode::IMMEDIATE)),
            0xA5 => Ok(Self::LDA(AddressingMode::ZERO_PAGE)),
            0xB5 => Ok(Self::LDA(AddressingMode::ZERO_PAGE_X)),
            0xAD => Ok(Self::LDA(AddressingMode::ABSOLUTE)),
            0xBD => Ok(Self::LDA(AddressingMode::ABSOLUTE_X)),
            0xB9 => Ok(Self::LDA(AddressingMode::ABSOLUTE_Y)),
            0xA1 => Ok(Self::LDA(AddressingMode::INDIRECT_X)),
            0xB1 => Ok(Self::LDA(AddressingMode::INDIRECT_Y)),

            0x86 => Ok(Self::STX(AddressingMode::ZERO_PAGE)),
            0x96 => Ok(Self::STX(AddressingMode::ZERO_PAGE_Y)),
            0x8E => Ok(Self::STX(AddressingMode::ABSOLUTE)),

            0x84 => Ok(Self::STY(AddressingMode::ZERO_PAGE)),
            0x94 => Ok(Self::STY(AddressingMode::ZERO_PAGE_X)),
            0x8C => Ok(Self::STY(AddressingMode::ABSOLUTE)),

            0xAA => Ok(Self::TAX),
            0xA8 => Ok(Self::TAY),
            0x8A => Ok(Self::TXA),
            0x9A => Ok(Self::TXS),
            0xBA => Ok(Self::TSX),
            0x98 => Ok(Self::TYA),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
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
}
