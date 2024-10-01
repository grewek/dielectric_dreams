use assembler::Assembler;

fn main() {
    let source = "push.dw (a0)+\npush.dw(a0)-";
    let mut assembler = Assembler::new();
    assembler.assemble(source);
}
