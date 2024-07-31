use assembler::Assembler;

fn main() {
    let source = "start:\nnop move.dw D0, D7";
    let mut assembler = Assembler::new();
    assembler.assemble(source);
}
