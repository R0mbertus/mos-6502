use crate::{
    memory::Memory,
    registers::{Registers, Status},
};

#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
    Accumulator,
    Implied,
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    IndirectX,
    IndirectY,
    Relative,
}

impl AddressingMode {
    pub fn bytes_count(self) -> u16 {
        match self {
            Self::Accumulator => 1,
            Self::Implied => 1,
            Self::Immediate => 2,
            Self::Absolute => 3,
            Self::AbsoluteX => 3,
            Self::AbsoluteY => 3,
            Self::Indirect => 3,
            Self::ZeroPage => 2,
            Self::ZeroPageX => 2,
            Self::ZeroPageY => 2,
            Self::IndirectX => 2,
            Self::IndirectY => 2,
            Self::Relative => 2,
        }
    }

    pub fn get_index(&self, memory: &Memory, registers: &mut Registers) -> u16 {
        let data_start = registers.pc.wrapping_add(1);
        registers.pc += self.bytes_count();

        match self {
            Self::Accumulator | Self::Implied => 0,
            Self::Immediate => data_start,
            Self::Absolute => memory.get_word(data_start),
            Self::AbsoluteX => memory.get_word(data_start).wrapping_add(registers.x as u16),
            Self::AbsoluteY => memory.get_word(data_start).wrapping_add(registers.y as u16),
            Self::Indirect => memory.get_word(memory.get_word(data_start)),
            Self::ZeroPage => memory.get_byte(data_start) as u16,
            Self::ZeroPageX => (memory.get_byte(data_start).wrapping_add(registers.x)) as u16,
            Self::ZeroPageY => (memory.get_byte(data_start).wrapping_add(registers.y)) as u16,
            Self::IndirectX => {
                (memory.get_byte((memory.get_byte(data_start).wrapping_add(registers.x)) as u16)) as u16
            }
            Self::IndirectY => {
                (memory.get_byte(memory.get_byte(data_start) as u16).wrapping_add(registers.y)) as u16
            }
            Self::Relative => {
                let res = memory.get_byte(data_start);
                let sign_extend = if res & 0x80 == 0x80 { 0xffu8 } else { 0x0 };
                u16::from_le_bytes([res, sign_extend])
            },
        }
    }
}

macro_rules! addressing_instructions {
    ($($instruction:ident),*) => {
        #[derive(Debug)]
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
            0x65 => Some(Instruction::ADC(AddressingMode::ZeroPage)),
            0x75 => Some(Instruction::ADC(AddressingMode::ZeroPageX)),
            0x6D => Some(Instruction::ADC(AddressingMode::Absolute)),
            0x7D => Some(Instruction::ADC(AddressingMode::AbsoluteX)),
            0x79 => Some(Instruction::ADC(AddressingMode::AbsoluteY)),
            0x61 => Some(Instruction::ADC(AddressingMode::IndirectX)),
            0x71 => Some(Instruction::ADC(AddressingMode::Indirect)),

            // AND -- AND Memory with Accumulator
            0x29 => Some(Instruction::AND(AddressingMode::Immediate)),
            0x25 => Some(Instruction::AND(AddressingMode::ZeroPage)),
            0x35 => Some(Instruction::AND(AddressingMode::ZeroPageX)),
            0x2D => Some(Instruction::AND(AddressingMode::Absolute)),
            0x3D => Some(Instruction::AND(AddressingMode::AbsoluteX)),
            0x39 => Some(Instruction::AND(AddressingMode::AbsoluteY)),
            0x21 => Some(Instruction::AND(AddressingMode::IndirectX)),
            0x31 => Some(Instruction::AND(AddressingMode::IndirectY)),

            // ASL -- Shift Left One Bit With
            0x0A => Some(Instruction::ASL(AddressingMode::Accumulator)),
            0x06 => Some(Instruction::ASL(AddressingMode::ZeroPage)),
            0x16 => Some(Instruction::ASL(AddressingMode::ZeroPageX)),
            0x0E => Some(Instruction::ASL(AddressingMode::Absolute)),
            0x1E => Some(Instruction::ASL(AddressingMode::AbsoluteX)),

            // BCC -- Branch on Carry Clear
            0x90 => Some(Instruction::BCC(AddressingMode::Relative)),

            // BCS -- Branch on Carry Set
            0xB0 => Some(Instruction::BCS(AddressingMode::Relative)),

            // BEQ -- Branch on Result Zero
            0xF0 => Some(Instruction::BEQ(AddressingMode::Relative)),

            // BIT -- Test Bits in Memory with Accumulator
            0x24 => Some(Instruction::BIT(AddressingMode::ZeroPage)),
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
            0xC5 => Some(Instruction::CMP(AddressingMode::ZeroPage)),
            0xD5 => Some(Instruction::CMP(AddressingMode::ZeroPageX)),
            0xCD => Some(Instruction::CMP(AddressingMode::Absolute)),
            0xDD => Some(Instruction::CMP(AddressingMode::AbsoluteX)),
            0xD9 => Some(Instruction::CMP(AddressingMode::AbsoluteY)),
            0xC1 => Some(Instruction::CMP(AddressingMode::IndirectX)),
            0xD1 => Some(Instruction::CMP(AddressingMode::IndirectY)),

            // CPX -- Compare Memory and Index X
            0xE0 => Some(Instruction::CPX(AddressingMode::Immediate)),
            0xE4 => Some(Instruction::CPX(AddressingMode::ZeroPage)),
            0xEC => Some(Instruction::CPX(AddressingMode::Absolute)),

            // CPY -- Compare Memory and Index Y
            0xC0 => Some(Instruction::CPY(AddressingMode::Immediate)),
            0xC4 => Some(Instruction::CPY(AddressingMode::ZeroPage)),
            0xCC => Some(Instruction::CPY(AddressingMode::Absolute)),

            // DEC -- Decrement Memory By One
            0xC6 => Some(Instruction::DEC(AddressingMode::ZeroPage)),
            0xD6 => Some(Instruction::DEC(AddressingMode::ZeroPageX)),
            0xCE => Some(Instruction::DEC(AddressingMode::Absolute)),
            0xDE => Some(Instruction::DEC(AddressingMode::AbsoluteX)),

            // DEX -- Decrement Index X by One
            0xCA => Some(Instruction::DEX(AddressingMode::Implied)),

            // DEY -- Decrement Index Y by One
            0x88 => Some(Instruction::DEY(AddressingMode::Implied)),

            // EOR -- Exclusive-OR Memory with Accumulator
            0x49 => Some(Instruction::EOR(AddressingMode::Immediate)),
            0x45 => Some(Instruction::EOR(AddressingMode::ZeroPage)),
            0x55 => Some(Instruction::EOR(AddressingMode::ZeroPageX)),
            0x4D => Some(Instruction::EOR(AddressingMode::Absolute)),
            0x5D => Some(Instruction::EOR(AddressingMode::AbsoluteX)),
            0x59 => Some(Instruction::EOR(AddressingMode::AbsoluteY)),
            0x41 => Some(Instruction::EOR(AddressingMode::IndirectX)),
            0x51 => Some(Instruction::EOR(AddressingMode::IndirectY)),

            // INC -- Increment Memory By One
            0xE6 => Some(Instruction::INC(AddressingMode::ZeroPage)),
            0xF6 => Some(Instruction::INC(AddressingMode::ZeroPageX)),
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
            0xA5 => Some(Instruction::LDA(AddressingMode::ZeroPage)),
            0xB5 => Some(Instruction::LDA(AddressingMode::ZeroPageX)),
            0xAD => Some(Instruction::LDA(AddressingMode::Absolute)),
            0xBD => Some(Instruction::LDA(AddressingMode::AbsoluteX)),
            0xB9 => Some(Instruction::LDA(AddressingMode::AbsoluteY)),
            0xA1 => Some(Instruction::LDA(AddressingMode::IndirectX)),
            0xB1 => Some(Instruction::LDA(AddressingMode::IndirectY)),

            // LDX -- Load Index X with Memory
            0xA2 => Some(Instruction::LDX(AddressingMode::Immediate)),
            0xA6 => Some(Instruction::LDX(AddressingMode::ZeroPage)),
            0xB6 => Some(Instruction::LDX(AddressingMode::ZeroPageX)),
            0xAE => Some(Instruction::LDX(AddressingMode::Absolute)),
            0xBE => Some(Instruction::LDX(AddressingMode::AbsoluteX)),

            // LDY -- Load Index Y with Memory
            0xA0 => Some(Instruction::LDY(AddressingMode::Immediate)),
            0xA4 => Some(Instruction::LDY(AddressingMode::ZeroPage)),
            0xB4 => Some(Instruction::LDY(AddressingMode::ZeroPageX)),
            0xAC => Some(Instruction::LDY(AddressingMode::Absolute)),
            0xBC => Some(Instruction::LDY(AddressingMode::AbsoluteX)),

            // LSR -- Shift One Bit Right (Memory or Accumulator)
            0x4A => Some(Instruction::LSR(AddressingMode::Accumulator)),
            0x46 => Some(Instruction::LSR(AddressingMode::ZeroPage)),
            0x56 => Some(Instruction::LSR(AddressingMode::ZeroPageX)),
            0x4E => Some(Instruction::LSR(AddressingMode::Absolute)),
            0x5E => Some(Instruction::LSR(AddressingMode::AbsoluteX)),

            // NOP -- No Operation
            0xEA => Some(Instruction::NOP(AddressingMode::Implied)),

            // ORA -- OR Memory with Accumulator
            0x09 => Some(Instruction::ORA(AddressingMode::Immediate)),
            0x05 => Some(Instruction::ORA(AddressingMode::ZeroPage)),
            0x15 => Some(Instruction::ORA(AddressingMode::ZeroPageX)),
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
            0x26 => Some(Instruction::ROL(AddressingMode::ZeroPage)),
            0x36 => Some(Instruction::ROL(AddressingMode::ZeroPageX)),
            0x2E => Some(Instruction::ROL(AddressingMode::Absolute)),
            0x3E => Some(Instruction::ROL(AddressingMode::AbsoluteX)),

            // ROR -- Rotate One Bit Right
            0x6A => Some(Instruction::ROR(AddressingMode::Accumulator)),
            0x66 => Some(Instruction::ROR(AddressingMode::ZeroPage)),
            0x76 => Some(Instruction::ROR(AddressingMode::ZeroPageX)),
            0x6E => Some(Instruction::ROR(AddressingMode::Absolute)),
            0x7E => Some(Instruction::ROR(AddressingMode::AbsoluteX)),

            // RTI -- Return from Interrupt
            0x40 => Some(Instruction::RTI(AddressingMode::Implied)),

            // RTS -- Return from Subroutine
            0x60 => Some(Instruction::RTS(AddressingMode::Implied)),

            // SBC -- Subtract Memory from Accumulator with Borrow
            0xE9 => Some(Instruction::SBC(AddressingMode::Immediate)),
            0xE5 => Some(Instruction::SBC(AddressingMode::ZeroPage)),
            0xF5 => Some(Instruction::SBC(AddressingMode::ZeroPageX)),
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
            0x85 => Some(Instruction::STA(AddressingMode::ZeroPage)),
            0x95 => Some(Instruction::STA(AddressingMode::ZeroPageX)),
            0x8D => Some(Instruction::STA(AddressingMode::Absolute)),
            0x9D => Some(Instruction::STA(AddressingMode::AbsoluteX)),
            0x99 => Some(Instruction::STA(AddressingMode::AbsoluteY)),
            0x81 => Some(Instruction::STA(AddressingMode::IndirectX)),
            0x91 => Some(Instruction::STA(AddressingMode::IndirectY)),

            // STX -- Store Index X in Memory
            0x86 => Some(Instruction::STX(AddressingMode::ZeroPage)),
            0x96 => Some(Instruction::STX(AddressingMode::ZeroPageY)),
            0x8E => Some(Instruction::STX(AddressingMode::Absolute)),

            // STY -- Store Index Y in Memory
            0x84 => Some(Instruction::STY(AddressingMode::ZeroPage)),
            0x94 => Some(Instruction::STY(AddressingMode::ZeroPageY)),
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

    // TODO: add digit mode
    pub fn adc(accumulator: &mut u8, status: &mut Status, value: u8) {
        let carry_old = status.carry as u8;
        let (result, carry) = accumulator.wrapping_add(carry_old).overflowing_add(value);

        status.carry = carry;
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        status.overflow = (*accumulator > 127 && value > 127 && result < 128)
            || (*accumulator < 128 && carry_old < 128 && result > 127);

        *accumulator = result;
    }

    pub fn and(accumulator: &mut u8, status: &mut Status, value: u8) {
        *accumulator &= value;

        status.zero = *accumulator == 0;
        status.negative = (*accumulator & 0x80) != 0;
    }

    pub fn asl(status: &mut Status, mem_value: &mut u8) {
        status.carry = (*mem_value & 0x80) != 0;
        status.negative = (*mem_value & 0x40) != 0;
        status.zero = *mem_value == 0x80;

        *mem_value >>= 1;
    }

    pub fn branch(pc: &mut u16, condition: bool, value: u16) {
        if condition {
            *pc = pc.wrapping_add(value);
        }
    }

    pub fn bit(accumulator: &mut u8, status: &mut Status, value: u8) {
        status.zero = (*accumulator & value) == 0;
        status.overflow = (value & 0x40) != 0;
        status.negative = (value & 0x80) != 0;
    }

    pub fn brk(registers: &mut Registers, memory: &mut Memory) {
        registers.pc = registers.pc.wrapping_add(1);
        registers.push((registers.pc >> 8) as u8, memory);
        registers.push(registers.pc as u8, memory);
        registers.push(registers.status.to_binary(), memory);

        registers.status.interrupt = true;

        registers.pc = ((memory.get_byte(0xFFFF) as u16) << 8) | memory.get_byte(0xFFFE) as u16;
    }

    pub fn compare(status: &mut Status, value_lhs: u8, value_rhs: u8) {
        let result = value_lhs.wrapping_sub(value_rhs);
        status.zero = value_lhs == value_rhs;
        status.negative = result & 0x80 != 0;
        status.carry = value_lhs >= value_rhs;
    }

    pub fn decrement(status: &mut Status, mem_value: &mut u8) {
        let result = mem_value.wrapping_sub(1);
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        *mem_value = result;
    }

    pub fn eor(accumulator: &mut u8, status: &mut Status, value: u8) {
        *accumulator ^= value;
        status.zero = *accumulator == 0;
        status.negative = (*accumulator & 0x80) != 0
    }

    pub fn increment(status: &mut Status, mem_value: &mut u8) {
        let result = mem_value.wrapping_add(1);
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        *mem_value = result;
    }

    pub fn jsr(registers: &mut Registers, memory: &mut Memory, value: u16) {
        registers.pc = registers.pc.wrapping_sub(1);
        registers.push((registers.pc >> 8) as u8, memory);
        registers.push(registers.pc as u8, memory);
        registers.pc = value;
    }

    pub fn load(status: &mut Status, destination: &mut u8, value: u8) {
        status.zero = value == 0;
        status.negative = (value & 0x80) != 0;
        *destination = value;
    }

    pub fn lsr(status: &mut Status, mem_value: &mut u8) {
        status.carry = (*mem_value & 0x1) != 0;
        status.negative = false;
        *mem_value >>= 1;
        status.zero = *mem_value == 0;
    }

    pub fn ora(accumulator: &mut u8, status: &mut Status, value: u8) {
        *accumulator |= value;
        status.zero = *accumulator == 0;
        status.negative = (*accumulator & 0x80) != 0;
    }

    pub fn pla(registers: &mut Registers, memory: &Memory) {
        registers.accumulator = registers.pop(&memory);
        registers.status.zero = registers.accumulator == 0;
        registers.status.negative = (registers.accumulator & 0x80) != 0;
    }

    pub fn plp(registers: &mut Registers, memory: &Memory) {
        let value = registers.pop(memory);
        registers.status = Status::from_binary(value);
    }

    pub fn rol(status: &mut Status, mem_value: &mut u8) {
        status.carry = (*mem_value & 0x80) != 0;
        status.negative = (*mem_value & 0x40) != 0;
        *mem_value = mem_value.rotate_left(1);
        status.zero = *mem_value == 0;
    }

    pub fn ror(status: &mut Status, mem_value: &mut u8) {
        status.carry = (*mem_value & 0x80) != 0;
        status.negative = (*mem_value & 0x40) != 0;
        *mem_value = mem_value.rotate_right(1);
        status.zero = *mem_value == 0;
    }

    pub fn rti(registers: &mut Registers, memory: &Memory) {
        registers.status = Status::from_binary(registers.pop(memory));
        registers.pc = registers.pop(memory) as u16;
        registers.pc |= (registers.pop(memory) as u16) << 8;
    }

    pub fn rts(registers: &mut Registers, memory: &Memory) {
        registers.pc = registers.pop(memory) as u16;
        registers.pc |= (registers.pop(memory) as u16) << 8;
        registers.pc = registers.pc.wrapping_add(1);
    }

    //TODO: add digit mode
    pub fn sbc(accumulator: &mut u8, status: &mut Status, value: u8) {
        let not_carry = !status.carry as u8;
        let result = accumulator.wrapping_sub(value).wrapping_sub(not_carry);
        status.carry = result > *accumulator;
        status.overflow = (not_carry == 0 && value > 127) && *accumulator < 128 && result > 127
            || (*accumulator > 127) && (0u8.wrapping_sub(value).wrapping_sub(not_carry) > 127) && result < 128;
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        *accumulator = result;
    }

    pub fn transfer(status: &mut Status, value_lhs: u8, value_rhs: &mut u8) {
        status.zero = value_lhs == 0;
        status.negative = (value_lhs & 0x80) != 0;
        *value_rhs = value_lhs;
    }

    pub fn txs(registers: &mut Registers) {
        registers.sp = registers.x;
    }
}
