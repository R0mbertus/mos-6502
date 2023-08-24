use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::registers::{Registers, Status};

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
                CPU::adc(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::AND(am) => {
                CPU::and(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::ASL(am) => {
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
                CPU::decrement(&mut self.registers.status, am.get_address_u8(&self.memory, pc));
            }
            Instruction::DEX(_am) => {
                CPU::decrement(&mut self.registers.status, &mut self.registers.x);
            }
            Instruction::DEY(_am) => {
                CPU::decrement(&mut self.registers.status, &mut self.registers.y);
            }
            Instruction::EOR(am) => {
                CPU::eor(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::INC(am) => {
                CPU::increment(&mut self.registers.status, am.get_address_u8(&self.memory, pc));
            }
            Instruction::INX(_am) => {
                CPU::increment(&mut self.registers.status, &mut self.registers.x);
            }
            Instruction::INY(_am) => {
                CPU::increment(&mut self.registers.status, &mut self.registers.y);
            }
            Instruction::JMP(am) => {
                CPU::jmp(&mut self.registers, am.get_data(&self.memory, pc));
            }
            Instruction::JSR(am) => {
                CPU::jsr(&mut self.registers, am.get_data(&self.memory, pc));
            }
            Instruction::LDA(am) => {
                self.registers.accumilator = am.get_data(&self.memory, pc) as u8;
            }
            Instruction::LDX(am) => {
                self.registers.x = am.get_data(&self.memory, pc) as u8;
            }
            Instruction::LDY(am) => {
                self.registers.y = am.get_data(&self.memory, pc) as u8;
            }
            Instruction::LSR(am) => {
                CPU::lsr(&mut self.registers, am.get_address_u8(&self.memory, pc));
            }
            Instruction::NOP(_am) => {}
            Instruction::ORA(am) => {
                CPU::ora(&mut self.registers, am.get_data(&self.memory, pc) as u8);
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
                CPU::rol(&mut self.registers, am.get_address_u8(&self.memory, pc));
            }
            Instruction::ROR(am) => {
                CPU::ror(&mut self.registers, am.get_address_u8(&self.memory, pc));
            }
            Instruction::RTI(am) => {
                unimplemented!()
            }
            Instruction::RTS(am) => {
                unimplemented!()
            }
            Instruction::SBC(am) => {
                CPU::sbc(&mut self.registers, am.get_data(&self.memory, pc) as u8);
            }
            Instruction::SEC(_am) => {
                self.registers.status.carry = true;
            }
            Instruction::SED(_am) => {
                self.registers.status.decimal = true;
            }
            Instruction::SEI(_am) => {
                self.registers.status.disable_interrupt = true;
            }
            Instruction::STA(am) => {
                *am.get_address_u8(&self.memory, pc) = self.registers.accumilator;
            }
            Instruction::STX(am) => {
                *am.get_address_u8(&self.memory, pc) = self.registers.x;
            }
            Instruction::STY(am) => {
                *am.get_address_u8(&self.memory, pc) = self.registers.y;
            }
            Instruction::TAX(_am) => {
                CPU::transfer(&mut self.registers.status, self.registers.accumilator, &mut self.registers.x);
            }
            Instruction::TAY(_am) => {
                CPU::transfer(&mut self.registers.status, self.registers.accumilator, &mut self.registers.y);
            }
            Instruction::TSX(_am) => {
                CPU::transfer(&mut self.registers.status, self.registers.stack_pointer, &mut self.registers.x);
            }
            Instruction::TXA(_am) => {
                CPU::transfer(&mut self.registers.status, self.registers.x, &mut self.registers.accumilator);
            }
            Instruction::TXS(_am) => {
                CPU::txs(&mut self.registers);
            }
            Instruction::TYA(_am) => {
                CPU::transfer(&mut self.registers.status, self.registers.y, &mut self.registers.accumilator);
            }
        }

        unimplemented!()
    }

    // TODO: add digit mode
    fn adc(registers: &mut Registers, value: u8) {
        let carry_old = registers.status.carry as u8;
        let (result, carry) = registers.accumilator.wrapping_add(carry_old).overflowing_add(value);

        registers.status.carry = carry;
        registers.status.negative = (result & 0x80) != 0;
        registers.status.zero = result == 0;
        registers.status.overflow = (registers.accumilator > 127 && value > 127 && result < 128)
            || (registers.accumilator < 128 && carry_old < 128 && result > 127);

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

        *mem_value >>= 1;
    }

    fn branch(registers: &mut Registers, condition: bool, value: u16) {
        if condition {
            registers.program_counter = value;
        }
    }

    fn bit(registers: &mut Registers, value: u8) {
        registers.status.zero = (registers.accumilator & value) == 0;
        registers.status.overflow = (value & 0x40) != 0;
        registers.status.negative = (value & 0x80) != 0;
    }

    //TODO: implement stack
    fn brk(registers: &mut Registers) {
        unimplemented!();
    }

    fn compare(registers: &mut Registers, value_lhs: u8, value_rhs: u8) {
        let result = value_lhs.wrapping_sub(value_rhs);
        registers.status.zero = value_lhs == value_rhs;
        registers.status.negative = result & 0x80 != 0;
        registers.status.carry = value_lhs >= value_rhs;
    }

    fn decrement(status: &mut Status, mem_value: &mut u8) {
        let result = mem_value.wrapping_sub(1);
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        *mem_value = result;
    }

    fn eor(registers: &mut Registers, value: u8) {
        registers.accumilator ^= value;
        registers.status.zero = registers.accumilator == 0;
        registers.status.negative = (registers.accumilator & 0x80) != 0
    }

    fn increment(status: &mut Status, mem_value: &mut u8) {
        let result = mem_value.wrapping_add(1);
        status.negative = (result & 0x80) != 0;
        status.zero = result == 0;
        *mem_value = result;
    }

    fn jmp(registers: &mut Registers, value: u16) {
        registers.program_counter = value;
    }

    //TODO: implement stack
    fn jsr(registers: &mut Registers, value: u16) {
        unimplemented!()
    }

    fn lsr(registers: &mut Registers, mem_value: &mut u8) {
        registers.status.carry = (*mem_value & 0x1) != 0;
        registers.status.negative = false;
        *mem_value >>= 1;
        registers.status.zero = *mem_value == 0;
    }

    fn ora(registers: &mut Registers, value: u8) {
        registers.accumilator |= value;
        registers.status.zero = registers.accumilator == 0;
        registers.status.negative = (registers.accumilator & 0x80) != 0;
    }

    fn rol(registers: &mut Registers, mem_value: &mut u8) {
        registers.status.carry = (*mem_value & 0x80) != 0;
        registers.status.negative = (*mem_value & 0x40) != 0;
        *mem_value = mem_value.rotate_left(1);
        registers.status.zero = *mem_value == 0;
    }

    fn ror(registers: &mut Registers, mem_value: &mut u8) {
        registers.status.carry = (*mem_value & 0x80) != 0;
        registers.status.negative = (*mem_value & 0x40) != 0;
        *mem_value = mem_value.rotate_right(1);
        registers.status.zero = *mem_value == 0;
    }

    fn sbc(registers: &mut Registers, value: u8) {
        let not_carry = !registers.status.carry as u8;
        let result = registers.accumilator.wrapping_sub(value).wrapping_sub(not_carry);
        registers.status.carry = result > registers.accumilator;
        registers.status.overflow = (not_carry == 0 && value > 127) && registers.accumilator < 128 && result > 127
            || (registers.accumilator > 127) && (value > 127) && result < 128;
        registers.accumilator = result;
    }

    fn transfer(status: &mut Status, value_lhs: u8, value_rhs: &mut u8) {
        status.zero = value_lhs == 0;
        status.negative = (value_lhs & 0x80) != 0;
        *value_rhs = value_lhs;
    }

    fn txs(registers: &mut Registers) {
        registers.stack_pointer = registers.x;
    }
}
