use assembler::Assembler;

fn main() {
    let source = "start:\nmove.dw A0, $AABBCCDD\nmove.w D0,$FFBB\nlea.dw A0,$FFBBCCDD";
    let mut assembler = Assembler::new();
    assembler.assemble(source);
}
