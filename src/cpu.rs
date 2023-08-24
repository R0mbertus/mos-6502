use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::registers::Registers;

pub struct CPU {
    memory: Memory,
    registers: Registers,
}

impl Default for CPU {
    fn default() -> Self {
        CPU::new(Memory::default())
    }
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        CPU {
            memory,
            registers: Registers::default(),
        }
    }

    pub fn decode_and_execute(&mut self) -> bool {
        let raw_opcode: u8 = self.memory.read_byte(self.registers.program_counter);
        let opcode = match Instruction::decode(raw_opcode) {
            Some(x) => x,
            None => panic!("Couldn't decode {raw_opcode} to valid opcode."),
        };

        let pc: u16 = self.registers.program_counter.wrapping_add(1);
        self.registers.program_counter = self
            .registers
            .program_counter
            .wrapping_add(opcode.addressing_mode().extra_bytes());

        match opcode {
            Instruction::ADC(am) => {
                dbg!("[EXECUTING]: add with carry, am: {am}");
                CPU::adc(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::AND(am) => {
                dbg!("[EXECUTING]: and operation, am: {am}");
                CPU::and(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::ASL(am) => {
                dbg!("[EXECUTING]: arithmetic shift left, am: {am}");
                CPU::asl(&mut self.registers, am.get_address_u8(&self.memory, pc));
            }
            Instruction::BCC(am) => {
                let condition = !self.registers.status.carry;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BCS(am) => {
                let condition = self.registers.status.carry;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BEQ(am) => {
                let condition = self.registers.status.zero;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BIT(am) => {
                CPU::bit(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::BMI(am) => {
                let condition = self.registers.status.negative;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BNE(am) => {
                let condition = !self.registers.status.zero;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BPL(am) => {
                let condition = !self.registers.status.negative;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BRK(_am) => {
                CPU::brk(&mut self.registers);
            }
            Instruction::BVC(am) => {
                let condition = !self.registers.status.overflow;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::BVS(am) => {
                let condition = self.registers.status.overflow;
                CPU::branch(
                    &mut self.registers,
                    condition,
                    am.get_data(&self.memory, pc),
                );
            }
            Instruction::CLC(_am) => {
                self.registers.status.carry = false;
            }
            Instruction::CLD(_am) => {
                self.registers.status.decimal = false;
            }
            Instruction::CLI(_am) => {
                self.registers.status.disable_interrupt = false;
            }
            Instruction::CLV(_am) => {
                self.registers.status.overflow = false;
            }
            Instruction::CMP(am) => {
                let value_lhs = self.registers.accumilator;
                CPU::compare(
                    &mut self.registers,
                    value_lhs,
                    am.get_data(&self.memory, pc) as u8,
                );
            }
            Instruction::CPX(am) => {
                let value_lhs = self.registers.x;
                CPU::compare(
                    &mut self.registers,
                    value_lhs,
                    am.get_data(&self.memory, pc) as u8,
                );
            }
            Instruction::CPY(am) => {
                let value_lhs = self.registers.y;
                CPU::compare(
                    &mut self.registers,
                    value_lhs,
                    am.get_data(&self.memory, pc) as u8,
                );
            }
            Instruction::DEC(am) => {
                unimplemented!()
            }
            Instruction::DEX(am) => {
                unimplemented!()
            }
            Instruction::DEY(am) => {
                unimplemented!()
            }
            Instruction::EOR(am) => {
                unimplemented!()
            }
            Instruction::INC(am) => {
                unimplemented!()
            }
            Instruction::INX(am) => {
                unimplemented!()
            }
            Instruction::INY(am) => {
                unimplemented!()
            }
            Instruction::JMP(am) => {
                unimplemented!()
            }
            Instruction::JSR(am) => {
                unimplemented!()
            }
            Instruction::LDA(am) => {
                unimplemented!()
            }
            Instruction::LDX(am) => {
                unimplemented!()
            }
            Instruction::LDY(am) => {
                unimplemented!()
            }
            Instruction::LSR(am) => {
                unimplemented!()
            }
            Instruction::NOP(_am) => {}
            Instruction::ORA(am) => {
                unimplemented!()
            }
            Instruction::PHA(am) => {
                unimplemented!()
            }
            Instruction::PHP(am) => {
                unimplemented!()
            }
            Instruction::PLA(am) => {
                unimplemented!()
            }
            Instruction::PLP(am) => {
                unimplemented!()
            }
            Instruction::ROL(am) => {
                unimplemented!()
            }
            Instruction::ROR(am) => {
                unimplemented!()
            }
            Instruction::RTI(am) => {
                unimplemented!()
            }
            Instruction::RTS(am) => {
                unimplemented!()
            }
            Instruction::SBC(am) => {
                unimplemented!()
            }
            Instruction::SEC(am) => {
                unimplemented!()
            }
            Instruction::SED(am) => {
                unimplemented!()
            }
            Instruction::SEI(am) => {
                unimplemented!()
            }
            Instruction::STA(am) => {
                unimplemented!()
            }
            Instruction::STX(am) => {
                unimplemented!()
            }
            Instruction::STY(am) => {
                unimplemented!()
            }
            Instruction::TAX(am) => {
                unimplemented!()
            }
            Instruction::TAY(am) => {
                unimplemented!()
            }
            Instruction::TSX(am) => {
                unimplemented!()
            }
            Instruction::TXA(am) => {
                unimplemented!()
            }
            Instruction::TXS(am) => {
                unimplemented!()
            }
            Instruction::TYA(am) => {
                unimplemented!()
            }
        }

        unimplemented!()
    }

    // TODO: add digit mode
    fn adc(registers: &mut Registers, value: u8) {
        let acc_old = registers.accumilator;
        let carry_old = registers.status.carry as u8;
        let (result, carry) = acc_old.wrapping_add(carry_old).overflowing_add(value);

        registers.status.carry = carry;
        registers.status.negative = (result & 0x80) != 0;
        registers.status.zero = result == 0;
        registers.status.overflow = (acc_old > 127 && value > 127 && result < 128)
            || (acc_old < 128 && carry_old < 128 && result > 127);

        registers.accumilator = result;
    }

    fn and(registers: &mut Registers, value: u8) {
        registers.accumilator &= value;

        registers.status.zero = registers.accumilator == 0;
        registers.status.negative = (registers.accumilator & 0x80) != 0;
    }

    fn asl(registers: &mut Registers, mem_value: &mut u8) {
        registers.status.carry = (*mem_value & 0x80) != 0;
        registers.status.negative = (*mem_value & 0x40) != 0;
        registers.status.zero = *mem_value == 0x80;

        *mem_value <<= 1;
    }

    fn branch(registers: &mut Registers, condition: bool, value: u16) {
        if condition {
            registers.program_counter = value;
        }
    }

    fn bit(registers: &mut Registers, value: u8) {
        registers.status.zero = registers.accumilator ^ value == 0;
        registers.status.overflow = (value & 0x40) != 0;
        registers.status.negative = (value & 0x80) != 0;
    }

    //TODO: implement stack
    fn brk(registers: &mut Registers) {
        unimplemented!();
    }

    fn compare(registers: &mut Registers, value_lhs: u8, value_rhs: u8) {
        let result = value_lhs - value_rhs;
        registers.status.zero = value_lhs == value_rhs;
        registers.status.negative = result & 0x80 != 0;
        registers.status.carry = value_lhs >= value_rhs;
    }
}
