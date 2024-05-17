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
