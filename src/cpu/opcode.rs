use super::{addressing_modes::AddressingMode, opcode_size::OpcodeSize};

const DECODER_OPCODE_MASK: u32 = 0x3FF;

const DECODER_DESTINATION_REGISTER_START: u32 = 10;
const DECODER_DESTINATION_REGISTER_MASK: u32 = 0x3F;

const DECODER_SOURCE_REGISTER_START: u32 = 16;
const DECODER_SOURCE_REGISTER_MASK: u32 = 0x3F;

const DECODER_OFFSET_START: u32 = 22;
const DECODER_OFFSET_MASK: u32 = 0x3F;

const DECODER_SIZE_START: u32 = 28;
const DECODER_SIZE_MASK: u32 = 0x03;

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
