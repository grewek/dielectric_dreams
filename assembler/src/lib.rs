mod lexer;
//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
use lexer::{Token, TokenType, Tokenizer};
use std::fmt;

pub enum RegisterTypes {
    DataRegisters,
    AddressRegisters,
}

pub enum MemoryTypes {
    ImmediateValue,
    Direct,
    DirectInc,
    DirectDec,
}
#[derive(Debug)]
pub enum ParserError {
    //TODO: I want more descriptive Errortypes which tell the user exactly what is wrong i.e. the opcode size on is a good example.
    UnexpectedSymbol(String, String),
    InvalidOpcodeSize(String),
    InvalidOperand(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParserError::UnexpectedSymbol(expected, got) => {
                write!(f, "expected a '{}' but got a '{}'", expected, got)
            }
            ParserError::InvalidOpcodeSize(got) => {
                write!(
                    f,
                    "expected an opcode size like 'b(yte)','w(ord)' or 'dw(ord)' but got {}",
                    got
                )
            }
            ParserError::InvalidOperand(got) => {
                write!(
                    f,
                    "the '{}' given operand is invalid in the current context",
                    got
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast<'a> {
    Nop {
        repr: Token<'a>,
    },
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

    MemoryTarget {
        repr: Token<'a>,
        operation: Token<'a>,
    },
    Plus {
        repr: Token<'a>,
    },
    Minus {
        repr: Token<'a>,
    },

    Register {
        repr: Token<'a>,
    },
    Number {
        repr: Token<'a>,
    },
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    prev_token: Option<Token<'a>>,
    curr_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
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

    fn match_token(&mut self, to_match: TokenType) -> bool {
        if self.curr_token.token_type() == to_match {
            self.advance();
            return true;
        }
        false
    }

    fn match_number_type(&mut self, tt: TokenType) -> Option<Ast<'a>> {
        match tt {
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

    fn match_size(&mut self, tt: TokenType) -> Option<Ast<'a>> {
        match tt {
            TokenType::Byte | TokenType::Word | TokenType::Dword => {
                let result = Some(Ast::Size {
                    repr: self.curr_token,
                });
                self.advance();
                result
            }
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

    fn parse_move(&mut self) -> Result<Ast, ParserError> {
        if !self.match_token(TokenType::Dot) {
            return Err(ParserError::UnexpectedSymbol(
                ".".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        let tt = self.curr_token.token_type();
        //Next up we match the Size of the Move Opcode
        let size_ast: Ast = if let Some(ast) = self.match_size(tt) {
            ast
        } else {
            return Err(ParserError::InvalidOpcodeSize(
                self.curr_token.get_repr().to_string(),
            ));
        };

        let tt = self.curr_token.token_type();
        let dest_register = if let Some(ast) = self.match_register_type(tt) {
            ast
        } else {
            return Err(ParserError::InvalidOperand(
                self.curr_token.get_repr().to_string(),
            ));
        };

        if !self.match_token(TokenType::Comma) {
            return Err(ParserError::UnexpectedSymbol(
                ",".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        let tt = self.curr_token.token_type();
        let src_register: Ast = if let Some(ast) = self.match_number_type(tt) {
            ast
        } else if let Some(ast) = self.match_register_type(tt) {
            ast
        } else {
            return Err(ParserError::InvalidOperand(
                self.curr_token.get_repr().to_string(),
            ));
        };

        Ok(Ast::Move {
            size: Box::new(size_ast),
            dest: Box::new(dest_register),
            src: Box::new(src_register),
        })
    }

    fn parse_lea(&mut self) -> Result<Ast, ParserError> {
        todo!()
    }

    fn parse_label_definition(&mut self) -> Result<Ast, ParserError> {
        let label = self.curr_token;
        self.advance();

        if !self.match_token(TokenType::Colon) {
            return Err(ParserError::UnexpectedSymbol(
                ":".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        return Ok(Ast::LabelDefinition { repr: label });
    }
    pub fn parse(&mut self) -> Result<Ast, ParserError> {
        match self.curr_token.token_type() {
            TokenType::Identifier => self.parse_label_definition(),
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

        assert!(matches!(node, Ok(Ast::Move { .. })));

        match node {
            Ok(Ast::Move { size, dest, src }) => {
                assert!(matches!(size.as_ref(), Ast::Size { .. }));
                assert!(matches!(dest.as_ref(), Ast::Register { .. }));
                assert!(matches!(src.as_ref(), Ast::Register { .. }));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_move_error_missing_dot() {
        let source = "move D0, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::UnexpectedSymbol(_, _))))
    }

    #[test]
    fn test_parse_move_error_missing_size() {
        let source = "move. D0, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOpcodeSize(_))))
    }

    #[test]
    fn test_parse_move_error_faulty_destination() {
        let source = "move.dw hello, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOperand(_))))
    }

    #[test]
    fn test_parse_move_error_faulty_source() {
        let source = "move.dw D0, +";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOperand(_))))
    }

    #[test]
    fn test_parse_move_immediate_value() {
        let source = "move.dw D0, $AABBCCDD";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Ok(Ast::Move { .. })));

        match node {
            Ok(Ast::Move { size, dest, src }) => {
                assert!(matches!(size.as_ref(), Ast::Size { .. }));
                assert!(matches!(dest.as_ref(), Ast::Register { .. }));
                assert!(matches!(src.as_ref(), Ast::Number { .. }));
            }
            _ => unreachable!(),
        }
    }
}
