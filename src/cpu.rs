use crate::Memory;
use crate::MoveOpcode;
use crate::Opcode;
use crate::OpcodeSize;
use crate::Register;
use crate::RegisterFile;

pub struct Cpu {
    registers: RegisterFile,
    status_register: u32,
    ip: u32,
    memory: Memory,
}

//TODO(Kay): If i really want to put this on a FPGA later i should make sure this fever dream is working on real electronic circuits...but i don't have a degree in electronics... :D
//TODO(Kay): The Documentation should be moved into a seperate file...
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

    pub fn decoder(&self, to_decode: u32) -> Opcode {
        let opcode = to_decode & 0x3FF;
        let dest = (to_decode >> 10) & 0x3F;
        let src = (to_decode >> 16) & 0x3F;

        let src_immediate_value = src & 0x20;

        let offset = (to_decode >> 22) & 0x3F;
        let size = (to_decode >> 28) & 0xF;

        match opcode {
            0x01 => Opcode::Move(MoveOpcode {
                destination: Register::new(dest),
                source: Register::new(src),
                offset,
                size: OpcodeSize::new(size),
            }),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    static REGISTERS: [Register; 32] = [
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
    use super::*;
    fn get_decoder_result(pattern: u32) -> Opcode {
        let cpu = Cpu::new();
        cpu.decoder(pattern)
    }

    fn simple_move_expect(dest: Register, src: Register, size: OpcodeSize) -> Opcode {
        Opcode::Move(MoveOpcode {
            destination: dest,
            source: src,
            offset: 0,
            size,
        })
    }

    fn generate_opcode(dest: Register, src: Register, offset: u32, size: OpcodeSize) -> u32 {
        let into: u32 = size.into();
        let src: u32 = src.into();
        let dest: u32 = dest.into();
        let result = (into << 28) | (offset << 22) | (src << 16) | (dest << 10) | 0x01;
        dbg!(result);
        result
    }

    //NOTE: This test covers all __possible__ combinations of 32bit registers!
    //      there shouldn't be any cases that contain undefined behaviour
    #[test]
    fn test_move_dword_registers() {
        for dest in &REGISTERS {
            for src in &REGISTERS {
                let opcode = generate_opcode(*dest, *src, 0, OpcodeSize::Dword);

                let result = get_decoder_result(opcode);
                let expected = simple_move_expect(*dest, *src, OpcodeSize::Dword);
                assert_eq!(result, expected, "Failed {:?} {:?}", result, expected)
            }
        }
    }
}
