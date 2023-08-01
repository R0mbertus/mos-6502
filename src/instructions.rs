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
    Indirect
}

impl AddressingMode {
    pub fn extra_bytes(&self) -> u16 {
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
            AddressingMode::Indirect => 2
        }
    }
}

pub struct Instruction {
    pub name: &'static str,
    pub addressing: AddressingMode
}

impl Instruction {
    // opcodes from https://www.masswerk.at/6502/6502_instruction_set.html#ASL
    pub fn get(code: u8) -> Self {
        match code {
            // ADC -- Add Memory to Accumulator with Carry
            0x69 => Instruction { name: "ADC", addressing: AddressingMode::Immediate },
            0x65 => Instruction { name: "ADC", addressing: AddressingMode::Zeropage },
            0x75 => Instruction { name: "ADC", addressing: AddressingMode::ZeropageX },
            0x6D => Instruction { name: "ADC", addressing: AddressingMode::Absolute },
            0x7D => Instruction { name: "ADC", addressing: AddressingMode::AbsoluteX },
            0x79 => Instruction { name: "ADC", addressing: AddressingMode::AbsoluteY },
            0x61 => Instruction { name: "ADC", addressing: AddressingMode::IndirectX },
            0x71 => Instruction { name: "ADC", addressing: AddressingMode::Indirect },

            // AND -- AND Memory with Accumulator
            0x29 => Instruction { name: "AND", addressing: AddressingMode::Immediate },
            0x25 => Instruction { name: "AND", addressing: AddressingMode::Zeropage },
            0x35 => Instruction { name: "AND", addressing: AddressingMode::ZeropageX },
            0x2D => Instruction { name: "AND", addressing: AddressingMode::Absolute },
            0x3D => Instruction { name: "AND", addressing: AddressingMode::AbsoluteX },
            0x39 => Instruction { name: "AND", addressing: AddressingMode::AbsoluteY },
            0x21 => Instruction { name: "AND", addressing: AddressingMode::IndirectX },
            0x31 => Instruction { name: "AND", addressing: AddressingMode::IndirectY },

            // ASL -- Shift Left One Bit With
            0x0A => Instruction { name: "ASL", addressing: AddressingMode::Accumulator },
            0x06 => Instruction { name: "ASL", addressing: AddressingMode::Zeropage },
            0x16 => Instruction { name: "ASL", addressing: AddressingMode::ZeropageX },
            0x0E => Instruction { name: "ASL", addressing: AddressingMode::Absolute },
            0x1E => Instruction { name: "ASL", addressing: AddressingMode::AbsoluteX },

            // BCC -- Branch on Carry Clear
            0x90 => Instruction { name: "BCC", addressing: AddressingMode::Relative },

            // BCS -- Branch on Carry Set
            0xB0 => Instruction { name: "BCS", addressing: AddressingMode::Relative },

            // BEQ -- Branch on Result Zero
            0xF0 => Instruction { name: "BEQ", addressing: AddressingMode::Relative },

            // BIT -- Test Bits in Memory with Accumulator
            0x24 => Instruction { name: "BIT", addressing: AddressingMode::Zeropage },
            0x2C => Instruction { name: "BIT", addressing: AddressingMode::Absolute },

            // BMI -- Branch on Result Minux
            0x30 => Instruction { name: "BMI", addressing: AddressingMode::Relative },

            // BNE -- Branch on Result not Zero
            0xD0 => Instruction { name: "BNE", addressing: AddressingMode::Relative },

            // BPL -- Branch on Result Plus
            0x10 => Instruction { name: "BPL", addressing: AddressingMode::Relative },

            // BRK -- Force Break
            0x00 => Instruction { name: "BRK", addressing: AddressingMode::Implied },

            // BVC -- Branch on Overflow Clear
            0x50 => Instruction { name: "BVC", addressing: AddressingMode::Relative },

            // BVS -- Branch on Overflow Set
            0x70 => Instruction { name: "BVS", addressing: AddressingMode::Relative },

            // CLC -- Clear Carry Flag
            0x18 => Instruction { name: "CLC", addressing: AddressingMode::Implied },
            
            // NOP -- NOP for all Non Defined Instructions
            _ => Instruction { name: "NOP", addressing: AddressingMode::Relative }
        }
    }
}
