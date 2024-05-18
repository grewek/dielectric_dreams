use super::{decoder::BitPattern, opcode::Opcode, register::Register};

pub fn generate_memory_registers(dest_reg: Register, src_reg: Register) -> String {
    if dest_reg >= Register::A0 {
        return format!("({}),{}", dest_reg, src_reg);
    }

    if src_reg >= Register::A0 {
        return format!("{},({})", src_reg, dest_reg);
    }

    unreachable!()
}

pub fn generate_valid_atomic_move_listing() {
    for opcode_size in 0..3 {
        for dest_pattern in 0..32 {
            for src_pattern in 0..32 {
                let pattern =
                    generate_atomic_move_opcode(dest_pattern, src_pattern, 0, opcode_size);
                generate_isa_for_opcode(pattern)
            }
        }
    }
}

pub fn generate_valid_memory_source_listing() {
    for opcode_size in 0..3 {
        for dest_pattern in 0..32 {
            for src_pattern in 16..32 {
                let memory_bit_mask = 1 << 5;
                let src_pattern = src_pattern | memory_bit_mask;

                let pattern =
                    generate_atomic_move_opcode(dest_pattern, src_pattern, 0, opcode_size);
                generate_isa_for_opcode(pattern)
            }
        }
    }
}

pub fn genrate_valid_memory_destination_listing() {
    for opcode_size in 0..3 {
        for dest_pattern in 16..32 {
            let memory_bit_mask = 1 << 5;
            let dest_pattern = dest_pattern | memory_bit_mask;

            for src_pattern in 0..32 {
                let pattern =
                    generate_atomic_move_opcode(dest_pattern, src_pattern, 0, opcode_size);
                generate_isa_for_opcode(pattern)
            }
        }
    }
}

pub fn generate_valid_memory_inc_move_listing() {
    todo!()
}

pub fn generate_valid_memory_dec_move_listing() {
    todo!()
}

pub fn generate_atomic_move_opcode(
    dest_pattern: u32,
    src_pattern: u32,
    offset: u32,
    size: u32,
) -> u32 {
    (size << 28) | (offset << 22) | (src_pattern << 16) | (dest_pattern << 10) | 0x01
}

pub fn generate_isa_for_opcode(pattern: u32) {
    let pattern = BitPattern::new(pattern);
    let opcode = pattern.into();

    match opcode {
        Opcode::Move(data) => match data.addr_mode {
            super::addressing_modes::AddressingMode::Atomic => println!(
                "{:b}\t{:x}\tMOVE.{}\t{},{}",
                pattern, pattern, data.size, data.destination, data.source
            ),
            super::addressing_modes::AddressingMode::Memory => println!(
                "{:b}\t{:x}\tMOVE.{}\t{}",
                pattern,
                pattern,
                data.size,
                generate_memory_registers(data.destination, data.source)
            ),
            super::addressing_modes::AddressingMode::MemoryInc => (),
            super::addressing_modes::AddressingMode::MemoryDec => (),
        },

        Opcode::Unknown => (),
    }
}
