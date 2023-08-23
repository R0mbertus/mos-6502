pub struct Status {
    pub carry: bool,
    pub zero: bool,
    pub disable_interrupt: bool,
    pub decimal: bool,
    pub brk: bool,
    pub unused: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Default for Status {
    fn default() -> Self {
        Status::new()
    }
}

impl Status {
    fn new() -> Self {
        Status {
            carry: false,
            zero: false,
            disable_interrupt: false,
            decimal: false,
            brk: false,
            unused: true,
            overflow: false,
            negative: false,
        }
    }
}

pub struct Registers {
    pub x: u8,                // x-register
    pub y: u8,                // y-register
    pub accumilator: u8,      // Accumulator register
    pub stack_pointer: u8,    // stack pointer
    pub program_counter: u16, // program counter
    pub status: Status,
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
            status: Status::default(),
        }
    }
}
