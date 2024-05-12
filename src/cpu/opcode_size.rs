#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpcodeSize {
    Byte,
    Word,
    Dword,
}

impl From<OpcodeSize> for u32 {
    fn from(value: OpcodeSize) -> Self {
        match value {
            OpcodeSize::Byte => 0x00,
            OpcodeSize::Word => 0x01,
            OpcodeSize::Dword => 0x02,
        }
    }
}

impl OpcodeSize {
    pub fn new(pattern: u32) -> Self {
        match pattern {
            0x00 => Self::Byte,
            0x01 => Self::Word,
            0x02 => Self::Dword,
            _ => unreachable!(),
        }
    }
}
