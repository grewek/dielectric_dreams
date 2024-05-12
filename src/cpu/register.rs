use super::decoder::Decoder;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,

    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
}

impl From<Register> for u32 {
    fn from(value: Register) -> Self {
        match value {
            Register::D0 => 0x00,
            Register::D1 => 0x01,
            Register::D2 => 0x02,
            Register::D3 => 0x03,
            Register::D4 => 0x04,
            Register::D5 => 0x05,
            Register::D6 => 0x06,
            Register::D7 => 0x07,
            Register::D8 => 0x08,
            Register::D9 => 0x09,
            Register::D10 => 0x0A,
            Register::D11 => 0x0B,
            Register::D12 => 0x0C,
            Register::D13 => 0x0D,
            Register::D14 => 0x0E,
            Register::D15 => 0x0F,
            Register::A0 => 0x10,
            Register::A1 => 0x11,
            Register::A2 => 0x12,
            Register::A3 => 0x13,
            Register::A4 => 0x14,
            Register::A5 => 0x15,
            Register::A6 => 0x16,
            Register::A7 => 0x17,
            Register::A8 => 0x18,
            Register::A9 => 0x19,
            Register::A10 => 0x1A,
            Register::A11 => 0x1B,
            Register::A12 => 0x1C,
            Register::A13 => 0x1D,
            Register::A14 => 0x1E,
            Register::A15 => 0x1F,
        }
    }
}

impl Register {
    pub fn new(pattern: u32) -> Self {
        match pattern {
            0x00 => Self::D0,
            0x01 => Self::D1,
            0x02 => Self::D2,
            0x03 => Self::D3,
            0x04 => Self::D4,
            0x05 => Self::D5,
            0x06 => Self::D6,
            0x07 => Self::D7,
            0x08 => Self::D8,
            0x09 => Self::D9,
            0x0A => Self::D10,
            0x0B => Self::D11,
            0x0C => Self::D12,
            0x0D => Self::D13,
            0x0E => Self::D14,
            0x0F => Self::D15,
            0x10 => Self::A0,
            0x11 => Self::A1,
            0x12 => Self::A2,
            0x13 => Self::A3,
            0x14 => Self::A4,
            0x15 => Self::A5,
            0x16 => Self::A6,
            0x17 => Self::A7,
            0x18 => Self::A8,
            0x19 => Self::A9,
            0x1A => Self::A10,
            0x1B => Self::A11,
            0x1C => Self::A12,
            0x1D => Self::A13,
            0x1E => Self::A14,
            0x1F => Self::A15,
            _ => unreachable!(),
        }
    }
}

impl Decoder for Register {
    type DecodedItem = Register;

    fn decode(pattern: u32) -> Self::DecodedItem {
        //Take the first five bits of the pattern, these describe the used registers!
        //This should work for our dest and source register!
        let register_pattern = pattern & 0x1F;

        Register::new(register_pattern)
    }
}
