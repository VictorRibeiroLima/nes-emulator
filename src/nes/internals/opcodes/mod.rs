#[cfg(test)]
mod test;

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
