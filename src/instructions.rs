use crate::{memory::Memory, status::Status};

pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    IndirectX,
    IndirectY,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
}

impl AddressingMode {
    pub fn get_address(&self) -> u16 {
        match self {
            AddressingMode::Accumulator => 0,
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::IndirectX => 1,
            AddressingMode::IndirectY => 1,
            AddressingMode::Zeropage => 1,
            AddressingMode::ZeropageX => 1,
            AddressingMode::ZeropageY => 1,
            AddressingMode::Relative => 1,
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
        }
    }
}

pub enum Instruction {
    ADC(AddressingMode),
    AND(AddressingMode),
    ASL(AddressingMode),
    BCC(AddressingMode),
    BCS(AddressingMode),
    BEQ(AddressingMode),
    BIT(AddressingMode),
    BMI(AddressingMode),
    BNE(AddressingMode),
    BPL(AddressingMode),
    BRK(AddressingMode),
    BVC(AddressingMode),
    BVS(AddressingMode),
    CLC(AddressingMode),
    CLD(AddressingMode),
    CLI(AddressingMode),
    CLV(AddressingMode),
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    DEX(AddressingMode),
    DEY(AddressingMode),
    EOR(AddressingMode),
    INC(AddressingMode),
    INX(AddressingMode),
    INY(AddressingMode),
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP(AddressingMode),
    ORA(AddressingMode),
    PHA(AddressingMode),
    PHP(AddressingMode),
    PLA(AddressingMode),
    PLP(AddressingMode),
    ROL(AddressingMode),
    ROR(AddressingMode),
    RTI(AddressingMode),
    RTS(AddressingMode),
    SBC(AddressingMode),
    SEC(AddressingMode),
    SED(AddressingMode),
    SEI(AddressingMode),
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    TAX(AddressingMode),
    TAY(AddressingMode),
    TSX(AddressingMode),
    TXA(AddressingMode),
    TXS(AddressingMode),
    TYA(AddressingMode),
}

impl Instruction {
    // opcodes from https://www.masswerk.at/6502/6502_instruction_set.html#ASL
    pub fn decode(code: u8) -> Self {
        match code {
            // ADC -- Add Memory to Accumulator with Carry
            0x69 => Instruction::ADC(AddressingMode::Immediate),
            0x65 => Instruction::ADC(AddressingMode::Zeropage),
            0x75 => Instruction::ADC(AddressingMode::ZeropageX),
            0x6D => Instruction::ADC(AddressingMode::Absolute),
            0x7D => Instruction::ADC(AddressingMode::AbsoluteX),
            0x79 => Instruction::ADC(AddressingMode::AbsoluteY),
            0x61 => Instruction::ADC(AddressingMode::IndirectX),
            0x71 => Instruction::ADC(AddressingMode::Indirect),

            // AND -- AND Memory with Accumulator
            0x29 => Instruction::AND(AddressingMode::Immediate),
            0x25 => Instruction::AND(AddressingMode::Zeropage),
            0x35 => Instruction::AND(AddressingMode::ZeropageX),
            0x2D => Instruction::AND(AddressingMode::Absolute),
            0x3D => Instruction::AND(AddressingMode::AbsoluteX),
            0x39 => Instruction::AND(AddressingMode::AbsoluteY),
            0x21 => Instruction::AND(AddressingMode::IndirectX),
            0x31 => Instruction::AND(AddressingMode::IndirectY),

            // ASL -- Shift Left One Bit With
            0x0A => Instruction::ASL(AddressingMode::Accumulator),
            0x06 => Instruction::ASL(AddressingMode::Zeropage),
            0x16 => Instruction::ASL(AddressingMode::ZeropageX),
            0x0E => Instruction::ASL(AddressingMode::Absolute),
            0x1E => Instruction::ASL(AddressingMode::AbsoluteX),

            // BCC -- Branch on Carry Clear
            0x90 => Instruction::BCC(AddressingMode::Relative),

            // BCS -- Branch on Carry Set
            0xB0 => Instruction::BCS(AddressingMode::Relative),

            // BEQ -- Branch on Result Zero
            0xF0 => Instruction::BEQ(AddressingMode::Relative),

            // BIT -- Test Bits in Memory with Accumulator
            0x24 => Instruction::BIT(AddressingMode::Zeropage),
            0x2C => Instruction::BIT(AddressingMode::Absolute),

            // BMI -- Branch on Result Minux
            0x30 => Instruction::BMI(AddressingMode::Relative),

            // BNE -- Branch on Result not Zero
            0xD0 => Instruction::BNE(AddressingMode::Relative),

            // BPL -- Branch on Result Plus
            0x10 => Instruction::BPL(AddressingMode::Relative),

            // BRK -- Force Break
            0x00 => Instruction::BRK(AddressingMode::Implied),

            // BVC -- Branch on Overflow Clear
            0x50 => Instruction::BVC(AddressingMode::Relative),

            // BVS -- Branch on Overflow Set
            0x70 => Instruction::BVS(AddressingMode::Relative),

            // CLC -- Clear Carry Flag
            0x18 => Instruction::CLC(AddressingMode::Implied),

            // CLD -- Clear Decimal Mode
            0xD8 => Instruction::CLD(AddressingMode::Implied),

            // CLI -- Clear Interrupt Disable Bit
            0x58 => Instruction::CLI(AddressingMode::Implied),

            // CLV -- Clear Overflow Flag
            0xB8 => Instruction::CLV(AddressingMode::Implied),

            // CMP -- Compare Memory with Accumulator
            0xC9 => Instruction::CMP(AddressingMode::Immediate),
            0xC5 => Instruction::CMP(AddressingMode::Zeropage),
            0xD5 => Instruction::CMP(AddressingMode::ZeropageX),
            0xCD => Instruction::CMP(AddressingMode::Absolute),
            0xDD => Instruction::CMP(AddressingMode::AbsoluteX),
            0xD9 => Instruction::CMP(AddressingMode::AbsoluteY),
            0xC1 => Instruction::CMP(AddressingMode::IndirectX),
            0xD1 => Instruction::CMP(AddressingMode::IndirectY),

            // CPX -- Compare Memory and Index X
            0xE0 => Instruction::CPX(AddressingMode::Immediate),
            0xE4 => Instruction::CPX(AddressingMode::Zeropage),
            0xEC => Instruction::CPX(AddressingMode::Absolute),

            // CPY -- Compare Memory and Index Y
            0xC0 => Instruction::CPY(AddressingMode::Immediate),
            0xC4 => Instruction::CPY(AddressingMode::Zeropage),
            0xCC => Instruction::CPY(AddressingMode::Absolute),

            // DEC -- Decrement Memory By One
            0xC6 => Instruction::DEC(AddressingMode::Zeropage),
            0xD6 => Instruction::DEC(AddressingMode::ZeropageX),
            0xCE => Instruction::DEC(AddressingMode::Absolute),
            0xDE => Instruction::DEC(AddressingMode::AbsoluteX),

            // DEX -- Decrement Index X by One
            0xCA => Instruction::DEX(AddressingMode::Implied),

            // DEY -- Decrement Index Y by One
            0x88 => Instruction::DEY(AddressingMode::Implied),

            // EOR -- Exclusive-OR Memory with Accumulator
            0x49 => Instruction::EOR(AddressingMode::Immediate),
            0x45 => Instruction::EOR(AddressingMode::Zeropage),
            0x55 => Instruction::EOR(AddressingMode::ZeropageX),
            0x4D => Instruction::EOR(AddressingMode::Absolute),
            0x5D => Instruction::EOR(AddressingMode::AbsoluteX),
            0x59 => Instruction::EOR(AddressingMode::AbsoluteY),
            0x41 => Instruction::EOR(AddressingMode::IndirectX),
            0x51 => Instruction::EOR(AddressingMode::IndirectY),

            // INC -- Increment Memory By One
            0xE6 => Instruction::INC(AddressingMode::Zeropage),
            0xF6 => Instruction::INC(AddressingMode::ZeropageX),
            0xEE => Instruction::INC(AddressingMode::Absolute),
            0xFE => Instruction::INC(AddressingMode::AbsoluteX),

            // INX -- Increment Index X by One
            0xE8 => Instruction::INX(AddressingMode::Implied),

            // INY -- Increment Index Y by One
            0xC8 => Instruction::INY(AddressingMode::Implied),

            // JMP -- Jump to New Location
            0x4C => Instruction::JMP(AddressingMode::Absolute),
            0x6C => Instruction::JMP(AddressingMode::Indirect),

            // JSR -- Jump to New Location Saving Return Address
            0x20 => Instruction::JSR(AddressingMode::Absolute),

            // LDA -- Load Accumulator with Memory
            0xA9 => Instruction::LDA(AddressingMode::Immediate),
            0xA5 => Instruction::LDA(AddressingMode::Zeropage),
            0xB5 => Instruction::LDA(AddressingMode::ZeropageX),
            0xAD => Instruction::LDA(AddressingMode::Absolute),
            0xBD => Instruction::LDA(AddressingMode::AbsoluteX),
            0xB9 => Instruction::LDA(AddressingMode::AbsoluteY),
            0xA1 => Instruction::LDA(AddressingMode::IndirectX),
            0xB1 => Instruction::LDA(AddressingMode::IndirectY),

            // LDX -- Load Index X with Memory
            0xA2 => Instruction::LDX(AddressingMode::Immediate),
            0xA6 => Instruction::LDX(AddressingMode::Zeropage),
            0xB6 => Instruction::LDX(AddressingMode::ZeropageX),
            0xAE => Instruction::LDX(AddressingMode::Absolute),
            0xBE => Instruction::LDX(AddressingMode::AbsoluteX),

            // LDY -- Load Index Y with Memory
            0xA0 => Instruction::LDY(AddressingMode::Immediate),
            0xA4 => Instruction::LDY(AddressingMode::Zeropage),
            0xB4 => Instruction::LDY(AddressingMode::ZeropageX),
            0xAC => Instruction::LDY(AddressingMode::Absolute),
            0xBC => Instruction::LDY(AddressingMode::AbsoluteX),

            // LSR -- Shift One Bit Right (Memory or Accumulator)
            0x4A => Instruction::LSR(AddressingMode::Accumulator),
            0x46 => Instruction::LSR(AddressingMode::Zeropage),
            0x56 => Instruction::LSR(AddressingMode::ZeropageX),
            0x4E => Instruction::LSR(AddressingMode::Absolute),
            0x5E => Instruction::LSR(AddressingMode::AbsoluteX),

            // NOP -- No Operation
            0xEA => Instruction::NOP(AddressingMode::Relative),

            // ORA -- OR Memory with Accumulator
            0x09 => Instruction::ORA(AddressingMode::Immediate),
            0x05 => Instruction::ORA(AddressingMode::Zeropage),
            0x15 => Instruction::ORA(AddressingMode::ZeropageX),
            0x0D => Instruction::ORA(AddressingMode::Absolute),
            0x1D => Instruction::ORA(AddressingMode::AbsoluteX),
            0x19 => Instruction::ORA(AddressingMode::AbsoluteY),
            0x01 => Instruction::ORA(AddressingMode::IndirectX),
            0x11 => Instruction::ORA(AddressingMode::IndirectY),

            // PHA -- Push Accumulator on Stack
            0x48 => Instruction::PHA(AddressingMode::Implied),

            // PHP -- Push Processor Status on Stack
            0x08 => Instruction::PHP(AddressingMode::Implied),

            // PLA -- Pull Accumulator from Stack
            0x68 => Instruction::PLA(AddressingMode::Implied),

            // PLP -- Pull Processor Status from Stack
            0x28 => Instruction::PLP(AddressingMode::Implied),

            // ROL -- Rotate One Bit Left
            0x2A => Instruction::ROL(AddressingMode::Accumulator),
            0x26 => Instruction::ROL(AddressingMode::Zeropage),
            0x36 => Instruction::ROL(AddressingMode::ZeropageX),
            0x2E => Instruction::ROL(AddressingMode::Absolute),
            0x3E => Instruction::ROL(AddressingMode::AbsoluteX),

            // ROR -- Rotate One Bit Right
            0x6A => Instruction::ROR(AddressingMode::Accumulator),
            0x66 => Instruction::ROR(AddressingMode::Zeropage),
            0x76 => Instruction::ROR(AddressingMode::ZeropageX),
            0x6E => Instruction::ROR(AddressingMode::Absolute),
            0x7E => Instruction::ROR(AddressingMode::AbsoluteX),

            // RTI -- Return from Interrupt
            0x40 => Instruction::RTI(AddressingMode::Implied),

            // RTS -- Return from Subroutine
            0x60 => Instruction::RTS(AddressingMode::Implied),

            // SBC -- Subtract Memory from Accumulator with Borrow
            0xE9 => Instruction::SBC(AddressingMode::Immediate),
            0xE5 => Instruction::SBC(AddressingMode::Zeropage),
            0xF5 => Instruction::SBC(AddressingMode::ZeropageX),
            0xED => Instruction::SBC(AddressingMode::Absolute),
            0xFD => Instruction::SBC(AddressingMode::AbsoluteX),
            0xF9 => Instruction::SBC(AddressingMode::AbsoluteY),
            0xE1 => Instruction::SBC(AddressingMode::IndirectX),
            0xF1 => Instruction::SBC(AddressingMode::IndirectY),

            // SEC -- Set Carry Flag
            0x38 => Instruction::SEC(AddressingMode::Implied),

            // SED -- Set Decimal Flag
            0xF8 => Instruction::SED(AddressingMode::Implied),

            // SEI -- Set Interrupt Disable Status
            0x78 => Instruction::SEI(AddressingMode::Implied),

            // STA -- Subtract Memory from Accumulator with Borrow
            0x85 => Instruction::STA(AddressingMode::Zeropage),
            0x95 => Instruction::STA(AddressingMode::ZeropageX),
            0x8D => Instruction::STA(AddressingMode::Absolute),
            0x9D => Instruction::STA(AddressingMode::AbsoluteX),
            0x99 => Instruction::STA(AddressingMode::AbsoluteY),
            0x81 => Instruction::STA(AddressingMode::IndirectX),
            0x91 => Instruction::STA(AddressingMode::IndirectY),

            // STX -- Store Index X in Memory
            0x86 => Instruction::STX(AddressingMode::Zeropage),
            0x96 => Instruction::STX(AddressingMode::ZeropageY),
            0x8E => Instruction::STX(AddressingMode::Absolute),

            // STY -- Store Index Y in Memory
            0x84 => Instruction::STY(AddressingMode::Zeropage),
            0x94 => Instruction::STY(AddressingMode::ZeropageY),
            0x8C => Instruction::STY(AddressingMode::Absolute),

            // TAX -- Transfer Accumulator to Index X
            0xAA => Instruction::TAX(AddressingMode::Implied),

            // TAY -- Transfer Accumulator to Index Y
            0xA8 => Instruction::TAY(AddressingMode::Implied),

            // TSX -- Transfer Stack Pointer to Index X
            0xBA => Instruction::TSX(AddressingMode::Implied),

            // TXA -- Transfer Index X to Accumulator
            0x8A => Instruction::TXA(AddressingMode::Implied),

            // TXS -- Transfer Index X to Stack Register
            0x9A => Instruction::TXS(AddressingMode::Implied),

            // TYA -- Transfer Index Y to Accumulator
            0x98 => Instruction::TYA(AddressingMode::Implied),

            // NOP -- NOP for all Non Documented Instructions, for now...
            _ => Instruction::NOP(AddressingMode::Relative),
        }
    }

    pub fn execute(&self, status: &mut Status, memory: &mut Memory) {
        match self {
            ADC => {

            }

            _ => unimplemented!()
        }
    }
}
