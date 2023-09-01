use crate::memory::Memory;

pub struct Status {
    pub negative: bool,
    pub overflow: bool,
    pub unused: bool,
    pub brk: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl Default for Status {
    fn default() -> Self {
        Status::new()
    }
}

impl Status {
    fn new() -> Self {
        Status {
            negative: false,
            overflow: false,
            unused: false,
            brk: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    pub fn to_binary(&self) -> u8 {
        (self.negative as u8) << 7
            | (self.overflow as u8) << 6
            | (self.unused as u8) << 5
            | (self.brk as u8) << 4
            | (self.decimal as u8) << 3
            | (self.interrupt as u8) << 2
            | (self.zero as u8) << 1
            | self.carry as u8
    }

    pub fn from_binary(status_binary: u8) -> Status {
        Status {
            negative: (status_binary & 0x80) != 0,
            overflow: (status_binary & 0x40) != 0,
            unused: (status_binary & 0x20) != 0,
            brk: (status_binary & 0x10) != 0,
            decimal: (status_binary & 0x8) != 0,
            interrupt: (status_binary & 0x4) != 0,
            zero: (status_binary & 0x2) != 0,
            carry: (status_binary & 0x1) != 0,
        }
    }
}

pub struct Registers {
    pub x: u8,           // x-register
    pub y: u8,           // y-register
    pub accumulator: u8, // Accumulator register
    pub sp: u8,          // stack pointer
    pub pc: u16,         // program counter
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
            accumulator: 0,
            sp: 0,
            pc: 0,
            status: Status::default(),
        }
    }

    pub fn push(&mut self, value: u8, memory: &mut Memory) {
        memory.write_byte(0x100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pop(&mut self, memory: &Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        memory.get_byte(0x100 + self.sp as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_binary() {
        let status = Status::new();
        assert_eq!(status.to_binary(), 0x30);
    }

    #[test]
    pub fn from_binary() {
        let status = Status::from_binary(0x80);
        assert_eq!(status.negative, true);
    }
}
