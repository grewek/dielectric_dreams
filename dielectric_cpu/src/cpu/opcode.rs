use std::fmt::Display;

use crate::{Memory, RegisterFile};

use super::{addressing_modes::AddressingMode, opcode_size::OpcodeSize, register::Register};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Move(MoveOpcode),
    Unknown,
}

impl Execute for Opcode {
    fn execute(&self, register_file: &mut RegisterFile, memory: &mut Memory) {
        match self {
            Opcode::Move(data) => data.execute(register_file, memory),
            Opcode::Unknown => todo!(),
        }
    }
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

impl Execute for MoveOpcode {
    fn execute(&self, register_file: &mut RegisterFile, memory: &mut Memory) {
        match self {
            MoveOpcode {
                addr_mode: AddressingMode::Atomic,
                destination,
                source,
                offset,
                size,
            } => {
                //TODO(Kay): Figure out if these should affect any flags, in most ISAs i know these
                //           do __not__ affect the flags in any way or form!
                let destination = *destination;
                let source = *source;
                let dest_index: u32 = destination.into();
                let source_index: u32 = source.into();
                let data_to_write =
                    size.retrieve_data(register_file.registers[source_index as usize]);

                register_file.registers[dest_index as usize] = data_to_write;
            }
            MoveOpcode {
                addr_mode: AddressingMode::Memory,
                destination,
                source,
                offset,
                size,
            } => {
                let destination = *destination;
                let source = *source;
                let dest_index: u32 = destination.into();
                let source_index: u32 = source.into();

                if source_index >= 0x10 && dest_index >= 0x10 {
                    unreachable!(
                        "illegal move instruction with two memory operands reached execution stage"
                    )
                }

                if source_index >= 0x10 {
                    let data_to_write = size.retrieve_data(
                        memory
                            .memory_bus_read(size, register_file.registers[source_index as usize]),
                    );

                    register_file.registers[dest_index as usize] = data_to_write;
                } else if dest_index >= 0x10 {
                    let target_address = register_file.registers[dest_index as usize];
                    let data_to_write =
                        size.retrieve_data(register_file.registers[source_index as usize]);
                    let command = size.memory_write_command(target_address, data_to_write);

                    memory.memory_bus_write(command);
                } else {
                    unreachable!("illegal move memory instruction with two data registers reached execution stage")
                }
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryInc,
                destination,
                source,
                offset,
                size,
            } => {
                let destination = *destination;
                let source = *source;
                let dest_index: u32 = destination.into();
                let source_index: u32 = source.into();

                let data_to_write = size.retrieve_data(
                    memory.memory_bus_read(size, register_file.registers[source_index as usize]),
                );

                register_file.registers[dest_index as usize] = data_to_write;
                register_file.registers[source_index as usize] += 1;
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryDec,
                destination,
                source,
                offset,
                size,
            } => todo!(),
        }
    }
}

pub(crate) trait Execute {
    fn execute(&self, register_file: &mut RegisterFile, memory: &mut Memory);
}
