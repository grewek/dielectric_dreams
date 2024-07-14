use assembler::Parser;

fn main() {
    let source = "start:\nmove.dw A0, $AABBCCDD";

    let mut parser = Parser::new(source);
    println!("{:?}", parser.parse().unwrap());
    println!("{:?}", parser.parse().unwrap());
}
