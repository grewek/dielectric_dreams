use super::{addressing_modes::AddressingMode, opcode_size::OpcodeSize, register::Register};
use crate::{Memory, RegisterFile};

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
                let raw_value: u32 = register_file.read_value(source);
                let data_to_write = size.retrieve_data(raw_value);

                register_file.write_value(destination, data_to_write);
            }
            MoveOpcode {
                addr_mode: AddressingMode::Immediate,
                destination,
                source,
                offset,
                size,
            } => {
                todo!()
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryDest,
                destination,
                source,
                offset,
                size,
            } => {
                let raw_value: u32 = register_file.read_value(source);

                let data_to_write = size.retrieve_data(raw_value);
                let command =
                    size.memory_write_command(register_file.read_value(destination), data_to_write);

                memory.memory_bus_write(command);
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemorySrc,
                destination,
                source,
                offset,
                size,
            } => {
                let address = register_file.read_value(source);
                let data_to_write = memory.memory_bus_read(size, address);

                register_file.write_value(destination, data_to_write);
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryDestInc,
                destination,
                source,
                offset,
                size,
            } => {
                let data_to_write = size.retrieve_data(register_file.read_value(source));
                let address = register_file.read_value(destination);
                let command = size.memory_write_command(address, data_to_write);

                memory.memory_bus_write(command);

                register_file.write_value(destination, address + size.size_in_bytes())
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryDestDec,
                destination,
                source,
                offset,
                size,
            } => {
                let data_to_write = size.retrieve_data(register_file.read_value(source));
                let address = register_file.read_value(destination);
                let command = size.memory_write_command(address, data_to_write);

                memory.memory_bus_write(command);

                register_file.write_value(destination, address - size.size_in_bytes());
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemorySrcInc,
                destination,
                source,
                offset,
                size,
            } => {
                let address = register_file.read_value(source);
                let data_to_write = memory.memory_bus_read(size, address);

                register_file.write_value(destination, data_to_write);
                register_file.write_value(source, address + size.size_in_bytes());
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemorySrcDec,
                destination,
                source,
                offset,
                size,
            } => {
                let address = register_file.read_value(source);
                let data_to_write = memory.memory_bus_read(size, address);

                register_file.write_value(destination, data_to_write);
                register_file.write_value(source, address - size.size_in_bytes());
            }
        }
    }
}

pub(crate) trait Execute {
    fn execute(&self, register_file: &mut RegisterFile, memory: &mut Memory);
}
