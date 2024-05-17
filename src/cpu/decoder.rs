use std::fmt::{Binary, Display, LowerHex};

use super::{
    addressing_modes::AddressingMode,
    opcode::{MoveOpcode, Opcode},
    opcode_size::OpcodeSize,
    register::Register,
};

const DECODER_OPCODE_MASK: u32 = 0x3FF;
const DECODER_DESTINATION_REGISTER_START: u32 = 10;
const DECODER_DESTINATION_REGISTER_MASK: u32 = 0x3F;
const DECODER_SOURCE_REGISTER_START: u32 = 16;
const DECODER_SOURCE_REGISTER_MASK: u32 = 0x3F;
const DECODER_OFFSET_START: u32 = 22;
const DECODER_OFFSET_MASK: u32 = 0x3F;
const DECODER_SIZE_START: u32 = 28;
const DECODER_SIZE_MASK: u32 = 0x03;
const DECODER_INCREMENT_START: u32 = 30;
const DECODER_INCREMENT_MASK: u32 = 0x03;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitPattern {
    pattern: u32,
    opcode: u32,
    dest_reg: u32,
    src_reg: u32,
    offset: u32,
    size: u32,
    increment: u32,
}

impl BitPattern {
    pub fn new(pattern: u32) -> Self {
        Self {
            pattern,
            opcode: pattern & DECODER_OPCODE_MASK,
            dest_reg: (pattern >> DECODER_DESTINATION_REGISTER_START)
                & DECODER_DESTINATION_REGISTER_MASK,
            src_reg: (pattern >> DECODER_SOURCE_REGISTER_START) & DECODER_SOURCE_REGISTER_MASK,
            offset: (pattern >> DECODER_OFFSET_START) & DECODER_OFFSET_MASK,
            size: (pattern >> DECODER_SIZE_START) & DECODER_SIZE_MASK,
            increment: (pattern >> DECODER_INCREMENT_START) & DECODER_INCREMENT_MASK,
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
        let addr_mode_dest = value.dest_reg >> 4;
        let addr_mode_src = value.src_reg >> 4;

        //TODO(Kay): Rethink the ISA, doing it like this is probably a bad idea! Also we loose the
        //           we can determine the move type by looking at the used registers and determine
        //           which one is the Address Register!
        let best_fit = u32::max(addr_mode_dest, addr_mode_src);

        match value.opcode {
            0x01 => Opcode::Move(MoveOpcode {
                addr_mode: addressing_mode(value.increment, best_fit),
                destination: Register::new(value.dest_reg),
                source: Register::new(value.src_reg),
                offset: value.offset,
                size: OpcodeSize::new(value.size),
            }),

            _ => Opcode::Unknown,
        }
    }
}

pub fn addressing_mode(increment_mode: u32, register: u32) -> AddressingMode {
    let addr_mode_bits = register;

    let increment = increment_mode & 0x01 == 0x01;
    let decrement = increment_mode & 0x02 == 0x02;

    match addr_mode_bits {
        0x00..=0x02 => AddressingMode::Atomic,
        0x03 if increment => AddressingMode::MemoryInc,
        0x03 if decrement => AddressingMode::MemoryDec,
        0x03 => AddressingMode::Memory,
        //TODO(Kay): We should rethink the ISA things are getting fiddly already...
        _ => unreachable!(),
    }
}
