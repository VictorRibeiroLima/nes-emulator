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
    IMMEDIATE,
    ZERO_PAGE,
    ZERO_PAGE_X,
    ZERO_PAGE_Y,
    ABSOLUTE,
    ABSOLUTE_X,
    ABSOLUTE_Y,
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcodes {
    AND(AddressingMode),
    ASL(AddressingMode),
    BBC,
    BCS,
    BEQ,
    BIT(AddressingMode),
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    TAX,
    TAY,
    INX,
    INY,
    BRK,
}

impl Opcodes {
    pub fn from_u8(value: u8) -> Result<Self, ()> {
        match value {
            0x29 => Ok(Self::AND(AddressingMode::IMMEDIATE)),
            0x25 => Ok(Self::AND(AddressingMode::ZERO_PAGE)),
            0x35 => Ok(Self::AND(AddressingMode::ZERO_PAGE_X)),
            0x2D => Ok(Self::AND(AddressingMode::ABSOLUTE)),
            0x3D => Ok(Self::AND(AddressingMode::ABSOLUTE_X)),
            0x39 => Ok(Self::AND(AddressingMode::ABSOLUTE_Y)),
            0x21 => Ok(Self::AND(AddressingMode::INDIRECT_X)),
            0x31 => Ok(Self::AND(AddressingMode::INDIRECT_Y)),

            0x0A => Ok(Self::ASL(AddressingMode::IMMEDIATE)),
            0x06 => Ok(Self::ASL(AddressingMode::ZERO_PAGE)),
            0x16 => Ok(Self::ASL(AddressingMode::ZERO_PAGE_X)),
            0x0E => Ok(Self::ASL(AddressingMode::ABSOLUTE)),
            0x1E => Ok(Self::ASL(AddressingMode::ABSOLUTE_X)),

            0x90 => Ok(Self::BBC),

            0xB0 => Ok(Self::BCS),

            0xF0 => Ok(Self::BEQ),

            0x24 => Ok(Self::BIT(AddressingMode::ZERO_PAGE)),
            0x2C => Ok(Self::BIT(AddressingMode::ABSOLUTE)),

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
            0x86 => Ok(Self::STX(AddressingMode::ZERO_PAGE)),
            0x96 => Ok(Self::STX(AddressingMode::ZERO_PAGE_Y)),
            0x8E => Ok(Self::STX(AddressingMode::ABSOLUTE)),
            0x84 => Ok(Self::STY(AddressingMode::ZERO_PAGE)),
            0x94 => Ok(Self::STY(AddressingMode::ZERO_PAGE_Y)),
            0x8C => Ok(Self::STY(AddressingMode::ABSOLUTE)),
            0xAA => Ok(Self::TAX),
            0xA8 => Ok(Self::TAY),
            0xE8 => Ok(Self::INX),
            0xC8 => Ok(Self::INY),
            0x00 => Ok(Self::BRK),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    test_opcodes!(LDY, 0xA4, ZERO_PAGE, test_ldy_zero_page);
    test_opcodes!(LDY, 0xB4, ZERO_PAGE_X, test_ldy_zero_page_x);
    test_opcodes!(LDY, 0xAC, ABSOLUTE, test_ldy_absolute);
    test_opcodes!(LDY, 0xBC, ABSOLUTE_X, test_ldy_absolute_x);

    test_opcodes!(TAX, 0xAA, test_tax);
    test_opcodes!(TAY, 0xA8, test_tay);
    test_opcodes!(INX, 0xE8, test_inx);
    test_opcodes!(INY, 0xC8, test_iny);
    test_opcodes!(BRK, 0x00, test_brk);
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
    test_opcodes!(STY, 0x94, ZERO_PAGE_Y, test_from_u8_sty_zero_page_y);
    test_opcodes!(STY, 0x8C, ABSOLUTE, test_from_u8_sty_absolute);

    test_opcodes!(AND, 0x29, IMMEDIATE, test_from_u8_and_immediate);
    test_opcodes!(AND, 0x25, ZERO_PAGE, test_from_u8_and_zero_page);
    test_opcodes!(AND, 0x35, ZERO_PAGE_X, test_from_u8_and_zero_page_x);
    test_opcodes!(AND, 0x2D, ABSOLUTE, test_from_u8_and_absolute);
    test_opcodes!(AND, 0x3D, ABSOLUTE_X, test_from_u8_and_absolute_x);
    test_opcodes!(AND, 0x39, ABSOLUTE_Y, test_from_u8_and_absolute_y);
    test_opcodes!(AND, 0x21, INDIRECT_X, test_from_u8_and_indirect_x);
    test_opcodes!(AND, 0x31, INDIRECT_Y, test_from_u8_and_indirect_y);

    test_opcodes!(BBC, 0x90, test_from_u8_bbc);

    test_opcodes!(BCS, 0xB0, test_from_u8_bcs);

    test_opcodes!(BEQ, 0xF0, test_from_u8_beq);

    test_opcodes!(BIT, 0x24, ZERO_PAGE, test_from_u8_bit_zero_page);

    test_opcodes!(BIT, 0x2C, ABSOLUTE, test_from_u8_bit_absolute);

    #[test]
    fn test_invalid_opcode() {
        assert_eq!(Opcodes::from_u8(0xFF), Err(()));
    }
}
