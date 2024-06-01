use super::opcode::Execute;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
    Atomic,
    Immediate,
    MemoryDest,
    MemorySrc,
    MemoryDestInc,
    MemorySrcInc,
    MemoryDestDec,
    MemorySrcDec,
}

impl From<AddressingMode> for u32 {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Atomic => 0x00,
            AddressingMode::Immediate => 0x01,
            AddressingMode::MemoryDest => 0x02,
            AddressingMode::MemorySrc => 0x03,
            AddressingMode::MemoryDestInc => 0x04,
            AddressingMode::MemorySrcInc => 0x05,
            AddressingMode::MemoryDestDec => 0x06,
            AddressingMode::MemorySrcDec => 0x07,
        }
    }
}

impl From<u32> for AddressingMode {
    fn from(value: u32) -> Self {
        match value {
            0x00 => AddressingMode::Atomic,
            0x01 => AddressingMode::Immediate,
            0x02 => AddressingMode::MemoryDest,
            0x03 => AddressingMode::MemorySrc,
            0x04 => AddressingMode::MemoryDestInc,
            0x05 => AddressingMode::MemorySrcInc,
            0x06 => AddressingMode::MemoryDestDec,
            0x07 => AddressingMode::MemorySrcDec,
            _ => unreachable!(),
        }
    }
}
