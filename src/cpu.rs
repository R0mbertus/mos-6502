use crate::memory::Memory;
use crate::instructions::{Instruction, AddressingMode};
use crate::registers::Registers;
use crate::status::Status;

pub struct CPU {
    memory: Memory,
    registers: Registers,
    status: Status,
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
            status: Status::default(),
        }
    }

    pub fn decode_and_execute(&mut self) -> bool {
        let raw_opcode: u8 = self.memory.read_byte(self.registers.program_counter);
        let opcode = match Instruction::decode(raw_opcode) {
            Some(x) => x,
            None => panic!("Couldn't decode {raw_opcode} to valid opcode.")
        };
    
        let pc: u16 = self.registers.program_counter.wrapping_add(1);
        self.registers.program_counter = 
            self.registers.program_counter.wrapping_add(opcode.addressing_mode().extra_bytes());
        
        match opcode {
            Instruction::ADC(am) => {
                dbg!("[EXECUTING]: add with carry, am: {am}");
                CPU::adc(am.get_data(&self.memory, pc) as u8);
            }
            _ => unimplemented!()
        }

        unimplemented!()
    }

    fn adc(mem_value: u8) {
        unimplemented!()
    }
}
