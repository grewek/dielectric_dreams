mod lexer;
mod parser;
use lexer::TokenType;
//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
use parser::{Ast, Parser};
use std::collections::{HashMap, HashSet};

pub struct Assembler {
    label_definitions: HashMap<String, usize>,
    missing_labels: HashSet<String>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            label_definitions: HashMap::new(),
            missing_labels: HashSet::new(),
        }
    }

    pub fn encode_register(tt: TokenType) -> u32 {
        match tt {
            TokenType::D0 => 0x00,
            TokenType::D1 => 0x01,
            TokenType::D2 => 0x02,
            TokenType::D3 => 0x03,
            TokenType::D4 => 0x04,
            TokenType::D5 => 0x05,
            TokenType::D6 => 0x06,
            TokenType::D7 => 0x07,
            TokenType::D8 => 0x08,
            TokenType::D9 => 0x09,
            TokenType::D10 => 0x0A,
            TokenType::D11 => 0x0B,
            TokenType::D12 => 0x0C,
            TokenType::D13 => 0x0D,
            TokenType::D14 => 0x0E,
            TokenType::D15 => 0x0F,
            TokenType::A0 => 0x10,
            TokenType::A1 => 0x11,
            TokenType::A2 => 0x12,
            TokenType::A3 => 0x13,
            TokenType::A4 => 0x14,
            TokenType::A5 => 0x15,
            TokenType::A6 => 0x16,
            TokenType::A7 => 0x17,
            TokenType::A8 => 0x18,
            TokenType::A9 => 0x19,
            TokenType::A10 => 0x1A,
            TokenType::A11 => 0x1B,
            TokenType::A12 => 0x1C,
            TokenType::A13 => 0x1D,
            TokenType::A14 => 0x1E,
            TokenType::A15 => 0x1F,
            _ => unreachable!(),
        }
    }
    pub fn encode_dest(ast: &Ast) -> u32 {
        match ast {
            Ast::MemoryTarget { repr, operation } => todo!(),
            Ast::Register { repr } => Assembler::encode_register(repr.token_type()) << 14,
            Ast::Number { repr } => todo!(),
            _ => unreachable!(),
        }
    }
    pub fn encode_source(ast: &Ast) -> u32 {
        match ast {
            Ast::MemoryTarget { repr, operation } => todo!(),
            Ast::Register { repr } => Assembler::encode_register(repr.token_type()) << 19,
            Ast::Number { repr } => todo!(),
            Ast::ProgramEnd => todo!(),
            _ => unreachable!(),
        }
    }
    pub fn generate_operation_size(ast: &Ast) -> u32 {
        match ast {
            Ast::Size { repr } => {
                match repr.token_type() {
                    TokenType::Byte => 0x00 << 29,
                    TokenType::Word => 0x01 << 29,
                    TokenType::Dword => 0x02 << 29,
                    //NOTE: Until this point all possible errors have been handled
                    //      if we reach this it would be a bug!
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn generate_operation_opcode(tt: TokenType) -> u32 {
        match tt {
            TokenType::Move => 0x01,
            TokenType::Lea => 0x02,
            _ => unreachable!(),
        }
    }
    pub fn assemble(&mut self, source: &str) {
        let mut assembled: Vec<u32> = vec![];
        let mut parser = Parser::new(source);

        let mut current_position_in_bytes = 0;
        loop {
            let to_parse = parser.parse().expect("failure in parsing the source code ");

            if to_parse == Ast::ProgramEnd {
                return;
            }

            match to_parse {
                Ast::Label { repr } => {
                    if self.label_definitions.contains_key(repr.get_repr()) {
                        //TODO: Insert the value stored inside our label hashmap
                    } else {
                        //TODO: This is for labels coming later in the soruce!
                        self.missing_labels.insert(repr.get_repr().to_string());
                    }
                }
                Ast::LabelDefinition { repr } => {
                    //TODO: Collisions
                    self.label_definitions
                        .insert(repr.get_repr().to_string(), current_position_in_bytes);
                }
                Ast::Move { size, dest, src } => {
                    let opcode = Assembler::generate_operation_opcode(TokenType::Move);
                    let size = Assembler::generate_operation_size(size.as_ref());
                    let dest = Assembler::encode_dest(dest.as_ref());
                    let src = Assembler::encode_source(src.as_ref());
                    assembled.push(size | src | dest | opcode);
                }
                Ast::Lea { dest, src } => todo!(),
                Ast::Nop { repr } => {
                    assembled.push(0x00000000);
                }
                _ => todo!(),
            }
            current_position_in_bytes += 4;
            //println!("{:?}", to_parse);

            println!("{:#x?}", assembled);
        }
    }
}
