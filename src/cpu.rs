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
    fn new(memory: Memory) -> Self {
        CPU {
            memory: memory,
            registers: Registers::default(),
            status: Status::default(),
        }
    }
}
