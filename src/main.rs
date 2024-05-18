mod cpu;

use cpu::core;

use crate::cpu::isa_generator;

const MEMORY_SIZE: usize = 128 * (1024 * 1024);

struct Memory {
    bytes: Box<[u8; MEMORY_SIZE]>,
}

impl Memory {
    fn new() -> Self {
        Self {
            bytes: vec![0; MEMORY_SIZE].into_boxed_slice().try_into().unwrap(),
        }
    }
}

struct RegisterFile {
    //D0..D15
    data_registers: [u32; 16],
    //A0..A15
    address_registers: [u32; 16],
}

impl RegisterFile {
    fn new() -> Self {
        Self {
            data_registers: [0; 16],
            address_registers: [0; 16],
        }
    }
}

fn main() {
    //let cpu = core::Cpu::new();
    //cpu.decoder(u32::MAX);
    //println!("Hello, world!");

    //YAY Combinatorical explosions -.-
    isa_generator::generate_valid_atomic_move_listing();
    isa_generator::generate_valid_memory_source_listing();
    isa_generator::genrate_valid_memory_destination_listing();
    isa_generator::generate_valid_memory_move_inc_source_listing();
}
