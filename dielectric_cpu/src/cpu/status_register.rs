pub(crate) enum Flags {
    Negative = 15,       //The value is negative
    Overflow = 14,       //A Over/Underflow occured while doing the operation
    Carry = 13,          //The value did not fit and it carried a one into the status register
    Parity = 12, //If the amount of Bits set in the last operation was even this bit is set if it is odd it will be off
    Zero = 11,   //The value was zero...
    Interrupt = 10, //Interrupt flag used to indicate that something important happend
    InterruptEnable = 9, //Interrupts are only handled if this flag is one!
    Trap = 8,    //Trapflag used for debugging
                 //...
}

pub(crate) struct StatusRegister {
    flags: u16,
}

impl StatusRegister {
    pub(crate) fn new() -> Self {
        Self { flags: 0x00 }
    }

    pub(crate) fn raise(&mut self, flag: Flags) {
        let target_flag = flag as usize;
        self.flags |= 1 << target_flag;
    }

    pub(crate) fn clear(&mut self, flag: Flags) {
        let target_flag = flag as usize;
        self.flags &= 1 << target_flag;
    }

    pub(crate) fn status_bits(&self) -> u16 {
        self.flags
    }
}

//TODO(Kay): Testing!
