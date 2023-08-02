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
            AddressingMode::Indirect => 2,
        }
    }
}

pub struct Instruction {
    pub name: &'static str,
    pub addressing: AddressingMode,
}

impl Instruction {
    // opcodes from https://www.masswerk.at/6502/6502_instruction_set.html#ASL
    pub fn get(code: u8) -> Self {
        match code {
            // ADC -- Add Memory to Accumulator with Carry
            0x69 => Instruction {
                name: "ADC",
                addressing: AddressingMode::Immediate,
            },
            0x65 => Instruction {
                name: "ADC",
                addressing: AddressingMode::Zeropage,
            },
            0x75 => Instruction {
                name: "ADC",
                addressing: AddressingMode::ZeropageX,
            },
            0x6D => Instruction {
                name: "ADC",
                addressing: AddressingMode::Absolute,
            },
            0x7D => Instruction {
                name: "ADC",
                addressing: AddressingMode::AbsoluteX,
            },
            0x79 => Instruction {
                name: "ADC",
                addressing: AddressingMode::AbsoluteY,
            },
            0x61 => Instruction {
                name: "ADC",
                addressing: AddressingMode::IndirectX,
            },
            0x71 => Instruction {
                name: "ADC",
                addressing: AddressingMode::Indirect,
            },

            // AND -- AND Memory with Accumulator
            0x29 => Instruction {
                name: "AND",
                addressing: AddressingMode::Immediate,
            },
            0x25 => Instruction {
                name: "AND",
                addressing: AddressingMode::Zeropage,
            },
            0x35 => Instruction {
                name: "AND",
                addressing: AddressingMode::ZeropageX,
            },
            0x2D => Instruction {
                name: "AND",
                addressing: AddressingMode::Absolute,
            },
            0x3D => Instruction {
                name: "AND",
                addressing: AddressingMode::AbsoluteX,
            },
            0x39 => Instruction {
                name: "AND",
                addressing: AddressingMode::AbsoluteY,
            },
            0x21 => Instruction {
                name: "AND",
                addressing: AddressingMode::IndirectX,
            },
            0x31 => Instruction {
                name: "AND",
                addressing: AddressingMode::IndirectY,
            },

            // ASL -- Shift Left One Bit With
            0x0A => Instruction {
                name: "ASL",
                addressing: AddressingMode::Accumulator,
            },
            0x06 => Instruction {
                name: "ASL",
                addressing: AddressingMode::Zeropage,
            },
            0x16 => Instruction {
                name: "ASL",
                addressing: AddressingMode::ZeropageX,
            },
            0x0E => Instruction {
                name: "ASL",
                addressing: AddressingMode::Absolute,
            },
            0x1E => Instruction {
                name: "ASL",
                addressing: AddressingMode::AbsoluteX,
            },

            // BCC -- Branch on Carry Clear
            0x90 => Instruction {
                name: "BCC",
                addressing: AddressingMode::Relative,
            },

            // BCS -- Branch on Carry Set
            0xB0 => Instruction {
                name: "BCS",
                addressing: AddressingMode::Relative,
            },

            // BEQ -- Branch on Result Zero
            0xF0 => Instruction {
                name: "BEQ",
                addressing: AddressingMode::Relative,
            },

            // BIT -- Test Bits in Memory with Accumulator
            0x24 => Instruction {
                name: "BIT",
                addressing: AddressingMode::Zeropage,
            },
            0x2C => Instruction {
                name: "BIT",
                addressing: AddressingMode::Absolute,
            },

            // BMI -- Branch on Result Minux
            0x30 => Instruction {
                name: "BMI",
                addressing: AddressingMode::Relative,
            },

            // BNE -- Branch on Result not Zero
            0xD0 => Instruction {
                name: "BNE",
                addressing: AddressingMode::Relative,
            },

            // BPL -- Branch on Result Plus
            0x10 => Instruction {
                name: "BPL",
                addressing: AddressingMode::Relative,
            },

            // BRK -- Force Break
            0x00 => Instruction {
                name: "BRK",
                addressing: AddressingMode::Implied,
            },

            // BVC -- Branch on Overflow Clear
            0x50 => Instruction {
                name: "BVC",
                addressing: AddressingMode::Relative,
            },

            // BVS -- Branch on Overflow Set
            0x70 => Instruction {
                name: "BVS",
                addressing: AddressingMode::Relative,
            },

            // CLC -- Clear Carry Flag
            0x18 => Instruction {
                name: "CLC",
                addressing: AddressingMode::Implied,
            },

            // CLD -- Clear Decimal Mode
            0xD8 => Instruction {
                name: "CLD",
                addressing: AddressingMode::Implied,
            },

            // CLI -- Clear Interrupt Disable Bit
            0x58 => Instruction {
                name: "CLI",
                addressing: AddressingMode::Implied,
            },

            // CLV -- Clear Overflow Flag
            0xB8 => Instruction {
                name: "CLV",
                addressing: AddressingMode::Implied,
            },

            // CMP -- Compare Memory with Accumulator
            0xC9 => Instruction {
                name: "CMP",
                addressing: AddressingMode::Immediate,
            },
            0xC5 => Instruction {
                name: "CMP",
                addressing: AddressingMode::Zeropage,
            },
            0xD5 => Instruction {
                name: "CMP",
                addressing: AddressingMode::ZeropageX,
            },
            0xCD => Instruction {
                name: "CMP",
                addressing: AddressingMode::Absolute,
            },
            0xDD => Instruction {
                name: "CMP",
                addressing: AddressingMode::AbsoluteX,
            },
            0xD9 => Instruction {
                name: "CMP",
                addressing: AddressingMode::AbsoluteY,
            },
            0xC1 => Instruction {
                name: "CMP",
                addressing: AddressingMode::IndirectX,
            },
            0xD1 => Instruction {
                name: "CMP",
                addressing: AddressingMode::IndirectY,
            },

            // CPX -- Compare Memory and Index X
            0xE0 => Instruction {
                name: "CPX",
                addressing: AddressingMode::Immediate,
            },
            0xE4 => Instruction {
                name: "CPX",
                addressing: AddressingMode::Zeropage,
            },
            0xEC => Instruction {
                name: "CPX",
                addressing: AddressingMode::Absolute,
            },

            // CPY -- Compare Memory and Index Y
            0xC0 => Instruction {
                name: "CPY",
                addressing: AddressingMode::Immediate,
            },
            0xC4 => Instruction {
                name: "CPY",
                addressing: AddressingMode::Zeropage,
            },
            0xCC => Instruction {
                name: "CPY",
                addressing: AddressingMode::Absolute,
            },

            // DEC -- Decrement Memory By One
            0xC6 => Instruction {
                name: "DEC",
                addressing: AddressingMode::Zeropage,
            },
            0xD6 => Instruction {
                name: "DEC",
                addressing: AddressingMode::ZeropageX,
            },
            0xCE => Instruction {
                name: "DEC",
                addressing: AddressingMode::Absolute,
            },
            0xDE => Instruction {
                name: "DEC",
                addressing: AddressingMode::AbsoluteX,
            },

            // DEX -- Decrement Index X by One
            0xCA => Instruction {
                name: "DEX",
                addressing: AddressingMode::Implied,
            },

            // DEY -- Decrement Index Y by One
            0x88 => Instruction {
                name: "DEY",
                addressing: AddressingMode::Implied,
            },

            // EOR -- Exclusive-OR Memory with Accumulator
            0x49 => Instruction {
                name: "EOR",
                addressing: AddressingMode::Immediate,
            },
            0x45 => Instruction {
                name: "EOR",
                addressing: AddressingMode::Zeropage,
            },
            0x55 => Instruction {
                name: "EOR",
                addressing: AddressingMode::ZeropageX,
            },
            0x4D => Instruction {
                name: "EOR",
                addressing: AddressingMode::Absolute,
            },
            0x5D => Instruction {
                name: "EOR",
                addressing: AddressingMode::AbsoluteX,
            },
            0x59 => Instruction {
                name: "EOR",
                addressing: AddressingMode::AbsoluteY,
            },
            0x41 => Instruction {
                name: "EOR",
                addressing: AddressingMode::IndirectX,
            },
            0x51 => Instruction {
                name: "EOR",
                addressing: AddressingMode::IndirectY,
            },

            // INC -- Increment Memory By One
            0xE6 => Instruction {
                name: "INC",
                addressing: AddressingMode::Zeropage,
            },
            0xF6 => Instruction {
                name: "INC",
                addressing: AddressingMode::ZeropageX,
            },
            0xEE => Instruction {
                name: "INC",
                addressing: AddressingMode::Absolute,
            },
            0xFE => Instruction {
                name: "INC",
                addressing: AddressingMode::AbsoluteX,
            },

            // INX -- Increment Index X by One
            0xE8 => Instruction {
                name: "INX",
                addressing: AddressingMode::Implied,
            },

            // INY -- Increment Index Y by One
            0xC8 => Instruction {
                name: "INY",
                addressing: AddressingMode::Implied,
            },

            // JMP -- Jump to New Location
            0x4C => Instruction {
                name: "JMP",
                addressing: AddressingMode::Absolute,
            },
            0x6C => Instruction {
                name: "JMP",
                addressing: AddressingMode::Indirect,
            },

            // JSR -- Jump to New Location Saving Return Address
            0x20 => Instruction {
                name: "JSR",
                addressing: AddressingMode::Absolute,
            },

            // LDA -- Load Accumulator with Memory
            0xA9 => Instruction {
                name: "LDA",
                addressing: AddressingMode::Immediate,
            },
            0xA5 => Instruction {
                name: "LDA",
                addressing: AddressingMode::Zeropage,
            },
            0xB5 => Instruction {
                name: "LDA",
                addressing: AddressingMode::ZeropageX,
            },
            0xAD => Instruction {
                name: "LDA",
                addressing: AddressingMode::Absolute,
            },
            0xBD => Instruction {
                name: "LDA",
                addressing: AddressingMode::AbsoluteX,
            },
            0xB9 => Instruction {
                name: "LDA",
                addressing: AddressingMode::AbsoluteY,
            },
            0xA1 => Instruction {
                name: "LDA",
                addressing: AddressingMode::IndirectX,
            },
            0xB1 => Instruction {
                name: "LDA",
                addressing: AddressingMode::IndirectY,
            },

            // LDX -- Load Index X with Memory
            0xA2 => Instruction {
                name: "LDX",
                addressing: AddressingMode::Immediate,
            },
            0xA6 => Instruction {
                name: "LDX",
                addressing: AddressingMode::Zeropage,
            },
            0xB6 => Instruction {
                name: "LDX",
                addressing: AddressingMode::ZeropageX,
            },
            0xAE => Instruction {
                name: "LDX",
                addressing: AddressingMode::Absolute,
            },
            0xBE => Instruction {
                name: "LDX",
                addressing: AddressingMode::AbsoluteX,
            },

            // LDY -- Load Index Y with Memory
            0xA0 => Instruction {
                name: "LDY",
                addressing: AddressingMode::Immediate,
            },
            0xA4 => Instruction {
                name: "LDY",
                addressing: AddressingMode::Zeropage,
            },
            0xB4 => Instruction {
                name: "LDY",
                addressing: AddressingMode::ZeropageX,
            },
            0xAC => Instruction {
                name: "LDY",
                addressing: AddressingMode::Absolute,
            },
            0xBC => Instruction {
                name: "LDY",
                addressing: AddressingMode::AbsoluteX,
            },

            // LSR -- Shift One Bit Right (Memory or Accumulator)
            0x4A => Instruction {
                name: "LSR",
                addressing: AddressingMode::Accumulator,
            },
            0x46 => Instruction {
                name: "LSR",
                addressing: AddressingMode::Zeropage,
            },
            0x56 => Instruction {
                name: "LSR",
                addressing: AddressingMode::ZeropageX,
            },
            0x4E => Instruction {
                name: "LSR",
                addressing: AddressingMode::Absolute,
            },
            0x5E => Instruction {
                name: "LSR",
                addressing: AddressingMode::AbsoluteX,
            },

            // NOP -- No Operation
            0xEA => Instruction {
                name: "NOP",
                addressing: AddressingMode::Relative,
            },

            // ORA -- OR Memory with Accumulator
            0x09 => Instruction {
                name: "ORA",
                addressing: AddressingMode::Immediate,
            },
            0x05 => Instruction {
                name: "ORA",
                addressing: AddressingMode::Zeropage,
            },
            0x15 => Instruction {
                name: "ORA",
                addressing: AddressingMode::ZeropageX,
            },
            0x0D => Instruction {
                name: "ORA",
                addressing: AddressingMode::Absolute,
            },
            0x1D => Instruction {
                name: "ORA",
                addressing: AddressingMode::AbsoluteX,
            },
            0x19 => Instruction {
                name: "ORA",
                addressing: AddressingMode::AbsoluteY,
            },
            0x01 => Instruction {
                name: "ORA",
                addressing: AddressingMode::IndirectX,
            },
            0x11 => Instruction {
                name: "ORA",
                addressing: AddressingMode::IndirectY,
            },

            // PHA -- Push Accumulator on Stack
            0x48 => Instruction {
                name: "PHA",
                addressing: AddressingMode::Implied,
            },

            // PHP -- Push Processor Status on Stack
            0x08 => Instruction {
                name: "PHP",
                addressing: AddressingMode::Implied,
            },

            // PLA -- Pull Accumulator from Stack
            0x68 => Instruction {
                name: "PLA",
                addressing: AddressingMode::Implied,
            },

            // PLP -- Pull Processor Status from Stack
            0x28 => Instruction {
                name: "PLP",
                addressing: AddressingMode::Implied,
            },

            // ROL -- Rotate One Bit Left
            0x2A => Instruction {
                name: "ROL",
                addressing: AddressingMode::Accumulator,
            },
            0x26 => Instruction {
                name: "ROL",
                addressing: AddressingMode::Zeropage,
            },
            0x36 => Instruction {
                name: "ROL",
                addressing: AddressingMode::ZeropageX,
            },
            0x2E => Instruction {
                name: "ROL",
                addressing: AddressingMode::Absolute,
            },
            0x3E => Instruction {
                name: "ROL",
                addressing: AddressingMode::AbsoluteX,
            },

            // ROR -- Rotate One Bit Right
            0x6A => Instruction {
                name: "ROR",
                addressing: AddressingMode::Accumulator,
            },
            0x66 => Instruction {
                name: "ROR",
                addressing: AddressingMode::Zeropage,
            },
            0x76 => Instruction {
                name: "ROR",
                addressing: AddressingMode::ZeropageX,
            },
            0x6E => Instruction {
                name: "ROR",
                addressing: AddressingMode::Absolute,
            },
            0x7E => Instruction {
                name: "ROR",
                addressing: AddressingMode::AbsoluteX,
            },

            // RTI -- Return from Interrupt
            0x40 => Instruction {
                name: "RTI",
                addressing: AddressingMode::Implied,
            },

            // RTS -- Return from Subroutine
            0x60 => Instruction {
                name: "RTS",
                addressing: AddressingMode::Implied,
            },

            // SBC -- Subtract Memory from Accumulator with Borrow
            0xE9 => Instruction {
                name: "SBC",
                addressing: AddressingMode::Immediate,
            },
            0xE5 => Instruction {
                name: "SBC",
                addressing: AddressingMode::Zeropage,
            },
            0xF5 => Instruction {
                name: "SBC",
                addressing: AddressingMode::ZeropageX,
            },
            0xED => Instruction {
                name: "SBC",
                addressing: AddressingMode::Absolute,
            },
            0xFD => Instruction {
                name: "SBC",
                addressing: AddressingMode::AbsoluteX,
            },
            0xF9 => Instruction {
                name: "SBC",
                addressing: AddressingMode::AbsoluteY,
            },
            0xE1 => Instruction {
                name: "SBC",
                addressing: AddressingMode::IndirectX,
            },
            0xF1 => Instruction {
                name: "SBC",
                addressing: AddressingMode::IndirectY,
            },

            // SEC -- Set Carry Flag
            0x38 => Instruction {
                name: "SEC",
                addressing: AddressingMode::Implied,
            },

            // SED -- Set Decimal Flag
            0xF8 => Instruction {
                name: "SED",
                addressing: AddressingMode::Implied,
            },

            // SEI -- Set Interrupt Disable Status
            0x78 => Instruction {
                name: "SEI",
                addressing: AddressingMode::Implied,
            },

            // STA -- Subtract Memory from Accumulator with Borrow
            0x85 => Instruction {
                name: "STA",
                addressing: AddressingMode::Zeropage,
            },
            0x95 => Instruction {
                name: "STA",
                addressing: AddressingMode::ZeropageX,
            },
            0x8D => Instruction {
                name: "STA",
                addressing: AddressingMode::Absolute,
            },
            0x9D => Instruction {
                name: "STA",
                addressing: AddressingMode::AbsoluteX,
            },
            0x99 => Instruction {
                name: "STA",
                addressing: AddressingMode::AbsoluteY,
            },
            0x81 => Instruction {
                name: "STA",
                addressing: AddressingMode::IndirectX,
            },
            0x91 => Instruction {
                name: "STA",
                addressing: AddressingMode::IndirectY,
            },

            // STX -- Store Index X in Memory
            0x86 => Instruction {
                name: "STX",
                addressing: AddressingMode::Zeropage,
            },
            0x96 => Instruction {
                name: "STX",
                addressing: AddressingMode::ZeropageY,
            },
            0x8E => Instruction {
                name: "STX",
                addressing: AddressingMode::Absolute,
            },

            // STY -- Store Index Y in Memory
            0x84 => Instruction {
                name: "STY",
                addressing: AddressingMode::Zeropage,
            },
            0x94 => Instruction {
                name: "STY",
                addressing: AddressingMode::ZeropageY,
            },
            0x8C => Instruction {
                name: "STY",
                addressing: AddressingMode::Absolute,
            },

            // TAX -- Transfer Accumulator to Index X
            0xAA => Instruction {
                name: "TAX",
                addressing: AddressingMode::Implied,
            },

            // TAY -- Transfer Accumulator to Index Y
            0xA8 => Instruction {
                name: "TAY",
                addressing: AddressingMode::Implied,
            },

            // TSX -- Transfer Stack Pointer to Index X
            0xBA => Instruction {
                name: "TSX",
                addressing: AddressingMode::Implied,
            },

            // TXA -- Transfer Index X to Accumulator
            0x8A => Instruction {
                name: "TXA",
                addressing: AddressingMode::Implied,
            },

            // TXS -- Transfer Index X to Stack Register
            0x9A => Instruction {
                name: "TXS",
                addressing: AddressingMode::Implied,
            },

            // TYA -- Transfer Index Y to Accumulator
            0x98 => Instruction {
                name: "TYA",
                addressing: AddressingMode::Implied,
            },

            // NOP -- NOP for all Non Documented Instructions, for now...
            _ => Instruction {
                name: "NOP",
                addressing: AddressingMode::Relative,
            },
        }
    }
}
