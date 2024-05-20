pub mod cpu;

use cpu::core;

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
    //D0..D15 & A0..A15
    registers: [u32; 32],
}

impl RegisterFile {
    fn new() -> Self {
        Self { registers: [0; 32] }
    }
}
