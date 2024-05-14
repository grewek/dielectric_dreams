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

//TODO(Kay): If i really want to put this on a FPGA later i should make sure this fever dream is working on real electronic circuits...but i don't have a degree in electronics... :D
//TODO(Kay): We leave the flags out for now as it is still to be determined how that should work :)
//TODO(Kay): We will take a bit of inspiration from the good old MOTOROLA 68K :)
//TODO(Kay): It might be that i misjudged the amount of opcodes i need!

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: Memory::new(),
            status_register: 0,
            ip: 0,
        }
    }

    /*pub fn addressing_mode(&self, increment_mode: u32, register: u32) -> AddressingMode {
        let addr_mode_bits = register >> 4;

        let increment = increment_mode & 0x01 == 0x01;
        let decrement = increment_mode & 0x02 == 0x02;

        match addr_mode_bits {
            0x00..=0x01 => AddressingMode::Atomic(Register::decode(register)),
            0x03 if increment => AddressingMode::MemoryInc(Register::decode(register)),
            0x03 if decrement => AddressingMode::MemoryDec(Register::decode(register)),
            //TODO(Kay): We should rethink the ISA things are getting fiddly already...
            0x03 => AddressingMode::Memory(Register::decode(register)),
            _ => unreachable!(),
        }
    }*/

    pub fn decoder(&self, to_decode: u32) -> Opcode {
        let raw_opcode = BitPattern::new(to_decode);
        raw_opcode.into()
        /*let opcode = to_decode & DECODER_OPCODE_MASK;
        let dest =
            (to_decode >> DECODER_DESTINATION_REGISTER_START) & DECODER_DESTINATION_REGISTER_MASK;
        let src = (to_decode >> DECODER_SOURCE_REGISTER_START) & DECODER_SOURCE_REGISTER_MASK;

        //let src_immediate_value = src & 0x20;

        let offset = (to_decode >> DECODER_OFFSET_START) & DECODER_OFFSET_MASK;
        let size = (to_decode >> DECODER_SIZE_START) & DECODER_SIZE_MASK;
        let increment = (to_decode >> DECODER_INCREMENT_START) & DECODER_INCREMENT_MASK;

        match opcode {
            0x01 => Opcode::Move(MoveOpcode {
                destination: self.addressing_mode(increment, dest), //AddressingMode::Atomic(Register::new(dest)),
                source: self.addressing_mode(increment, src), //AddressingMode::Atomic(Register::new(src)),
                offset,
                size: OpcodeSize::new(size),
            }),
            _ => todo!(),
        }*/
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
            destination: AddressingMode::Atomic(dest),
            source: AddressingMode::Atomic(src),
            offset: 0,
            size,
        })
    }

    fn simple_memory_move_expect(dest: Register, src: Register, size: OpcodeSize) -> Opcode {
        Opcode::Move(MoveOpcode {
            destination: AddressingMode::Atomic(dest),
            source: AddressingMode::Memory(src),
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
