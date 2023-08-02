use crate::memory::Memory;
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

    pub fn step(&mut self) -> bool {
        let opcode: u8 = self.memory.read_byte(self.registers.program_counter);

        unimplemented!()
    }
}
