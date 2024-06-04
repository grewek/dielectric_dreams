use super::decoder::BitPattern;
use super::opcode::Opcode;
use crate::cpu::opcode::Execute;
use crate::Memory;
use crate::RegisterFile;

pub struct Cpu {
    register_file: RegisterFile,
    status_register: u32,
    ip: u32,
    memory: Memory,
}

//NOTE(Kay): Make clippy happy!
impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register_file: RegisterFile::new(),
            memory: Memory::new(),
            status_register: 0,
            ip: 0,
        }
    }

    pub fn decoder(&self, to_decode: u32) -> Opcode {
        let raw_opcode = BitPattern::new(to_decode);
        raw_opcode.into()
    }

    //TODO(Kay): Refactor to the Opcode enum!
    pub fn execution_stage(&mut self, opcode: Opcode) {
        opcode.execute(&mut self.register_file, &mut self.memory);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{
        addressing_modes::AddressingMode, opcode::MoveOpcode, opcode_size::OpcodeSize,
        register::Register,
    };

    use super::*;

    static ALL_REGISTERS: [Register; 32] = [
        Register::D0,
        Register::D1,
        Register::D2,
        Register::D3,
        Register::D4,
        Register::D5,
        Register::D6,
        Register::D7,
        Register::D8,
        Register::D9,
        Register::D10,
        Register::D11,
        Register::D12,
        Register::D13,
        Register::D14,
        Register::D15,
        Register::A0,
        Register::A1,
        Register::A2,
        Register::A3,
        Register::A4,
        Register::A5,
        Register::A6,
        Register::A7,
        Register::A8,
        Register::A9,
        Register::A10,
        Register::A11,
        Register::A12,
        Register::A13,
        Register::A14,
        Register::A15,
    ];

    static DATA_REGISTERS: [Register; 16] = [
        Register::D0,
        Register::D1,
        Register::D2,
        Register::D3,
        Register::D4,
        Register::D5,
        Register::D6,
        Register::D7,
        Register::D8,
        Register::D9,
        Register::D10,
        Register::D11,
        Register::D12,
        Register::D13,
        Register::D14,
        Register::D15,
    ];

    static ADDRESS_REGISTERS: [Register; 16] = [
        Register::A0,
        Register::A1,
        Register::A2,
        Register::A3,
        Register::A4,
        Register::A5,
        Register::A6,
        Register::A7,
        Register::A8,
        Register::A9,
        Register::A10,
        Register::A11,
        Register::A12,
        Register::A13,
        Register::A14,
        Register::A15,
    ];

    fn get_decoder_result(pattern: u32) -> Opcode {
        let cpu = Cpu::new();
        cpu.decoder(pattern)
    }

    fn simple_move_expect(
        mode: AddressingMode,
        dest: Register,
        src: Register,
        size: OpcodeSize,
    ) -> Opcode {
        Opcode::Move(MoveOpcode {
            addr_mode: mode,
            destination: dest,
            source: src,
            offset: 0,
            size,
        })
    }

    fn generate_opcode(
        mode: AddressingMode,
        dest: Register,
        src: Register,
        offset: u32,
        size: OpcodeSize,
    ) -> u32 {
        let size: u32 = size.into();
        let mode: u32 = mode.into();

        let src: u32 = src.into();

        let dest: u32 = dest.into();

        let result =
            (size << 30) | (offset << 24) | (src << 19) | (dest << 14) | (mode << 8) | 0x01;
        dbg!(result);
        result
    }

    //NOTE: This test covers all __possible__ combinations of 32bit registers!
    //      there shouldn't be any cases that contain undefined behaviour
    #[test]
    fn test_move_dword_registers() {
        for dest in &ALL_REGISTERS {
            for src in &ALL_REGISTERS {
                let opcode =
                    generate_opcode(AddressingMode::Atomic, *dest, *src, 0, OpcodeSize::Dword);

                let result = get_decoder_result(opcode);
                let expected =
                    simple_move_expect(AddressingMode::Atomic, *dest, *src, OpcodeSize::Dword);
                assert_eq!(result, expected, "Failed {:?} {:?}", result, expected)
            }
        }
    }

    #[test]
    fn test_move_memory_dword_execution() {
        for data_reg in DATA_REGISTERS {
            for src_reg in ADDRESS_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemorySrc,
                    data_reg,
                    src_reg,
                    0,
                    OpcodeSize::Dword,
                );
                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = data_reg.into();
                cpu.register_file.registers[src_index as usize] = 0x7000BA5;
                let opcode = cpu.decoder(opcode);

                cpu.memory.bytes[0x7000BA5] = 0xAA;
                cpu.memory.bytes[0x7000BA6] = 0xBB;
                cpu.memory.bytes[0x7000BA7] = 0xCC;
                cpu.memory.bytes[0x7000BA8] = 0xDD;
                cpu.execution_stage(opcode);

                assert_eq!(cpu.register_file.registers[dest_index as usize], 0xAABBCCDD);
            }
        }
    }

    #[test]
    fn test_move_memory_word_execution() {
        for data_reg in DATA_REGISTERS {
            for src_reg in ADDRESS_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemorySrc,
                    data_reg,
                    src_reg,
                    0,
                    OpcodeSize::Word,
                );
                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = data_reg.into();
                cpu.register_file.registers[src_index as usize] = 0x7000BA5;
                let opcode = cpu.decoder(opcode);

                cpu.memory.bytes[0x7000BA5] = 0xAA;
                cpu.memory.bytes[0x7000BA6] = 0xBB;
                cpu.memory.bytes[0x7000BA7] = 0xCC;
                cpu.memory.bytes[0x7000BA8] = 0xDD;
                cpu.execution_stage(opcode);

                assert_eq!(cpu.register_file.registers[dest_index as usize], 0x0000AABB);
            }
        }
    }

    #[test]
    fn test_move_memory_byte_execution() {
        for data_reg in DATA_REGISTERS {
            for src_reg in ADDRESS_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemorySrc,
                    data_reg,
                    src_reg,
                    0,
                    OpcodeSize::Byte,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = data_reg.into();
                cpu.register_file.registers[src_index as usize] = 0x7000BA5;
                let opcode = cpu.decoder(opcode);

                cpu.memory.bytes[0x7000BA5] = 0xAA;
                cpu.memory.bytes[0x7000BA6] = 0xBB;
                cpu.memory.bytes[0x7000BA7] = 0xCC;
                cpu.memory.bytes[0x7000BA8] = 0xDD;
                cpu.execution_stage(opcode);

                assert_eq!(cpu.register_file.registers[dest_index as usize], 0x000000AA);
            }
        }
    }

    #[test]
    fn test_move_dword_registers_execution() {
        for dest_reg in ALL_REGISTERS {
            for src_reg in ALL_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::Atomic,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Dword,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xAABBCCDD;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(
                    cpu.register_file.registers[dest_index as usize],
                    cpu.register_file.registers[src_index as usize]
                );
            }
        }
    }

    #[test]
    fn test_move_word_registers_execution() {
        for dest_reg in ALL_REGISTERS {
            for src_reg in ALL_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::Atomic,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Word,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xAABBCCDD;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(cpu.register_file.registers[dest_index as usize], 0x0000CCDD);
            }
        }
    }

    #[test]
    fn test_move_byte_registers_execution() {
        for dest_reg in ALL_REGISTERS {
            for src_reg in ALL_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::Atomic,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Byte,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xAABBCCDD;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(cpu.register_file.registers[dest_index as usize], 0x000000DD);
            }
        }
    }

    #[test]
    fn test_move_byte_register_into_memory_execution() {
        for dest_reg in ADDRESS_REGISTERS {
            for src_reg in DATA_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemoryDest,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Byte,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xDEADBEEF;
                cpu.register_file.registers[dest_index as usize] = 0x05403502;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(
                    cpu.memory.memory_bus_read(&OpcodeSize::Byte, 0x05403502),
                    0x000000EF
                );
            }
        }
    }

    #[test]
    fn test_move_word_register_into_memory_execution() {
        for dest_reg in ADDRESS_REGISTERS {
            for src_reg in DATA_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemoryDest,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Word,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xDEADBEEF;
                cpu.register_file.registers[dest_index as usize] = 0x05403502;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(
                    cpu.memory.memory_bus_read(&OpcodeSize::Word, 0x05403502),
                    0x0000BEEF
                );
            }
        }
    }

    #[test]
    fn test_move_dword_register_into_memory_execution() {
        for dest_reg in ADDRESS_REGISTERS {
            for src_reg in DATA_REGISTERS {
                let opcode = generate_opcode(
                    AddressingMode::MemoryDest,
                    dest_reg,
                    src_reg,
                    0,
                    OpcodeSize::Dword,
                );

                let mut cpu = Cpu::new();
                let src_index: u32 = src_reg.into();
                let dest_index: u32 = dest_reg.into();
                cpu.register_file.registers[src_index as usize] = 0xDEADBEEF;
                cpu.register_file.registers[dest_index as usize] = 0x05403502;
                let opcode = cpu.decoder(opcode);
                cpu.execution_stage(opcode);

                assert_eq!(
                    cpu.memory.memory_bus_read(&OpcodeSize::Dword, 0x05403502),
                    0xDEADBEEF
                );
            }
        }
    }

    #[test]
    fn test_move_byte_inc_register_into_memory_execution() {
        let opcode = generate_opcode(
            AddressingMode::MemorySrcInc,
            Register::D1,
            Register::A0,
            0,
            OpcodeSize::Byte,
        );
        let mut cpu = Cpu::new();

        cpu.memory.write_byte(0x05403502, 0xEF);
        cpu.register_file.registers[16] = 0x05403502;
        let opcode = cpu.decoder(opcode);
        cpu.execution_stage(opcode);

        assert_eq!(cpu.register_file.registers[1], 0x000000EF);
        assert_eq!(cpu.register_file.registers[16], 0x05403503)
    }

    #[test]
    fn test_move_word_inc_register_into_memory_execution() {
        let opcode = generate_opcode(
            AddressingMode::MemorySrcInc,
            Register::D1,
            Register::A0,
            0,
            OpcodeSize::Word,
        );
        let mut cpu = Cpu::new();

        cpu.memory.write_word(0x05403502, 0xBEEF);
        cpu.register_file.registers[16] = 0x05403502;
        let opcode = cpu.decoder(opcode);
        cpu.execution_stage(opcode);

        assert_eq!(cpu.register_file.registers[1], 0x0000BEEF);
        assert_eq!(cpu.register_file.registers[16], 0x05403504)
    }

    #[test]
    fn test_move_dword_inc_register_into_memory_execution() {
        let opcode = generate_opcode(
            AddressingMode::MemorySrcInc,
            Register::D1,
            Register::A0,
            0,
            OpcodeSize::Dword,
        );
        let mut cpu = Cpu::new();

        cpu.memory.write_dword(0x05403502, 0xDEADBEEF);
        cpu.register_file.registers[16] = 0x05403502;
        let opcode = cpu.decoder(opcode);
        cpu.execution_stage(opcode);

        assert_eq!(cpu.register_file.registers[1], 0xDEADBEEF);
        assert_eq!(cpu.register_file.registers[16], 0x05403506)
    }

    #[test]
    fn test_move_memory_to_dword_registers() {
        //TODO(Kay): Add the rest of the registers as well!
        for address_register in &ADDRESS_REGISTERS {
            let opcode = generate_opcode(
                AddressingMode::MemorySrc,
                Register::D0,
                *address_register,
                0,
                OpcodeSize::Dword,
            );
            let result = get_decoder_result(opcode);
            let expected = simple_move_expect(
                AddressingMode::MemorySrc,
                Register::D0,
                *address_register,
                OpcodeSize::Dword,
            );
            assert_eq!(result, expected, "Failed {:?}, {:?}", result, expected);
        }
    }
}
