use super::addressing_modes::AddressingMode;
use super::decoder::BitPattern;
use super::opcode::MoveOpcode;
use super::opcode::Opcode;
use super::opcode_size::OpcodeSize;
use super::register::Register;
use crate::Memory;
use crate::RegisterFile;

pub struct Cpu {
    registers: RegisterFile,
    status_register: u32,
    ip: u32,
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: Memory::new(),
            status_register: 0,
            ip: 0,
        }
    }

    pub fn decoder(&self, to_decode: u32) -> Opcode {
        let raw_opcode = BitPattern::new(to_decode);
        raw_opcode.into()
    }
}

#[cfg(test)]
mod test {
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

    fn simple_move_expect(dest: Register, src: Register, size: OpcodeSize) -> Opcode {
        Opcode::Move(MoveOpcode {
            addr_mode: AddressingMode::Atomic,
            destination: dest,
            source: src,
            offset: 0,
            size,
        })
    }

    fn simple_memory_move_expect(dest: Register, src: Register, size: OpcodeSize) -> Opcode {
        Opcode::Move(MoveOpcode {
            addr_mode: AddressingMode::Memory,
            destination: dest,
            source: src,
            offset: 0,
            size,
        })
    }

    fn generate_memory_source(src: Register) -> u32 {
        let src_pattern: u32 = match src {
            Register::A0 => src.into(),
            Register::A1 => src.into(),
            Register::A2 => src.into(),
            Register::A3 => src.into(),
            Register::A4 => src.into(),
            Register::A5 => src.into(),
            Register::A6 => src.into(),
            Register::A7 => src.into(),
            Register::A8 => src.into(),
            Register::A9 => src.into(),
            Register::A10 => src.into(),
            Register::A11 => src.into(),
            Register::A12 => src.into(),
            Register::A13 => src.into(),
            Register::A14 => src.into(),
            Register::A15 => src.into(),
            _ => panic!("invalid register specified!"),
        };

        (1 << 5) | src_pattern
    }

    fn generate_opcode(dest: Register, src: Register, offset: u32, size: OpcodeSize) -> u32 {
        let size: u32 = size.into();
        let src: u32 = src.into();
        let dest: u32 = dest.into();
        let result = (size << 28) | (offset << 22) | (src << 16) | (dest << 10) | 0x01;
        dbg!(result);
        result
    }

    fn generate_memory_move_opcode(
        dest: Register,
        src: Register,
        offset: u32,
        size: OpcodeSize,
    ) -> u32 {
        let size: u32 = size.into();
        let src: u32 = generate_memory_source(src);
        let dest: u32 = dest.into();
        let result = (size << 28) | (offset << 22) | (src << 16) | (dest << 10) | 0x01;
        dbg!(result);
        result
    }

    //NOTE: This test covers all __possible__ combinations of 32bit registers!
    //      there shouldn't be any cases that contain undefined behaviour
    #[test]
    fn test_move_dword_registers() {
        for dest in &ALL_REGISTERS {
            for src in &ALL_REGISTERS {
                let opcode = generate_opcode(*dest, *src, 0, OpcodeSize::Dword);

                let result = get_decoder_result(opcode);
                let expected = simple_move_expect(*dest, *src, OpcodeSize::Dword);
                assert_eq!(result, expected, "Failed {:?} {:?}", result, expected)
            }
        }
    }

    #[test]
    fn test_move_memory_to_dword_registers() {
        //TODO(Kay): Add the rest of the registers as well!
        for address_register in &ADDRESS_REGISTERS {
            let opcode =
                generate_memory_move_opcode(Register::D0, *address_register, 0, OpcodeSize::Dword);
            let result = get_decoder_result(opcode);
            let expected =
                simple_memory_move_expect(Register::D0, *address_register, OpcodeSize::Dword);
            assert_eq!(result, expected, "Failed {:?}, {:?}", result, expected);
        }
    }
}
