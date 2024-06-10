use super::{
    addressing_modes::AddressingMode,
    opcode_size::OpcodeSize,
    register::Register,
    status_register::{Flags, StatusRegister},
};
use crate::{Memory, RegisterFile};

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Move(MoveOpcode),
    Lea(LeaOpcode),
    Unknown,
}

impl Execute for Opcode {
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    ) {
        match self {
            Opcode::Move(data) => data.execute(pc, register_file, status_register, memory),
            Opcode::Lea(data) => data.execute(pc, register_file, status_register, memory),
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
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    ) {
        match self {
            MoveOpcode {
                addr_mode: AddressingMode::Atomic,
                destination,
                source,
                offset: _,
                size,
            } => {
                let raw_value: u32 = register_file.read_value(source);
                let data_to_write = size.retrieve_data(raw_value);

                register_file.write_value(destination, data_to_write);
            }
            MoveOpcode {
                addr_mode: AddressingMode::Immediate,
                destination,
                source: _,
                offset: _,
                size: _,
            } => {
                *pc += 4;
                let value = memory.read_dword(*pc);
                register_file.write_value(destination, value);
            }
            MoveOpcode {
                addr_mode: AddressingMode::MemoryDest,
                destination,
                source,
                offset: _,
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
                offset: _,
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
                offset: _,
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
                offset: _,
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
                offset: _,
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
                offset: _,
                size,
            } => {
                let address = register_file.read_value(source);
                let data_to_write = memory.memory_bus_read(size, address);

                register_file.write_value(destination, data_to_write);
                register_file.write_value(source, address - size.size_in_bytes());
            }
        }

        //NOTE: Update the register flags right after we have written the values
        let value = register_file.last_written_value();
        if value == 0x00 {
            status_register.raise(Flags::Zero);
        } else {
            status_register.clear(Flags::Zero);
        }

        let sign_mask: u32 = 1 << 31;
        if value & sign_mask == 0x80000000 {
            status_register.raise(Flags::Negative)
        } else {
            status_register.clear(Flags::Negative);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LeaOpcode {
    pub destination: Register,
}

impl Execute for LeaOpcode {
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    ) {
        if !(self.destination >= Register::A0 && self.destination <= Register::A15) {
            unreachable!("Lea opcode can only be used with the Registers A0 to A15")
        }

        let dest_index: u32 = self.destination.into();
        *pc += 4;

        let address = memory.read_dword(*pc);
        register_file.registers[dest_index as usize] = address;
    }
}

pub(crate) trait Execute {
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    );
}
