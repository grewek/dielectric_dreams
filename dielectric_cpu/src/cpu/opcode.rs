use std::fmt::Display;

use super::{addressing_modes::AddressingMode, opcode_size::OpcodeSize, register::Register};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Move(MoveOpcode),
    Unknown,
}

//TODO(Kay): The fields are public for now but later they shouldn't be accesible by the outside world!
#[derive(Debug, PartialEq, Eq)]
pub struct MoveOpcode {
    pub addr_mode: AddressingMode,
    pub destination: Register,
    pub source: Register,
    pub offset: u32,
    pub size: OpcodeSize,
}
