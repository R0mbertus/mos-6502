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
