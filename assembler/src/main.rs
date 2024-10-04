use assembler::Assembler;
use std::env;
use std::fs;
use std::io::prelude::*;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("USAGE: assembler [source_file].dasm");
        return;
    }

    let source = fs::read_to_string(&args[0]).unwrap();

    //let source = "push.dw (a0)+\npush.dw(a0)-";
    let mut assembler = Assembler::new();
    let machine_code = assembler.assemble(&source);

    let mut output_file = fs::File::create("output.bin").unwrap();

    for code in machine_code {
        let bytes: [u8; 4] = [
            (code >> 24 & 0xFF) as u8,
            (code >> 16 & 0xFF) as u8,
            (code >> 8 & 0xFF) as u8,
            (code & 0xFF) as u8,
        ];

        output_file.write_all(&bytes).unwrap();
    }
}
