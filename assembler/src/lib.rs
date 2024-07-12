mod lexer;
//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
use lexer::{Token, TokenType, Tokenizer};

enum Ast<'a> {
    LabelDefinition {
        repr: Token<'a>,
    },
    Label {
        repr: Token<'a>,
    },
    Move {
        size: Box<Ast<'a>>,
        dest: Box<Ast<'a>>,
        src: Box<Ast<'a>>,
    },
    Lea {
        repr: Token<'a>,

        dest: Box<Ast<'a>>,
        src: Box<Ast<'a>>,
    },

    Size {
        repr: Token<'a>,
    },

    Register {
        repr: Token<'a>,
    },
    Number {
        repr: Token<'a>,
    },
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    prev_token: Option<Token<'a>>,
    curr_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        let curr_token = tokenizer.next();
        Self {
            tokenizer,
            prev_token: None,
            curr_token,
        }
    }

    fn advance(&mut self) {
        self.prev_token = Some(self.curr_token);
        self.curr_token = self.tokenizer.next();
    }

    fn parse_move(&mut self) -> Ast {
        //Next up we match the Size of the Move Opcode
        let size_ast = match self.curr_token.token_type() {
            TokenType::Byte | TokenType::Word | TokenType::Dword => Ast::Size {
                repr: self.curr_token,
            },
            _ => todo!(), //If we cannot match any of these then we have an error!
        };

        let dest_register = match self.curr_token.token_type() {
            //TODO: Memory, MemoryInc, MemoryDec
            TokenType::HexNumber(_) | TokenType::BinaryNumber(_) | TokenType::DecimalNumber(_) => {
                //I only really care if we have some kind of number not what the number actually is...
                //so we can safely ignore all the information that is contained within the TokenType
                //and just use the "header" for matching purposes :) Then we just store the token inside
                //our Ast node and we have the number at the ready when we need it!
                Ast::Number {
                    repr: self.curr_token,
                }
            }
            TokenType::D0
            | TokenType::D1
            | TokenType::D2
            | TokenType::D3
            | TokenType::D4
            | TokenType::D5
            | TokenType::D6
            | TokenType::D7
            | TokenType::D8
            | TokenType::D9
            | TokenType::D10
            | TokenType::D11
            | TokenType::D12
            | TokenType::D13
            | TokenType::D14
            | TokenType::D15
            | TokenType::A0
            | TokenType::A1
            | TokenType::A2
            | TokenType::A3
            | TokenType::A4
            | TokenType::A5
            | TokenType::A6
            | TokenType::A7
            | TokenType::A8
            | TokenType::A9
            | TokenType::A10
            | TokenType::A11
            | TokenType::A12
            | TokenType::A13
            | TokenType::A14
            | TokenType::A15 => Ast::Register {
                repr: self.curr_token,
            },
            _ => todo!(),
        };

        let src_register = match self.curr_token.token_type() {
            TokenType::D0
            | TokenType::D1
            | TokenType::D2
            | TokenType::D3
            | TokenType::D4
            | TokenType::D5
            | TokenType::D6
            | TokenType::D7
            | TokenType::D8
            | TokenType::D9
            | TokenType::D10
            | TokenType::D11
            | TokenType::D12
            | TokenType::D13
            | TokenType::D14
            | TokenType::D15
            | TokenType::A0
            | TokenType::A1
            | TokenType::A2
            | TokenType::A3
            | TokenType::A4
            | TokenType::A5
            | TokenType::A6
            | TokenType::A7
            | TokenType::A8
            | TokenType::A9
            | TokenType::A10
            | TokenType::A11
            | TokenType::A12
            | TokenType::A13
            | TokenType::A14
            | TokenType::A15 => Ast::Register {
                repr: self.curr_token,
            },
            _ => todo!(),
        };

        Ast::Move {
            size: Box::new(size_ast),
            dest: Box::new(dest_register),
            src: Box::new(src_register),
        }
    }

    fn parse_lea(&mut self) -> Ast {
        todo!()
    }
    fn parse(&mut self) -> Ast {
        match self.curr_token.token_type() {
            TokenType::Move => {
                self.advance();
                self.parse_move()
            }
            TokenType::Lea => self.parse_lea(),
            _ => todo!(),
        }
    }
}

//Let's hope that we can keep the borrowchecker happy or if we have to do some trickery in order to keep all the tokens
//with their fancy lifetimes alive...
/*
   Move.Dw D0, D1
   Ast::Move {
       repr: {TokenType::Move ...}
       size: Ast::Size { TokenType::DWord }
       dest: Ast::Register { TokenType::D0 }
       src: Ast::Register { TokenType::D1 }
   }
*/
