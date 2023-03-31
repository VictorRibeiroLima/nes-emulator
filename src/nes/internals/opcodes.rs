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

    #[test]
    fn test_from_u8() {
        assert_eq!(
            Opcodes::from_u8(0xA9),
            Ok(Opcodes::LDA(AddressingMode::IMMEDIATE))
        );
        assert_eq!(
            Opcodes::from_u8(0xA5),
            Ok(Opcodes::LDA(AddressingMode::ZERO_PAGE))
        );
        assert_eq!(
            Opcodes::from_u8(0xB5),
            Ok(Opcodes::LDA(AddressingMode::ZERO_PAGE_X))
        );
        assert_eq!(
            Opcodes::from_u8(0xAD),
            Ok(Opcodes::LDA(AddressingMode::ABSOLUTE))
        );
        assert_eq!(
            Opcodes::from_u8(0xBD),
            Ok(Opcodes::LDA(AddressingMode::ABSOLUTE_X))
        );
        assert_eq!(
            Opcodes::from_u8(0xB9),
            Ok(Opcodes::LDA(AddressingMode::ABSOLUTE_Y))
        );
        assert_eq!(
            Opcodes::from_u8(0xA1),
            Ok(Opcodes::LDA(AddressingMode::INDIRECT_X))
        );
        assert_eq!(
            Opcodes::from_u8(0xB1),
            Ok(Opcodes::LDA(AddressingMode::INDIRECT_Y))
        );
        assert_eq!(
            Opcodes::from_u8(0xA2),
            Ok(Opcodes::LDX(AddressingMode::IMMEDIATE))
        );
        assert_eq!(
            Opcodes::from_u8(0xA6),
            Ok(Opcodes::LDX(AddressingMode::ZERO_PAGE))
        );
        assert_eq!(
            Opcodes::from_u8(0xB6),
            Ok(Opcodes::LDX(AddressingMode::ZERO_PAGE_Y))
        );
        assert_eq!(
            Opcodes::from_u8(0xAE),
            Ok(Opcodes::LDX(AddressingMode::ABSOLUTE))
        );
        assert_eq!(
            Opcodes::from_u8(0xBE),
            Ok(Opcodes::LDX(AddressingMode::ABSOLUTE_Y))
        );
        assert_eq!(
            Opcodes::from_u8(0xA0),
            Ok(Opcodes::LDY(AddressingMode::IMMEDIATE))
        );
        assert_eq!(
            Opcodes::from_u8(0xA4),
            Ok(Opcodes::LDY(AddressingMode::ZERO_PAGE))
        );
        assert_eq!(
            Opcodes::from_u8(0xB4),
            Ok(Opcodes::LDY(AddressingMode::ZERO_PAGE_X))
        );
        assert_eq!(
            Opcodes::from_u8(0xAC),
            Ok(Opcodes::LDY(AddressingMode::ABSOLUTE))
        );
        assert_eq!(
            Opcodes::from_u8(0xBC),
            Ok(Opcodes::LDY(AddressingMode::ABSOLUTE_X))
        );
        assert_eq!(Opcodes::from_u8(0xAA), Ok(Opcodes::TAX));
        assert_eq!(Opcodes::from_u8(0xA8), Ok(Opcodes::TAY));
        assert_eq!(Opcodes::from_u8(0xE8), Ok(Opcodes::INX));
        assert_eq!(Opcodes::from_u8(0xC8), Ok(Opcodes::INY));
        assert_eq!(Opcodes::from_u8(0x00), Ok(Opcodes::BRK));
        assert_eq!(Opcodes::from_u8(0xFF), Err(()));
    }
}
