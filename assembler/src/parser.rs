use crate::lexer::{Token, TokenType, Tokenizer};
use std::fmt;

enum Opcode {
    Nop = 0x00,
    Move = 0x01,
    Lea = 0x02,
}

struct OpcodeGrammar<'a> {
    opcode: Opcode,
    dest_reg_modes: &'a [RegisterModes],
    dest_mem_modes: &'a [MemoryModes],

    src_reg_modes: &'a [RegisterModes],
    src_mem_modes: &'a [MemoryModes],
}

static OPCODE_GRAMMARS: [OpcodeGrammar; 3] = [
    OpcodeGrammar {
        opcode: Opcode::Nop,
        dest_reg_modes: &[],
        dest_mem_modes: &[],
        src_reg_modes: &[],
        src_mem_modes: &[],
    },
    OpcodeGrammar {
        opcode: Opcode::Move,
        dest_reg_modes: &[
            RegisterModes::AddressRegisters,
            RegisterModes::DataRegisters,
        ],
        dest_mem_modes: &[],
        src_reg_modes: &[
            RegisterModes::AddressRegisters,
            RegisterModes::DataRegisters,
        ],
        src_mem_modes: &[
            MemoryModes::Direct,
            MemoryModes::DirectDec,
            MemoryModes::DirectInc,
            MemoryModes::ImmediateValue,
        ],
    },
    OpcodeGrammar {
        opcode: Opcode::Lea,
        dest_reg_modes: &[],
        dest_mem_modes: &[],
        src_reg_modes: &[],
        src_mem_modes: &[],
    },
];

//TODO: Error recovery!
pub enum RegisterModes {
    DataRegisters,
    AddressRegisters,
}

impl fmt::Display for RegisterModes {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            RegisterModes::DataRegisters => write!(f, "Data Register Target: D0..D15"),
            RegisterModes::AddressRegisters => write!(f, "Address Register Target: A0..A15"),
        }
    }
}

pub enum MemoryModes {
    ImmediateValue,
    Direct,
    DirectInc,
    DirectDec,
}

#[derive(Debug)]
pub enum ParserError {
    //TODO: I want more descriptive Errortypes which tell the user exactly what is wrong i.e. the opcode size.
    UnexpectedSymbol(usize, usize, String, String),
    InvalidOpcodeSize(usize, usize, String),
    InvalidOperand(usize, usize, String),
    IllegalSize(usize, usize, String, String),
    MissingSeperator(usize, usize, String, String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParserError::UnexpectedSymbol(line, position, expected, got) => {
                write!(
                    f,
                    "{}:{}:expected a '{}' but got a '{}'",
                    line, position, expected, got
                )
            }
            ParserError::InvalidOpcodeSize(line, position, got) => {
                write!(
                    f,
                    "{}:{}:expected an opcode size like 'b(yte)','w(ord)' or 'dw(ord)' but got {}",
                    line, position, got
                )
            }
            ParserError::InvalidOperand(line, position, got) => {
                write!(f, "{}:{}:found unexpected symbol {}", line, position, got)
            }
            ParserError::IllegalSize(line, position, got, message) => {
                write!(
                    f,
                    "{}:{}:the size '{}' is invalid for the given opcode.\nhint: {}",
                    line, position, got, message
                )
            }
            ParserError::MissingSeperator(line, position, expected, got) => {
                write!(
                    f,
                    "{}:{}:expected a '{}' between arguments of an opcode but got '{}'",
                    line, position, expected, got
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
    ProgramEnd,
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
            TokenType::ByteHexNumber(_)
            | TokenType::ByteDecimalNumber(_)
            | TokenType::ByteBinaryNumber(_)
            | TokenType::WordHexNumber(_)
            | TokenType::WordDecimalNumber(_)
            | TokenType::WordBinaryNumber(_)
            | TokenType::DwordHexNumber(_)
            | TokenType::DwordDecimalNumber(_)
            | TokenType::DwordBinaryNumber(_) => {
                let result = Some(Ast::Number {
                    repr: self.curr_token,
                });
                self.advance();
                result
            }
            _token => None,
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

    fn match_address_register(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        match current_token {
            TokenType::A0
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
    fn match_data_register(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
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
            | TokenType::D15 => {
                let result = Some(Ast::Register {
                    repr: self.curr_token,
                });
                self.advance();

                result
            }
            _ => None,
        }
    }

    fn match_all_registers(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
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

    fn parse_immediate_value(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        let tt = self.curr_token.token_type();
        self.match_number_type(tt)
    }

    fn parse_direct(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        None
    }

    fn parse_direct_inc(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        None
    }

    fn parse_direct_dec(&mut self, current_token: TokenType) -> Option<Ast<'a>> {
        None
    }

    fn parse_register_arg(
        &mut self,
        mode: &RegisterModes,
        current_token: TokenType,
    ) -> Option<Ast<'a>> {
        match mode {
            RegisterModes::DataRegisters => self.match_data_register(current_token),
            RegisterModes::AddressRegisters => self.match_address_register(current_token),
        }
    }
    fn parse_memory_arg(
        &mut self,
        mode: &MemoryModes,
        current_token: TokenType,
    ) -> Option<Ast<'a>> {
        match mode {
            MemoryModes::ImmediateValue => self.parse_immediate_value(current_token),
            MemoryModes::Direct => self.parse_direct(current_token),
            MemoryModes::DirectInc => self.parse_direct_inc(current_token),
            MemoryModes::DirectDec => self.parse_direct_dec(current_token),
        }
    }
    fn parse_arg(
        &mut self,
        register_types: &[RegisterModes],
        memory_types: &[MemoryModes],
    ) -> Result<Ast<'a>, ParserError> {
        let tt = self.curr_token.token_type();

        for mode in register_types {
            if let Some(register_arg) = self.parse_register_arg(mode, tt) {
                return Ok(register_arg);
            }
        }

        for mode in memory_types {
            if let Some(memory_arg) = self.parse_memory_arg(mode, tt) {
                return Ok(memory_arg);
            }
        }

        Err(ParserError::InvalidOperand(
            self.curr_token.get_line(),
            self.curr_token.get_position(),
            self.curr_token.get_repr().to_string(),
        ))
    }

    fn parse_move(&mut self, grammar: &OpcodeGrammar) -> Result<Ast, ParserError> {
        //Why are my parsers always so messy -.- how do i improve this...?!?
        if !self.match_token(TokenType::Dot) {
            return Err(ParserError::UnexpectedSymbol(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
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
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                self.curr_token.get_repr().to_string(),
            ));
        };

        let dest_register = self.parse_arg(grammar.dest_reg_modes, grammar.dest_mem_modes)?;

        if !self.match_token(TokenType::Comma) {
            return Err(ParserError::MissingSeperator(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                ",".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        let src_register = self.parse_arg(grammar.src_reg_modes, grammar.src_mem_modes)?;

        Ok(Ast::Move {
            size: Box::new(size_ast),
            dest: Box::new(dest_register),
            src: Box::new(src_register),
        })
    }

    fn parse_lea(&mut self, grammar: &OpcodeGrammar) -> Result<Ast, ParserError> {
        if !self.match_token(TokenType::Dot) {
            return Err(ParserError::UnexpectedSymbol(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                ".".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        //Match the size only dword is possible!
        if !self.match_token(TokenType::Dword) {
            let repr = self.curr_token.get_repr();
            return Err(ParserError::IllegalSize(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                repr.to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        let dest_register = self.parse_arg(&[RegisterModes::AddressRegisters], &[])?;

        if !self.match_token(TokenType::Comma) {
            return Err(ParserError::MissingSeperator(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                ",".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        let src_register = self.parse_arg(&[], &[MemoryModes::ImmediateValue])?;

        Ok(Ast::Lea {
            dest: Box::new(dest_register),
            src: Box::new(src_register),
        })
    }

    fn parse_label_definition(&mut self) -> Result<Ast, ParserError> {
        let label = self.curr_token;
        self.advance();

        if !self.match_token(TokenType::Colon) {
            return Err(ParserError::UnexpectedSymbol(
                self.curr_token.get_line(),
                self.curr_token.get_position(),
                ":".to_string(),
                self.curr_token.get_repr().to_string(),
            ));
        }

        Ok(Ast::LabelDefinition { repr: label })
    }

    pub fn parse(&mut self) -> Result<Ast, ParserError> {
        match self.curr_token.token_type() {
            TokenType::Identifier => self.parse_label_definition(),
            TokenType::Move => {
                self.advance();
                self.parse_move(&OPCODE_GRAMMARS[Opcode::Move as usize])
            }
            TokenType::Lea => {
                self.advance();
                self.parse_lea(&OPCODE_GRAMMARS[Opcode::Lea as usize])
            }
            TokenType::Nop => {
                self.advance();

                Ok(Ast::Nop {
                    repr: self.curr_token,
                })
            }
            TokenType::EndOfFile => Ok(Ast::ProgramEnd),
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

        assert!(matches!(
            node,
            Err(ParserError::UnexpectedSymbol(_, _, _, _))
        ))
    }

    #[test]
    fn test_parse_move_error_missing_size() {
        let source = "move. D0, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOpcodeSize(_, _, _))))
    }

    #[test]
    fn test_parse_move_error_faulty_destination() {
        let source = "move.dw hello, A2";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOperand(_, _, _))))
    }

    #[test]
    fn test_parse_move_error_faulty_source() {
        let source = "move.dw D0, +";

        let mut parser = Parser::new(source);

        let node = parser.parse();

        assert!(matches!(node, Err(ParserError::InvalidOperand(_, _, _))))
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
