pub mod cpu;

use cpu::{core, opcode_size::OpcodeSize};

const MEMORY_SIZE: usize = 128 * (1024 * 1024);

enum MemoryWriteCommand {
    WriteByte { address: u32, value: u8 },
    WriteWord { address: u32, value: u16 },
    WriteDword { address: u32, value: u32 },
}

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

    fn memory_bus_write(&mut self, command: MemoryWriteCommand) {
        match command {
            MemoryWriteCommand::WriteByte { address, value } => self.write_byte(address, value),
            MemoryWriteCommand::WriteWord { address, value } => self.write_word(address, value),
            MemoryWriteCommand::WriteDword { address, value } => self.write_dword(address, value),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        self.bytes[address as usize] = value;
    }

    fn write_word(&mut self, address: u32, value: u16) {
        let byte_hi = ((value >> 8) & 0xFF) as u8;
        let byte_lo = (value & 0xFF) as u8;

        self.bytes[address as usize] = byte_hi;
        self.bytes[(address + 1) as usize] = byte_lo;
    }

    fn write_dword(&mut self, address: u32, value: u32) {
        let byte_a = ((value >> 24) & 0xFF) as u8;
        let byte_b = ((value >> 16) & 0xFF) as u8;
        let byte_c = ((value >> 8) & 0xFF) as u8;
        let byte_d = (value & 0xFF) as u8;

        self.bytes[address as usize] = byte_a;
        self.bytes[(address + 1) as usize] = byte_b;
        self.bytes[(address + 2) as usize] = byte_c;
        self.bytes[(address + 3) as usize] = byte_d;
    }

    fn read_byte(&self, address: u32) -> u32 {
        let address = address as usize;
        self.bytes[address] as u32
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
