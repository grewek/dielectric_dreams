use std::fmt::Display;

use crate::MemoryWrite;

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

    pub fn retrieve_data(&self, data: u32) -> u32 {
        match self {
            OpcodeSize::Byte => data & 0x000000FF,
            OpcodeSize::Word => data & 0x0000FFFF,
            OpcodeSize::Dword => data,
        }
    }

    pub fn size_in_bytes(&self) -> u32 {
        match self {
            OpcodeSize::Byte => 1,
            OpcodeSize::Word => 2,
            OpcodeSize::Dword => 4,
        }
    }

    pub(crate) fn memory_write_command(&self, address: u32, value: u32) -> MemoryWrite {
        match self {
            OpcodeSize::Byte => MemoryWrite::Byte {
                address,
                value: value as u8,
            },
            OpcodeSize::Word => MemoryWrite::Word {
                address,
                value: value as u16,
            },
            OpcodeSize::Dword => MemoryWrite::Dword { address, value },
        }
    }
}
