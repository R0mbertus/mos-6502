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
                CPU::bcc(&mut self.registers, am.get_data(&self.memory, pc));
            }
            _ => unimplemented!(),
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

    fn bcc(registers: &mut Registers, value: u16) {
        if registers.status.carry {
            registers.program_counter = value;
        }
    }
}
