use dielectric_cpu::cpu::core;
use std::fs;
use std::io::prelude::*;
fn main() {
    let mut mcode = fs::File::open("output.bin").unwrap();
    let mut buffer = Vec::new();

    mcode.read_to_end(&mut buffer).unwrap();

    let mut core = core::Cpu::new();
    core.load_bytes_into_memory_debug(&buffer);

    core.cycle();
    core.cycle();
    core.cycle();
}
