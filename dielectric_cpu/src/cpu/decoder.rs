use std::fmt::{Binary, Display, LowerHex};

use super::{
    opcode::{MoveOpcode, Opcode},
    opcode_size::OpcodeSize,
    register::Register,
};

const DECODER_OPCODE_MASK: u32 = 0xFF;
const DECODER_ADDR_MODE_START: u32 = 8;
const DECODER_ADDR_MODE_MASK: u32 = 0x07;
const DECODER_DESTINATION_REGISTER_START: u32 = 14;
const DECODER_DESTINATION_REGISTER_MASK: u32 = 0x3F;
const DECODER_SOURCE_REGISTER_START: u32 = 19;
const DECODER_SOURCE_REGISTER_MASK: u32 = 0x3F;
const DECODER_OFFSET_START: u32 = 24;
const DECODER_OFFSET_MASK: u32 = 0x3F;
const DECODER_SIZE_START: u32 = 30;
const DECODER_SIZE_MASK: u32 = 0x03;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitPattern {
    pattern: u32,
    opcode: u32,
    addr_mode: u32,
    dest_reg: u32,
    src_reg: u32,
    offset: u32,
    size: u32,
}

impl BitPattern {
    pub fn new(pattern: u32) -> Self {
        let addr_mode = (pattern >> DECODER_ADDR_MODE_START) & DECODER_ADDR_MODE_MASK;
        let dest_reg =
            (pattern >> DECODER_DESTINATION_REGISTER_START) & DECODER_DESTINATION_REGISTER_MASK;
        let src_reg = (pattern >> DECODER_SOURCE_REGISTER_START) & DECODER_SOURCE_REGISTER_MASK;
        Self {
            pattern,
            opcode: pattern & DECODER_OPCODE_MASK,
            addr_mode,
            dest_reg,
            src_reg,
            offset: (pattern >> DECODER_OFFSET_START) & DECODER_OFFSET_MASK,
            size: (pattern >> DECODER_SIZE_START) & DECODER_SIZE_MASK,
        }
    }
}

impl Display for BitPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern)
    }
}

impl Binary for BitPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032b}", self.pattern)
    }
}

impl LowerHex for BitPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.pattern)
    }
}

impl From<BitPattern> for Opcode {
    fn from(value: BitPattern) -> Self {
        match value.opcode {
            0x01 => Opcode::Move(MoveOpcode {
                addr_mode: value.addr_mode.into(),
                destination: Register::new(value.dest_reg),
                source: Register::new(value.src_reg),
                offset: value.offset,
                size: OpcodeSize::new(value.size),
            }),

            _ => Opcode::Unknown,
        }
    }
}
