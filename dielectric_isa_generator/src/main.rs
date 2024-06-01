use std::{fs::File, io::Write};

use dielectric_cpu::cpu::{
    addressing_modes, decoder::BitPattern, opcode::Opcode, register::Register,
};

pub fn generate_memory_registers(mode: u32, dest_reg: Register, src_reg: Register) -> String {
    if dest_reg >= Register::A0 {
        //TODO(Kay): Collapse this into a match?
        if mode == 0x01 {
            return format!("({})+,{}", dest_reg, src_reg);
        } else if mode == 0x02 {
            return format!("({})-,{}", dest_reg, src_reg);
        } else {
            return format!("({}),{}", dest_reg, src_reg);
        }
    }

    if src_reg >= Register::A0 {
        if mode == 0x01 {
            return format!("{},({})+", dest_reg, src_reg);
        } else if mode == 0x02 {
            return format!("{},({})-", dest_reg, src_reg);
        } else {
            return format!("{},({})", dest_reg, src_reg);
        }
    }

    unreachable!()
}

fn generate_listing_for(
    file: &mut File,
    description: &str,
    opcode: u32,
    dest_registers: (&[Register], bool),
    src_registers: (&[Register], bool),
    offset: u32,
    inc_mode: u32,
) {
    writeln!(file, "{}", description).unwrap();
    for opcode_size in 0..3 {
        let (dest_registers, dest_memory_loc) = dest_registers;
        let (src_registers, src_memory_loc) = src_registers;

        for dest_pattern in dest_registers {
            let dest = *dest_pattern;
            let dest_pattern: u32 = if dest_memory_loc {
                let memory_bit_mask: u32 = 1 << 5;
                let dest: u32 = dest.into();
                dest | memory_bit_mask
            } else {
                dest.into()
            };

            for src_pattern in src_registers {
                let src = *src_pattern;
                let src_pattern: u32 = if src_memory_loc {
                    let memory_bit_mask: u32 = 1 << 5;
                    let src: u32 = src.into();
                    src | memory_bit_mask
                } else {
                    src.into()
                };
                let pattern = generate_atomic_move_opcode(
                    opcode,
                    dest_pattern,
                    src_pattern,
                    offset,
                    opcode_size,
                    inc_mode,
                );
                //generate_isa_for_opcode(file, pattern);
            }
        }
    }
}

const ALL_REGISTERS: [Register; 32] = [
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

const DATA_REGISTERS: [Register; 16] = [
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

const ADDRESS_REGISTERS: [Register; 16] = [
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

const OPCODE_MOVE: u32 = 0x01;

pub fn generate_isa(file: &mut File) {
    generate_listing_for(
        file,
        "Register to Register Moves",
        OPCODE_MOVE,
        (&ALL_REGISTERS, false),
        (&ALL_REGISTERS, false),
        0,
        0,
    );
    generate_listing_for(
        file,
        "Memory Source to Register Moves",
        OPCODE_MOVE,
        (&DATA_REGISTERS, false),
        (&ADDRESS_REGISTERS, true),
        0,
        0,
    );
    generate_listing_for(
        file,
        "Register Value to Memory Location",
        OPCODE_MOVE,
        (&ADDRESS_REGISTERS, true),
        (&DATA_REGISTERS, false),
        0,
        0,
    );

    generate_listing_for(
        file,
        "Memory Source to Registers Moves with Increment",
        OPCODE_MOVE,
        (&DATA_REGISTERS, false),
        (&ADDRESS_REGISTERS, true),
        0,
        1,
    );
    generate_listing_for(
        file,
        "Memory Source to Registers Moves with Decrement",
        OPCODE_MOVE,
        (&DATA_REGISTERS, false),
        (&ADDRESS_REGISTERS, true),
        0,
        2,
    );

    generate_listing_for(
        file,
        "Data Register to Memory Location with Increment",
        OPCODE_MOVE,
        (&ADDRESS_REGISTERS, true),
        (&DATA_REGISTERS, false),
        0,
        1,
    );

    generate_listing_for(
        file,
        "Data Register to Memory Location with Decrement",
        OPCODE_MOVE,
        (&ADDRESS_REGISTERS, true),
        (&DATA_REGISTERS, false),
        0,
        2,
    );
}

//TODO(Kay): Function renaming!
pub fn generate_atomic_move_opcode(
    opcode: u32,
    dest_pattern: u32,
    src_pattern: u32,
    offset: u32,
    size: u32,
    increment_mode: u32,
) -> u32 {
    (increment_mode << 30)
        | (size << 28)
        | (offset << 22)
        | (src_pattern << 16)
        | (dest_pattern << 10)
        | opcode
}

//TODO(Kay): We should do this once we nailed down the isa a bit more...i expect there will be more refactors :(
/*pub fn generate_isa_for_opcode(file: &mut File, pattern: u32) {
    let pattern = BitPattern::new(pattern);
    let opcode = pattern.into();

    match opcode {
        Opcode::Move(data) => match data.addr_mode {
            addressing_modes::AddressingMode::Atomic => writeln!(
                file,
                "{:b}\t{:x}\tMOVE.{}\t{},{}",
                pattern, pattern, data.size, data.destination, data.source
            )
            .unwrap(),
            addressing_modes::AddressingMode::Memory => writeln!(
                file,
                "{:b}\t{:x}\tMOVE.{}\t{}",
                pattern,
                pattern,
                data.size,
                generate_memory_registers(pattern.increment, data.destination, data.source)
            )
            .unwrap(),
            addressing_modes::AddressingMode::MemoryInc => writeln!(
                file,
                "{:b}\t{:x}\tMOVE.{}\t{}",
                pattern,
                pattern,
                data.size,
                generate_memory_registers(pattern.increment, data.destination, data.source)
            )
            .unwrap(),
            addressing_modes::AddressingMode::MemoryDec => writeln!(
                file,
                "{:b}\t{:x}\tMOVE.{}\t{}",
                pattern,
                pattern,
                data.size,
                generate_memory_registers(pattern.increment, data.destination, data.source)
            )
            .unwrap(),
        },
        Opcode::Unknown => (),
    }
}*/

fn main() {
    let mut output = File::create("docs/generated_isa.txt").unwrap();
    generate_isa(&mut output)
}
