use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::registers::Registers;

pub struct CPU {
    pub memory: Memory,
    pub registers: Registers,
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

    pub fn step(&mut self) {
        if let Some(instruction) = self.fetch() {
            self.execute(instruction);
        }
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.fetch() {
            self.execute(instruction);
        }
    }

    fn fetch(&mut self) -> Option<Instruction> {
        Instruction::decode(self.memory.get_byte(self.registers.pc))
    }

    fn execute(&mut self, instruction: Instruction) {
        let index = instruction
            .addressing_mode()
            .get_index(&self.memory, &mut self.registers);

        match instruction {
            Instruction::ADC(_) => {
                Instruction::adc(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::AND(_) => {
                Instruction::and(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::ASL(_) => {
                Instruction::asl(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::BCC(_) => {
                let condition = !self.registers.status.carry;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BCS(_) => {
                let condition = self.registers.status.carry;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BEQ(_) => {
                let condition = self.registers.status.zero;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BIT(_) => {
                Instruction::bit(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::BMI(_) => {
                let condition = self.registers.status.negative;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BNE(_) => {
                let condition = !self.registers.status.zero;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BPL(_) => {
                let condition = !self.registers.status.negative;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BRK(_) => {
                Instruction::brk(&mut self.registers, &mut self.memory);
            }
            Instruction::BVC(_) => {
                let condition = !self.registers.status.overflow;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::BVS(_) => {
                let condition = self.registers.status.overflow;
                Instruction::branch(
                    &mut self.registers.pc,
                    condition,
                    index,
                );
            }
            Instruction::CLC(_) => {
                self.registers.status.carry = false;
            }
            Instruction::CLD(_) => {
                self.registers.status.decimal = false;
            }
            Instruction::CLI(_) => {
                self.registers.status.interrupt = false;
            }
            Instruction::CLV(_) => {
                self.registers.status.overflow = false;
            }
            Instruction::CMP(_) => {
                let value_lhs = self.registers.accumulator;
                Instruction::compare(
                    &mut self.registers.status,
                    value_lhs,
                    self.memory.get_byte(index),
                );
            }
            Instruction::CPX(_) => {
                let value_lhs = self.registers.x;
                Instruction::compare(
                    &mut self.registers.status,
                    value_lhs,
                    self.memory.get_byte(index),
                );
            }
            Instruction::CPY(_) => {
                let value_lhs = self.registers.y;
                Instruction::compare(
                    &mut self.registers.status,
                    value_lhs,
                    self.memory.get_byte(index),
                );
            }
            Instruction::DEC(_) => {
                Instruction::decrement(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::DEX(_) => {
                Instruction::decrement(&mut self.registers.status, &mut self.registers.x);
            }
            Instruction::DEY(_) => {
                Instruction::decrement(&mut self.registers.status, &mut self.registers.y);
            }
            Instruction::EOR(_) => {
                Instruction::eor(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::INC(_) => {
                Instruction::increment(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::INX(_) => {
                Instruction::increment(&mut self.registers.status, &mut self.registers.x);
            }
            Instruction::INY(_) => {
                Instruction::increment(&mut self.registers.status, &mut self.registers.y);
            }
            Instruction::JMP(_) => {
                self.registers.pc = index;
            }
            Instruction::JSR(_) => {
                Instruction::jsr(&mut self.registers, &mut self.memory, index);
            }
            Instruction::LDA(_) => {
                Instruction::load(
                    &mut self.registers.status,
                    &mut self.registers.accumulator,
                    self.memory.get_byte(index),
                );
            }
            Instruction::LDX(_) => {
                Instruction::load(
                    &mut self.registers.status,
                    &mut self.registers.x,
                    self.memory.get_byte(index),
                );
            }
            Instruction::LDY(_) => {
                Instruction::load(
                    &mut self.registers.status,
                    &mut self.registers.y,
                    self.memory.get_byte(index),
                );
            }
            Instruction::LSR(_) => {
                Instruction::lsr(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::NOP(_) => {}
            Instruction::ORA(_) => {
                Instruction::ora(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::PHA(_) => {
                self.registers
                    .push(self.registers.accumulator, &mut self.memory);
            }
            Instruction::PHP(_) => {
                self.registers
                    .push(self.registers.status.to_binary(), &mut self.memory);
            }
            Instruction::PLA(_) => {
                Instruction::pla(&mut self.registers, &self.memory);
            }
            Instruction::PLP(_) => {
                Instruction::plp(&mut self.registers, &self.memory);
            }
            Instruction::ROL(_) => {
                Instruction::rol(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::ROR(_) => {
                Instruction::ror(&mut self.registers.status, self.memory.get_byte_mut(index));
            }
            Instruction::RTI(_) => {
                Instruction::rti(&mut self.registers, &self.memory);
            }
            Instruction::RTS(_) => {
                Instruction::rts(&mut self.registers, &self.memory);
            }
            Instruction::SBC(_) => {
                Instruction::sbc(
                    &mut self.registers.accumulator,
                    &mut self.registers.status,
                    self.memory.get_byte(index),
                );
            }
            Instruction::SEC(_) => {
                self.registers.status.carry = true;
            }
            Instruction::SED(_) => {
                self.registers.status.decimal = true;
            }
            Instruction::SEI(_) => {
                self.registers.status.interrupt = true;
            }
            Instruction::STA(_) => {
                *self.memory.get_byte_mut(index) = self.registers.accumulator;
            }
            Instruction::STX(_) => {
                *self.memory.get_byte_mut(index) = self.registers.x;
            }
            Instruction::STY(_) => {
                *self.memory.get_byte_mut(index) = self.registers.y;
            }
            Instruction::TAX(_) => {
                Instruction::transfer(
                    &mut self.registers.status,
                    self.registers.accumulator,
                    &mut self.registers.x,
                );
            }
            Instruction::TAY(_) => {
                Instruction::transfer(
                    &mut self.registers.status,
                    self.registers.accumulator,
                    &mut self.registers.y,
                );
            }
            Instruction::TSX(_) => {
                Instruction::transfer(
                    &mut self.registers.status,
                    self.registers.sp,
                    &mut self.registers.x,
                );
            }
            Instruction::TXA(_) => {
                Instruction::transfer(
                    &mut self.registers.status,
                    self.registers.x,
                    &mut self.registers.accumulator,
                );
            }
            Instruction::TXS(_) => {
                Instruction::txs(&mut self.registers);
            }
            Instruction::TYA(_) => {
                Instruction::transfer(
                    &mut self.registers.status,
                    self.registers.y,
                    &mut self.registers.accumulator,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;

    fn load_bin(file_name: &str) -> Option<Vec<u8>> {
        // Load the binary file from disk
        match read(format!("tests/bin/{}.bin", file_name)) {
            Ok(data) => Some(data),
            Err(err) => {
                println!("Error reading {}.bin: {}", file_name, err);
                None
            }
        }
    }

    #[test]
    pub fn functional_test_no_decimal() {
        let program = load_bin("6502_functional_test").unwrap();
        let mut cpu = CPU::default();
        cpu.memory.write_bytes(0x0a, &program);
        cpu.registers.pc = 0x400;
        cpu.run();
    }
}
