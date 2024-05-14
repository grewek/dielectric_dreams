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

pub struct BitPattern {
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

impl From<BitPattern> for Opcode {
    fn from(value: BitPattern) -> Self {
        match value.opcode {
            0x01 => Opcode::Move(MoveOpcode {
                destination: addressing_mode(value.increment, value.dest_reg),
                source: addressing_mode(value.increment, value.src_reg),
                offset: value.offset,
                size: OpcodeSize::new(value.size),
            }),
            _ => unreachable!(),
        }
    }
}

pub fn addressing_mode(increment_mode: u32, register: u32) -> AddressingMode {
    let addr_mode_bits = register >> 4;

    let increment = increment_mode & 0x01 == 0x01;
    let decrement = increment_mode & 0x02 == 0x02;

    match addr_mode_bits {
        0x00..=0x01 => AddressingMode::Atomic(Register::new(register)),
        0x03 if increment => AddressingMode::MemoryInc(Register::new(register)),
        0x03 if decrement => AddressingMode::MemoryDec(Register::new(register)),
        //TODO(Kay): We should rethink the ISA things are getting fiddly already...
        0x03 => AddressingMode::Memory(Register::new(register)),
        _ => unreachable!(),
    }
}

pub fn decoder(pattern: u32) -> Opcode {
    let opcode = pattern & DECODER_OPCODE_MASK;
    let dest = (pattern >> DECODER_DESTINATION_REGISTER_START) & DECODER_DESTINATION_REGISTER_MASK;
    let src = (pattern >> DECODER_SOURCE_REGISTER_START) & DECODER_SOURCE_REGISTER_MASK;

    //let src_immediate_value = src & 0x20;

    let offset = (pattern >> DECODER_OFFSET_START) & DECODER_OFFSET_MASK;
    let size = (pattern >> DECODER_SIZE_START) & DECODER_SIZE_MASK;
    let increment = (pattern >> DECODER_INCREMENT_START) & DECODER_INCREMENT_MASK;

    match opcode {
        0x01 => Opcode::Move(MoveOpcode {
            destination: addressing_mode(increment, dest), //AddressingMode::Atomic(Register::new(dest)),
            source: addressing_mode(increment, src), //AddressingMode::Atomic(Register::new(src)),
            offset,
            size: OpcodeSize::new(size),
        }),
        _ => todo!(),
    }
}
