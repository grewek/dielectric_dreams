mod lexer;
//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
use lexer::{Token, TokenType, Tokenizer};

#[derive(Debug, PartialEq, Eq)]
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

    fn match_token(&mut self, to_match: TokenType, error_message: &str) -> bool {
        if self.curr_token.token_type() == to_match {
            self.advance();
            return true;
        }

        println!("Error: {}", error_message);
        false
    }

    fn match_number_type(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        match current_token {
            TokenType::HexNumber(_) | TokenType::DecimalNumber(_) | TokenType::BinaryNumber(_) => {
                let result = Some(Ast::Number {
                    repr: self.curr_token,
                });
                self.advance();
                result
            }
            token => None,
        }
    }

    fn match_size(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        match self.curr_token.token_type() {
            TokenType::Byte | TokenType::Word | TokenType::Dword => Some(Ast::Size {
                repr: self.curr_token,
            }),
            _ => None, //If we cannot match any of these then we have an error!
        }
    }

    fn match_register_type(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        match current_token {
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
            | TokenType::A15 => {
                let result = Some(Ast::Register {
                    repr: self.curr_token,
                });
                self.advance();

                result
            }
            _ => None,
        }
    }

    fn parse_move(&mut self) -> Ast {
        if !self.match_token(TokenType::Dot, "'.' expected after a sized opcode!") {
            todo!();
        }

        //TODO: We need a way to match many tokens!

        let tt = self.curr_token.token_type();
        //Next up we match the Size of the Move Opcode
        let size_ast: Ast = if let Some(ast) = self.match_size(tt) {
            ast
        } else {
            panic!("error");
        };

        self.advance();
        let tt = self.curr_token.token_type();
        let dest_register = if let Some(ast) = self.match_register_type(tt) {
            ast
        } else {
            panic!("error");
        };

        if !self.match_token(
            TokenType::Comma,
            "missing ',' after the first operand of the opcode",
        ) {
            todo!()
        }

        let tt = self.curr_token.token_type();
        let src_register: Ast = if let Some(ast) = self.match_number_type(tt) {
            ast
        } else if let Some(ast) = self.match_register_type(tt) {
            ast
        } else {
            panic!("Error")
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_move() {
        let source = "move.dw D0, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();
        println!("{:#?}", node);

        assert!(matches!(node, Ast::Move { .. }));

        match node {
            Ast::Move { size, dest, src } => {
                assert!(matches!(size.as_ref(), Ast::Size { .. }));
                assert!(matches!(dest.as_ref(), Ast::Register { .. }));
                assert!(matches!(src.as_ref(), Ast::Register { .. }));
            }
            _ => unreachable!(),
        }

        /*assert_eq!(
            node,
            Ast::Move {
                size: Box::new(Ast::Size {
                    repr: Token {
                        token_type: TokenType::Dword,
                        repr: "dw",
                        start: "move.".len(),
                        end: "move.dw".len(),
                        line: 1,
                    }
                }),
                dest: Box::new(Ast::Register {
                    repr: Token {
                        token_type: TokenType::D0,
                        repr: "D0",
                        start: "move.dw ".len(),
                        end: "move.dw D0".len(),
                        line: 1,
                    }
                }),
                src: Box::new(Ast::Register {
                    repr: Token {
                        token_type: TokenType::A2,
                        repr: "A2",
                        start: "move.dw D0, ".len(),
                        end: "move.dw D0, A2".len(),
                        line: 1,
                    }
                })
            }
        )*/
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
