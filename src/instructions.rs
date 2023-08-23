use crate::memory::Memory;

#[derive(Clone, Copy)]
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
    pub fn extra_bytes(self) -> u16 {
        match self {
            AddressingMode::Accumulator => 0,
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => 1,
            AddressingMode::Zeropage => 1,
            AddressingMode::ZeropageX => 1,
            AddressingMode::ZeropageY => 1,
            AddressingMode::Relative => 1,
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Indirect => 2,
            AddressingMode::IndirectX => 1,
            AddressingMode::IndirectY => 1,
        }
    }

    pub fn get_data(&self, memory: &Memory, pc: u16) -> u16 {
        unimplemented!()
    }

    pub fn get_address_u8(&self, memory: &Memory, pc: u16) -> &mut u8 {
        unimplemented!()
    }

    pub fn get_address_u16(&self, memory: &Memory, pc: u16) -> &mut u16 {
        unimplemented!()
    }
}

macro_rules! addressing_instructions {
    ($($instruction:ident),*) => {
        pub enum Instruction {
            $(
                $instruction(AddressingMode),
            )*
        }

        impl Instruction {
            pub fn addressing_mode(&self) -> &AddressingMode {
                match self {
                    $(
                        Instruction::$instruction(mode) => mode,
                    )*
                }
            }
        }
    };
}

addressing_instructions!(
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP, CPX,
    CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA,
    PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA
);

impl Instruction {
    // opcodes from https://www.masswerk.at/6502/6502_instruction_set.html#ASL
    pub fn decode(code: u8) -> Option<Self> {
        match code {
            // ADC -- Add Memory to Accumulator with Carry
            0x69 => Some(Instruction::ADC(AddressingMode::Immediate)),
            0x65 => Some(Instruction::ADC(AddressingMode::Zeropage)),
            0x75 => Some(Instruction::ADC(AddressingMode::ZeropageX)),
            0x6D => Some(Instruction::ADC(AddressingMode::Absolute)),
            0x7D => Some(Instruction::ADC(AddressingMode::AbsoluteX)),
            0x79 => Some(Instruction::ADC(AddressingMode::AbsoluteY)),
            0x61 => Some(Instruction::ADC(AddressingMode::IndirectX)),
            0x71 => Some(Instruction::ADC(AddressingMode::Indirect)),

            // AND -- AND Memory with Accumulator
            0x29 => Some(Instruction::AND(AddressingMode::Immediate)),
            0x25 => Some(Instruction::AND(AddressingMode::Zeropage)),
            0x35 => Some(Instruction::AND(AddressingMode::ZeropageX)),
            0x2D => Some(Instruction::AND(AddressingMode::Absolute)),
            0x3D => Some(Instruction::AND(AddressingMode::AbsoluteX)),
            0x39 => Some(Instruction::AND(AddressingMode::AbsoluteY)),
            0x21 => Some(Instruction::AND(AddressingMode::IndirectX)),
            0x31 => Some(Instruction::AND(AddressingMode::IndirectY)),

            // ASL -- Shift Left One Bit With
            0x0A => Some(Instruction::ASL(AddressingMode::Accumulator)),
            0x06 => Some(Instruction::ASL(AddressingMode::Zeropage)),
            0x16 => Some(Instruction::ASL(AddressingMode::ZeropageX)),
            0x0E => Some(Instruction::ASL(AddressingMode::Absolute)),
            0x1E => Some(Instruction::ASL(AddressingMode::AbsoluteX)),

            // BCC -- Branch on Carry Clear
            0x90 => Some(Instruction::BCC(AddressingMode::Relative)),

            // BCS -- Branch on Carry Set
            0xB0 => Some(Instruction::BCS(AddressingMode::Relative)),

            // BEQ -- Branch on Result Zero
            0xF0 => Some(Instruction::BEQ(AddressingMode::Relative)),

            // BIT -- Test Bits in Memory with Accumulator
            0x24 => Some(Instruction::BIT(AddressingMode::Zeropage)),
            0x2C => Some(Instruction::BIT(AddressingMode::Absolute)),

            // BMI -- Branch on Result Minux
            0x30 => Some(Instruction::BMI(AddressingMode::Relative)),

            // BNE -- Branch on Result not Zero
            0xD0 => Some(Instruction::BNE(AddressingMode::Relative)),

            // BPL -- Branch on Result Plus
            0x10 => Some(Instruction::BPL(AddressingMode::Relative)),

            // BRK -- Force Break
            0x00 => Some(Instruction::BRK(AddressingMode::Implied)),

            // BVC -- Branch on Overflow Clear
            0x50 => Some(Instruction::BVC(AddressingMode::Relative)),

            // BVS -- Branch on Overflow Set
            0x70 => Some(Instruction::BVS(AddressingMode::Relative)),

            // CLC -- Clear Carry Flag
            0x18 => Some(Instruction::CLC(AddressingMode::Implied)),

            // CLD -- Clear Decimal Mode
            0xD8 => Some(Instruction::CLD(AddressingMode::Implied)),

            // CLI -- Clear Interrupt Disable Bit
            0x58 => Some(Instruction::CLI(AddressingMode::Implied)),

            // CLV -- Clear Overflow Flag
            0xB8 => Some(Instruction::CLV(AddressingMode::Implied)),

            // CMP -- Compare Memory with Accumulator
            0xC9 => Some(Instruction::CMP(AddressingMode::Immediate)),
            0xC5 => Some(Instruction::CMP(AddressingMode::Zeropage)),
            0xD5 => Some(Instruction::CMP(AddressingMode::ZeropageX)),
            0xCD => Some(Instruction::CMP(AddressingMode::Absolute)),
            0xDD => Some(Instruction::CMP(AddressingMode::AbsoluteX)),
            0xD9 => Some(Instruction::CMP(AddressingMode::AbsoluteY)),
            0xC1 => Some(Instruction::CMP(AddressingMode::IndirectX)),
            0xD1 => Some(Instruction::CMP(AddressingMode::IndirectY)),

            // CPX -- Compare Memory and Index X
            0xE0 => Some(Instruction::CPX(AddressingMode::Immediate)),
            0xE4 => Some(Instruction::CPX(AddressingMode::Zeropage)),
            0xEC => Some(Instruction::CPX(AddressingMode::Absolute)),

            // CPY -- Compare Memory and Index Y
            0xC0 => Some(Instruction::CPY(AddressingMode::Immediate)),
            0xC4 => Some(Instruction::CPY(AddressingMode::Zeropage)),
            0xCC => Some(Instruction::CPY(AddressingMode::Absolute)),

            // DEC -- Decrement Memory By One
            0xC6 => Some(Instruction::DEC(AddressingMode::Zeropage)),
            0xD6 => Some(Instruction::DEC(AddressingMode::ZeropageX)),
            0xCE => Some(Instruction::DEC(AddressingMode::Absolute)),
            0xDE => Some(Instruction::DEC(AddressingMode::AbsoluteX)),

            // DEX -- Decrement Index X by One
            0xCA => Some(Instruction::DEX(AddressingMode::Implied)),

            // DEY -- Decrement Index Y by One
            0x88 => Some(Instruction::DEY(AddressingMode::Implied)),

            // EOR -- Exclusive-OR Memory with Accumulator
            0x49 => Some(Instruction::EOR(AddressingMode::Immediate)),
            0x45 => Some(Instruction::EOR(AddressingMode::Zeropage)),
            0x55 => Some(Instruction::EOR(AddressingMode::ZeropageX)),
            0x4D => Some(Instruction::EOR(AddressingMode::Absolute)),
            0x5D => Some(Instruction::EOR(AddressingMode::AbsoluteX)),
            0x59 => Some(Instruction::EOR(AddressingMode::AbsoluteY)),
            0x41 => Some(Instruction::EOR(AddressingMode::IndirectX)),
            0x51 => Some(Instruction::EOR(AddressingMode::IndirectY)),

            // INC -- Increment Memory By One
            0xE6 => Some(Instruction::INC(AddressingMode::Zeropage)),
            0xF6 => Some(Instruction::INC(AddressingMode::ZeropageX)),
            0xEE => Some(Instruction::INC(AddressingMode::Absolute)),
            0xFE => Some(Instruction::INC(AddressingMode::AbsoluteX)),

            // INX -- Increment Index X by One
            0xE8 => Some(Instruction::INX(AddressingMode::Implied)),

            // INY -- Increment Index Y by One
            0xC8 => Some(Instruction::INY(AddressingMode::Implied)),

            // JMP -- Jump to New Location
            0x4C => Some(Instruction::JMP(AddressingMode::Absolute)),
            0x6C => Some(Instruction::JMP(AddressingMode::Indirect)),

            // JSR -- Jump to New Location Saving Return Address
            0x20 => Some(Instruction::JSR(AddressingMode::Absolute)),

            // LDA -- Load Accumulator with Memory
            0xA9 => Some(Instruction::LDA(AddressingMode::Immediate)),
            0xA5 => Some(Instruction::LDA(AddressingMode::Zeropage)),
            0xB5 => Some(Instruction::LDA(AddressingMode::ZeropageX)),
            0xAD => Some(Instruction::LDA(AddressingMode::Absolute)),
            0xBD => Some(Instruction::LDA(AddressingMode::AbsoluteX)),
            0xB9 => Some(Instruction::LDA(AddressingMode::AbsoluteY)),
            0xA1 => Some(Instruction::LDA(AddressingMode::IndirectX)),
            0xB1 => Some(Instruction::LDA(AddressingMode::IndirectY)),

            // LDX -- Load Index X with Memory
            0xA2 => Some(Instruction::LDX(AddressingMode::Immediate)),
            0xA6 => Some(Instruction::LDX(AddressingMode::Zeropage)),
            0xB6 => Some(Instruction::LDX(AddressingMode::ZeropageX)),
            0xAE => Some(Instruction::LDX(AddressingMode::Absolute)),
            0xBE => Some(Instruction::LDX(AddressingMode::AbsoluteX)),

            // LDY -- Load Index Y with Memory
            0xA0 => Some(Instruction::LDY(AddressingMode::Immediate)),
            0xA4 => Some(Instruction::LDY(AddressingMode::Zeropage)),
            0xB4 => Some(Instruction::LDY(AddressingMode::ZeropageX)),
            0xAC => Some(Instruction::LDY(AddressingMode::Absolute)),
            0xBC => Some(Instruction::LDY(AddressingMode::AbsoluteX)),

            // LSR -- Shift One Bit Right (Memory or Accumulator)
            0x4A => Some(Instruction::LSR(AddressingMode::Accumulator)),
            0x46 => Some(Instruction::LSR(AddressingMode::Zeropage)),
            0x56 => Some(Instruction::LSR(AddressingMode::ZeropageX)),
            0x4E => Some(Instruction::LSR(AddressingMode::Absolute)),
            0x5E => Some(Instruction::LSR(AddressingMode::AbsoluteX)),

            // NOP -- No Operation
            0xEA => Some(Instruction::NOP(AddressingMode::Relative)),

            // ORA -- OR Memory with Accumulator
            0x09 => Some(Instruction::ORA(AddressingMode::Immediate)),
            0x05 => Some(Instruction::ORA(AddressingMode::Zeropage)),
            0x15 => Some(Instruction::ORA(AddressingMode::ZeropageX)),
            0x0D => Some(Instruction::ORA(AddressingMode::Absolute)),
            0x1D => Some(Instruction::ORA(AddressingMode::AbsoluteX)),
            0x19 => Some(Instruction::ORA(AddressingMode::AbsoluteY)),
            0x01 => Some(Instruction::ORA(AddressingMode::IndirectX)),
            0x11 => Some(Instruction::ORA(AddressingMode::IndirectY)),

            // PHA -- Push Accumulator on Stack
            0x48 => Some(Instruction::PHA(AddressingMode::Implied)),

            // PHP -- Push Processor Status on Stack
            0x08 => Some(Instruction::PHP(AddressingMode::Implied)),

            // PLA -- Pull Accumulator from Stack
            0x68 => Some(Instruction::PLA(AddressingMode::Implied)),

            // PLP -- Pull Processor Status from Stack
            0x28 => Some(Instruction::PLP(AddressingMode::Implied)),

            // ROL -- Rotate One Bit Left
            0x2A => Some(Instruction::ROL(AddressingMode::Accumulator)),
            0x26 => Some(Instruction::ROL(AddressingMode::Zeropage)),
            0x36 => Some(Instruction::ROL(AddressingMode::ZeropageX)),
            0x2E => Some(Instruction::ROL(AddressingMode::Absolute)),
            0x3E => Some(Instruction::ROL(AddressingMode::AbsoluteX)),

            // ROR -- Rotate One Bit Right
            0x6A => Some(Instruction::ROR(AddressingMode::Accumulator)),
            0x66 => Some(Instruction::ROR(AddressingMode::Zeropage)),
            0x76 => Some(Instruction::ROR(AddressingMode::ZeropageX)),
            0x6E => Some(Instruction::ROR(AddressingMode::Absolute)),
            0x7E => Some(Instruction::ROR(AddressingMode::AbsoluteX)),

            // RTI -- Return from Interrupt
            0x40 => Some(Instruction::RTI(AddressingMode::Implied)),

            // RTS -- Return from Subroutine
            0x60 => Some(Instruction::RTS(AddressingMode::Implied)),

            // SBC -- Subtract Memory from Accumulator with Borrow
            0xE9 => Some(Instruction::SBC(AddressingMode::Immediate)),
            0xE5 => Some(Instruction::SBC(AddressingMode::Zeropage)),
            0xF5 => Some(Instruction::SBC(AddressingMode::ZeropageX)),
            0xED => Some(Instruction::SBC(AddressingMode::Absolute)),
            0xFD => Some(Instruction::SBC(AddressingMode::AbsoluteX)),
            0xF9 => Some(Instruction::SBC(AddressingMode::AbsoluteY)),
            0xE1 => Some(Instruction::SBC(AddressingMode::IndirectX)),
            0xF1 => Some(Instruction::SBC(AddressingMode::IndirectY)),

            // SEC -- Set Carry Flag
            0x38 => Some(Instruction::SEC(AddressingMode::Implied)),

            // SED -- Set Decimal Flag
            0xF8 => Some(Instruction::SED(AddressingMode::Implied)),

            // SEI -- Set Interrupt Disable Status
            0x78 => Some(Instruction::SEI(AddressingMode::Implied)),

            // STA -- Subtract Memory from Accumulator with Borrow
            0x85 => Some(Instruction::STA(AddressingMode::Zeropage)),
            0x95 => Some(Instruction::STA(AddressingMode::ZeropageX)),
            0x8D => Some(Instruction::STA(AddressingMode::Absolute)),
            0x9D => Some(Instruction::STA(AddressingMode::AbsoluteX)),
            0x99 => Some(Instruction::STA(AddressingMode::AbsoluteY)),
            0x81 => Some(Instruction::STA(AddressingMode::IndirectX)),
            0x91 => Some(Instruction::STA(AddressingMode::IndirectY)),

            // STX -- Store Index X in Memory
            0x86 => Some(Instruction::STX(AddressingMode::Zeropage)),
            0x96 => Some(Instruction::STX(AddressingMode::ZeropageY)),
            0x8E => Some(Instruction::STX(AddressingMode::Absolute)),

            // STY -- Store Index Y in Memory
            0x84 => Some(Instruction::STY(AddressingMode::Zeropage)),
            0x94 => Some(Instruction::STY(AddressingMode::ZeropageY)),
            0x8C => Some(Instruction::STY(AddressingMode::Absolute)),

            // TAX -- Transfer Accumulator to Index X
            0xAA => Some(Instruction::TAX(AddressingMode::Implied)),

            // TAY -- Transfer Accumulator to Index Y
            0xA8 => Some(Instruction::TAY(AddressingMode::Implied)),

            // TSX -- Transfer Stack Pointer to Index X
            0xBA => Some(Instruction::TSX(AddressingMode::Implied)),

            // TXA -- Transfer Index X to Accumulator
            0x8A => Some(Instruction::TXA(AddressingMode::Implied)),

            // TXS -- Transfer Index X to Stack Register
            0x9A => Some(Instruction::TXS(AddressingMode::Implied)),

            // TYA -- Transfer Index Y to Accumulator
            0x98 => Some(Instruction::TYA(AddressingMode::Implied)),

            // NOP -- NOP for all Non Documented Instructions, for now...
            _ => None,
        }
    }
}
