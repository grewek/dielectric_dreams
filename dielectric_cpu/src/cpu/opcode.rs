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
    Push(PushOpcode),
    Pop(PopOpcode),
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
            Opcode::Push(data) => data.execute(pc, register_file, status_register, memory),
            Opcode::Pop(data) => data.execute(pc, register_file, status_register, memory),
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
pub struct LeaOpcode {
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

#[derive(Debug, PartialEq, Eq)]
pub struct PushOpcode {
    addressing_mode: AddressingMode,
    source: Register,
    size: OpcodeSize,
}

impl Execute for PushOpcode {
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    ) {
        //TODO: For now we assume that SP is the A15 Register but the user should be able to move the sp?!
        let stack_pointer = Register::A15;

        match self {
            PushOpcode {
                addressing_mode: AddressingMode::Atomic,
                source,
                size,
            } => {
                let raw_value: u32 = register_file.read_value(source);
                let data_to_write = size.retrieve_data(raw_value);
                let write_command = size
                    .memory_write_command(register_file.read_value(&stack_pointer), data_to_write);

                memory.memory_bus_write(write_command);
            }
            PushOpcode {
                addressing_mode: AddressingMode::Immediate,
                source: _,
                size,
            } => {
                *pc += 4;
                let data_to_write = memory.memory_bus_read(size, *pc);

                let write_command = size
                    .memory_write_command(register_file.read_value(&stack_pointer), data_to_write);

                memory.memory_bus_write(write_command);
            }
            PushOpcode {
                addressing_mode: AddressingMode::MemoryDest,
                source,
                size,
            } => {
                todo!()
            }
            PushOpcode {
                addressing_mode: AddressingMode::MemoryDestInc,
                source,
                size,
            } => {
                todo!()
            }
            PushOpcode {
                addressing_mode: AddressingMode::MemoryDestDec,
                source,
                size,
            } => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PopOpcode {
    addressing_mode: AddressingMode,
    destination: Register,
    size: OpcodeSize,
}

impl Execute for PopOpcode {
    fn execute(
        &self,
        pc: &mut u32,
        register_file: &mut RegisterFile,
        status_register: &mut StatusRegister,
        memory: &mut Memory,
    ) {
        match self {
            PopOpcode {
                addressing_mode: AddressingMode::Atomic,
                destination,
                size,
            } => {
                todo!()
            }
            PopOpcode {
                addressing_mode: AddressingMode::MemoryDest,
                destination,
                size,
            } => {
                todo!()
            }
            PopOpcode {
                addressing_mode: AddressingMode::MemoryDestInc,
                destination,
                size,
            } => {
                todo!()
            }
            PopOpcode {
                addressing_mode: AddressingMode::MemoryDestDec,
                destination,
                size,
            } => {
                todo!()
            }
            _ => unreachable!(),
        }
    }
}
