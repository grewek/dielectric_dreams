use std::fmt::Display;

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

impl Display for OpcodeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte => write!(f, "B"),
            Self::Word => write!(f, "W"),
            Self::Dword => write!(f, "DW"),
        }
    }
}

impl OpcodeSize {
    pub fn new(pattern: u32) -> Self {
        match pattern {
            0x00 => Self::Byte,
            0x01 => Self::Word,
            0x02 => Self::Dword,
            //TODO(Kay): We have a unused bit! :( :| What can we do ? Void size ?
            0x03 => Self::Dword,
            _ => unreachable!(),
        }
    }
}
