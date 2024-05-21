pub mod cpu;

use cpu::{core, opcode_size::OpcodeSize};

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

    fn memory_bus_read(&self, size: &OpcodeSize, address: u32) -> u32 {
        match size {
            OpcodeSize::Byte => self.read_byte(address),
            OpcodeSize::Word => self.read_word(address),
            OpcodeSize::Dword => self.read_dword(address),
        }
    }

    fn read_byte(&self, address: u32) -> u32 {
        self.bytes[address as usize] as u32
    }

    fn read_word(&self, address: u32) -> u32 {
        (self.bytes[address as usize] as u32) << 8 | (self.bytes[(address + 1) as usize] as u32)
    }

    fn read_dword(&self, address: u32) -> u32 {
        println!("{}", self.bytes.len());
        //NOTE: For now we use big endian !
        (self.bytes[address as usize] as u32) << 24
            | (self.bytes[(address + 1) as usize] as u32) << 16
            | (self.bytes[(address + 2) as usize] as u32) << 8
            | (self.bytes[(address + 3) as usize] as u32)
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
