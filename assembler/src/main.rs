use assembler::Assembler;

fn main() {
    let source = "start:\nnop move.dw D0, D7\nmove.dw D2, $AABBCCDD\nmove.dw D3, #-5055324";
    let mut assembler = Assembler::new();
    assembler.assemble(source);
}
