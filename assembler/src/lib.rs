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

    pub fn generate_operation_size(tt: TokenType) -> u32 {
        match tt {
            TokenType::Byte => 0x00 << 29,
            TokenType::Word => 0x01 << 29,
            TokenType::Dword => 0x02 << 29,
            //NOTE: Until this point all possible errors have been handled
            //      if we reach this it would be a bug!
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
                Ast::Move { size, dest, src } => todo!(),
                Ast::Lea { dest, src } => todo!(),
                Ast::Nop { repr } => {
                    assembled.push(0x00000000);
                }
                _ => todo!(),
            }
            current_position_in_bytes += 4;
            println!("{:?}", to_parse);
        }
    }
}
