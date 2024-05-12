use super::{addressing_modes::AddressingMode, opcode_size::OpcodeSize};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Move(MoveOpcode),
}

//TODO(Kay): The fields are public for now but later they shouldn't be accesible by the outside world!
#[derive(Debug, PartialEq, Eq)]
pub struct MoveOpcode {
    pub destination: AddressingMode,
    pub source: AddressingMode,
    pub offset: u32,
    pub size: OpcodeSize,
}
