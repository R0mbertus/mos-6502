pub struct Registers {
    pub x: u8,                // x-register
    pub y: u8,                // y-register
    pub accumilator: u8,      // Accumulator register
    pub stack_pointer: u8,    // stack pointer
    pub program_counter: u16, // program counter
}

impl Default for Registers {
    fn default() -> Self {
        Registers::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            x: 0,
            y: 0,
            accumilator: 0,
            stack_pointer: 0,
            program_counter: 0,
        }
    }
}
