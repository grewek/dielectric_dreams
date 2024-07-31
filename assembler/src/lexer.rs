use std::str;

enum BitWidth {
    Byte,
    Word,
    Dword,
}

//TODO:
//      - Refactor the lexer into it's own file
//      - remove the last unwraps!
//      - add more tests for possible opcodes like addressing modes and immediate values!
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    //Identifiers
    Identifier,
    //Keywords
    Move,
    Lea,
    Nop,

    Byte,
    Word,
    Dword,

    //Registers
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,

    // Number Types
    ByteHexNumber(u8),
    ByteDecimalNumber(i8),
    ByteBinaryNumber(u8),

    WordHexNumber(u16),
    WordDecimalNumber(i16),
    WordBinaryNumber(u16),

    DwordHexNumber(u32),
    DwordDecimalNumber(i32),
    DwordBinaryNumber(u32),

    //Operators
    PoundSign,
    Percent,
    DollarSign,
    OpenParen,
    CloseParen,
    Comma,
    Dot,
    Colon,
    Plus,
    Minus,
    EndOfFile,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    repr: &'a str,
    start: usize,
    end: usize,
    line: usize,
}

impl<'a> Token<'a> {
    fn new_identifier(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        //NOTE: I will __not__ implement string interning yet...as i just started a course
        //      on performance oriented programming and i want to get better at profiling stuff
        //      i should really stay away from doing any optimization...
        let repr: &str = str::from_utf8(repr).unwrap();
        let possible_keyword = repr.to_lowercase();

        let token_type = match possible_keyword.as_str() {
            "nop" => TokenType::Nop,
            "move" => TokenType::Move,
            "lea" => TokenType::Lea,
            "b" => TokenType::Byte,
            "w" => TokenType::Word,
            "dw" => TokenType::Dword,

            "d0" => TokenType::D0,
            "d1" => TokenType::D1,
            "d2" => TokenType::D2,
            "d3" => TokenType::D3,
            "d4" => TokenType::D4,
            "d5" => TokenType::D5,
            "d6" => TokenType::D6,
            "d7" => TokenType::D7,
            "d8" => TokenType::D8,
            "d9" => TokenType::D9,
            "d10" => TokenType::D10,
            "d11" => TokenType::D11,
            "d12" => TokenType::D12,
            "d13" => TokenType::D13,
            "d14" => TokenType::D14,
            "d15" => TokenType::D15,

            "a0" => TokenType::A0,
            "a1" => TokenType::A1,
            "a2" => TokenType::A2,
            "a3" => TokenType::A3,
            "a4" => TokenType::A4,
            "a5" => TokenType::A5,
            "a6" => TokenType::A6,
            "a7" => TokenType::A7,
            "a8" => TokenType::A8,
            "a9" => TokenType::A9,
            "a10" => TokenType::A10,
            "a11" => TokenType::A11,
            "a12" => TokenType::A12,
            "a13" => TokenType::A13,
            "a14" => TokenType::A14,
            "a15" => TokenType::A15,

            _ => TokenType::Identifier,
        };

        Self {
            token_type,
            repr,
            start,
            end,
            line,
        }
    }

    fn convert_typed_value(value: u32) -> BitWidth {
        //TODO: Figure out if this is the job of the parser or the lexer ?!?
        if value <= u8::MAX as u32 {
            BitWidth::Byte
        } else if value <= u16::MAX as u32 {
            BitWidth::Word
        } else {
            BitWidth::Dword
        }
    }

    fn new_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        //NOTE: Due to the way we parse numbers i am pretty sure that i can just unwrap here! and do not need to
        //      care about the error state! But i might be wrong so lets add a panic in case anything goes haywire...
        let repr = str::from_utf8(repr).unwrap();
        let value = repr
            .parse()
            .unwrap_or_else(|_| panic!("ERROR: Scanned Item was apparently not a value {}", repr));

        let token_type = match Self::convert_typed_value(value) {
            BitWidth::Byte => TokenType::ByteDecimalNumber(value as i8),
            BitWidth::Word => TokenType::WordDecimalNumber(value as i16),
            BitWidth::Dword => TokenType::DwordDecimalNumber(value as i32),
        };

        Self {
            token_type,
            repr,
            start,
            end,
            line,
        }
    }

    fn new_binary_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        let repr = str::from_utf8(repr).unwrap();
        let value = u32::from_str_radix(repr, 2).unwrap_or_else(|_| {
            panic!(
                "Error: Scanned item could not be converted into a hexadecimal value: {}",
                repr
            )
        });

        let token_type = match Self::convert_typed_value(value) {
            BitWidth::Byte => TokenType::ByteBinaryNumber(value as u8),
            BitWidth::Word => TokenType::WordBinaryNumber(value as u16),
            BitWidth::Dword => TokenType::DwordBinaryNumber(value as u32),
        };

        Self {
            token_type,
            repr,
            start,
            end,
            line,
        }
    }
    fn new_hex_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        let repr = str::from_utf8(repr).unwrap();
        let value = u32::from_str_radix(repr, 16).unwrap_or_else(|_| {
            panic!(
                "Error: Scanned Item could not be converted into a hexadecimal value: {}",
                repr
            );
        });

        let token_type = match Self::convert_typed_value(value) {
            BitWidth::Byte => TokenType::ByteHexNumber(value as u8),
            BitWidth::Word => TokenType::WordHexNumber(value as u16),
            BitWidth::Dword => TokenType::DwordHexNumber(value as u32),
        };

        Self {
            token_type,
            repr,
            start,
            end,
            line,
        }
    }

    fn eof(start: usize, line: usize) -> Self {
        Self {
            token_type: TokenType::EndOfFile,
            repr: "",
            start,
            end: start,
            line,
        }
    }

    fn new_operator(
        operator: TokenType,
        repr: &'a [u8],
        start: usize,
        end: usize,
        line: usize,
    ) -> Self {
        let repr = str::from_utf8(repr).unwrap();

        Self {
            token_type: operator,
            repr,
            start,
            end,
            line,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::PoundSign
                | TokenType::Percent
                | TokenType::DollarSign
                | TokenType::OpenParen
                | TokenType::CloseParen
                | TokenType::Comma
                | TokenType::Dot
                | TokenType::Colon
                | TokenType::Plus
                | TokenType::Minus
        )
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_repr(&self) -> &'a str {
        self.repr
    }
    pub fn is_identifier(&self) -> bool {
        matches!(self.token_type, TokenType::Identifier)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Identifier
                | TokenType::Move
                | TokenType::Lea
                | TokenType::Byte
                | TokenType::Word
                | TokenType::Dword
                | TokenType::D0
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
                | TokenType::A15
        )
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::ByteHexNumber(_)
                | TokenType::ByteDecimalNumber(_)
                | TokenType::ByteBinaryNumber(_)
                | TokenType::WordHexNumber(_)
                | TokenType::WordDecimalNumber(_)
                | TokenType::WordBinaryNumber(_)
                | TokenType::DwordHexNumber(_)
                | TokenType::DwordDecimalNumber(_)
                | TokenType::DwordBinaryNumber(_)
        )
    }
}

pub struct Tokenizer<'a> {
    source: &'a [u8],
    line: usize,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            line: 1,
            position: 0,
        }
    }

    fn advance(&mut self) {
        if self.position < self.source.len() {
            self.position += 1;
        }
    }

    fn peek(&mut self) -> Option<&u8> {
        self.source.get(self.position + 1)
    }

    fn digest_identifier(&mut self) -> (usize, usize) {
        let start = self.position;
        while let Some(ch) = self.source.get(self.position) {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || *ch == b'_' {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }

    fn digest_binary_number(&mut self) -> (usize, usize) {
        let start = self.position;
        while let Some(ch) = self.source.get(self.position) {
            if Tokenizer::is_binary(*ch) {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }
    fn digest_hex_number(&mut self) -> (usize, usize) {
        let start = self.position;
        while let Some(ch) = self.source.get(self.position) {
            if Tokenizer::is_hexadecimal(*ch) {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }

    fn digest_decmial_number(&mut self) -> (usize, usize) {
        let start = self.position;

        while let Some(ch) = self.source.get(self.position) {
            if ch.is_ascii_digit() || *ch == b'-' {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }

    fn digest_operator(&mut self) -> (TokenType, usize, usize) {
        let start = self.position;

        //TODO(Kay): From?
        let operator = match self.source.get(self.position) {
            Some(b'#') => TokenType::PoundSign,
            Some(b'%') => TokenType::Percent,
            Some(b'$') => TokenType::DollarSign,
            Some(b'(') => TokenType::OpenParen,
            Some(b')') => TokenType::CloseParen,
            Some(b',') => TokenType::Comma,
            Some(b'.') => TokenType::Dot,
            Some(b':') => TokenType::Colon,
            Some(b'+') => TokenType::Plus,
            Some(b'-') => TokenType::Minus,
            Some(ch) => todo!("Reached a {}", ch),
            None => unreachable!(),
        };

        self.advance();

        (operator, start, self.position)
    }

    fn digest_whitespace(&mut self) {
        while let Some(ch) = self.source.get(self.position) {
            if ch.is_ascii_whitespace() {
                if *ch == b'\n' || *ch == b'\t' || *ch == b'\r' {
                    self.line += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_hexadecimal(symbol: u8) -> bool {
        match symbol {
            b'a'..=b'f' | b'A'..=b'F' => true,
            b'0'..=b'9' => true,
            _ => false,
        }
    }

    fn is_binary(symbol: u8) -> bool {
        match symbol {
            b'0'..=b'1' => true,
            _ => false,
        }
    }

    pub fn next(&mut self) -> Token<'a> {
        self.digest_whitespace();

        if self.position == self.source.len() {
            return Token::eof(self.position, self.line);
        }

        match self.source.get(self.position) {
            Some(ch) if ch.is_ascii_alphabetic() || *ch == b'_' => {
                let (start, end) = self.digest_identifier();
                return Token::new_identifier(&self.source[start..end], start, end, self.line);
            }
            Some(ch)
                if *ch == b'#'
                    && (self.peek().unwrap().is_ascii_digit() || *self.peek().unwrap() == b'-') =>
            {
                self.advance();
                let (start, end) = self.digest_decmial_number();
                return Token::new_number(&self.source[start..end], start, end, self.line);
            }
            Some(ch) if *ch == b'$' && Tokenizer::is_hexadecimal(*self.peek().unwrap()) => {
                self.advance();

                let (start, end) = self.digest_hex_number();
                return Token::new_hex_number(&self.source[start..end], start, end, self.line);
            }
            Some(ch) if *ch == b'%' && Tokenizer::is_binary(*self.peek().unwrap()) => {
                self.advance();
                let (start, end) = self.digest_binary_number();
                return Token::new_binary_number(&self.source[start..end], start, end, self.line);
            }
            Some(_) => {
                //NOTE: Scanner is desperate it does not know what the next symbol is so it __must__
                //      be a operator!
                let (operator, start, end) = self.digest_operator();

                return Token::new_operator(
                    operator,
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                );
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let source = "";
        let mut tokenizer = Tokenizer::new(source);

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::EndOfFile,
                repr: "",
                start: 0,
                end: 0,
                line: 1,
            }
        );
    }

    #[test]
    fn test_discards_whitespace() {
        let whitespace = "   ";
        let mut tokenizer = Tokenizer::new(whitespace);

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::EndOfFile,
                repr: "",
                start: 3,
                end: 3,
                line: 1,
            }
        );
    }

    #[test]
    fn test_start_pos_is_zero() {
        let whitespace = "   ";
        let tokenizer = Tokenizer::new(whitespace);

        assert_eq!(tokenizer.position, 0);
    }

    #[test]
    fn test_mutate_position_to_the_end_of_the_source() {
        let whitespace = "   ";
        let mut tokenizer = Tokenizer::new(whitespace);

        let token = tokenizer.next();
        assert_eq!(
            token,
            Token {
                token_type: TokenType::EndOfFile,
                repr: "",
                start: 3,
                end: 3,
                line: 1,
            }
        );
        assert_eq!(tokenizer.position, whitespace.len());
    }

    #[test]
    fn test_generate_identifiers() {
        let source = "move";
        let mut tokenizer = Tokenizer::new(source);

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Move,
                repr: "move",
                start: 0,
                end: source.len(),
                line: 1,
            }
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_generate_identifiers_underscore() {
        let source = "_move";
        let mut tokenizer = Tokenizer::new(source);

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Identifier,
                repr: "_move",
                start: 0,
                end: source.len(),
                line: 1,
            }
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_identifier_whitespace_mix() {
        let source = "move dest src";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Move,
                repr: "move",
                start: 0,
                end: "move".len(),
                line: 1,
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Identifier,
                repr: "dest",
                start: "move".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len(),
                line: 1,
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Identifier,
                repr: "src",
                start: "move".len() + " ".len() + "dest".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len() + " ".len() + "src".len(),
                line: 1,
            }
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_decimal_numbers() {
        let source = "#1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::DecimalNumber(1337),
                repr: "1337",
                start: 1,
                end: source.len(),
                line: 1,
            }
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_negative_decimal_numbers() {
        let source = "#-1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::DecimalNumber(-1337),
                repr: "-1337",
                start: 1,
                end: source.len(),
                line: 1,
            }
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn parse_operators() {
        let source = "#%$(),.:+-";

        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::PoundSign,
                repr: "#",
                start: 0,
                end: 1,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Percent,
                repr: "%",
                start: 1,
                end: 2,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::DollarSign,
                repr: "$",
                start: 2,
                end: 3,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::OpenParen,
                repr: "(",
                start: 3,
                end: 4,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::CloseParen,
                repr: ")",
                start: 4,
                end: 5,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Comma,
                repr: ",",
                start: 5,
                end: 6,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Dot,
                repr: ".",
                start: 6,
                end: 7,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Colon,
                repr: ":",
                start: 7,
                end: 8,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Plus,
                repr: "+",
                start: 8,
                end: 9,
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::Minus,
                repr: "-",
                start: 9,
                end: 10,
                line: 1,
            }
        );
    }

    #[test]
    fn peek_character() {
        let source = "#-50";

        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(tokenizer.peek(), Some(&b'-'));
        tokenizer.next();
        assert_eq!(tokenizer.peek(), None);
    }

    #[test]
    fn test_hex_numbers() {
        let source = "$65BBCCDD";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::HexNumber(1706806493),
                repr: "65BBCCDD",
                start: 1,
                end: source.len(),
                line: 1,
            }
        );
    }

    #[test]
    fn test_binary_numbers() {
        let source = "%10110000";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::BinaryNumber(176),
                repr: "10110000",
                start: 1,
                end: source.len(),
                line: 1,
            }
        );
    }

    #[test]
    fn test_move() {
        let source = "move.dw D0,A5";

        let mut tokenizer = Tokenizer::new(source);
        let opcode_token = tokenizer.next();
        let dot_token = tokenizer.next();
        let size_token = tokenizer.next();
        let dest_token = tokenizer.next();
        let comma_token = tokenizer.next();
        let source_token = tokenizer.next();

        assert_eq!(
            opcode_token,
            Token {
                token_type: TokenType::Move,
                repr: "move",
                start: 0,
                end: "move".len(),
                line: 1,
            }
        );

        assert_eq!(
            dot_token,
            Token {
                token_type: TokenType::Dot,
                repr: ".",
                start: "move".len(),
                end: "move".len() + ".".len(),
                line: 1,
            }
        );

        assert_eq!(
            size_token,
            Token {
                token_type: TokenType::Dword,
                repr: "dw",
                start: "move".len() + ".".len(),
                end: "move".len() + ".".len() + "dw".len(),
                line: 1,
            }
        );

        assert_eq!(
            dest_token,
            Token {
                token_type: TokenType::D0,
                repr: "D0",
                start: "move".len() + ".".len() + "dw".len() + " ".len(),
                end: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len(),
                line: 1,
            }
        );

        assert_eq!(
            comma_token,
            Token {
                token_type: TokenType::Comma,
                repr: ",",
                start: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len(),
                end: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len() + ",".len(),
                line: 1,
            }
        );

        assert_eq!(
            source_token,
            Token {
                token_type: TokenType::A5,
                repr: "A5",
                start: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len() + ",".len(),
                //Baaahhh thats disgusting... Okay i think we are ready to refactor the tests...
                end: "move".len()
                    + ".".len()
                    + "dw".len()
                    + " ".len()
                    + "D0".len()
                    + ",".len()
                    + "A5".len(),
                line: 1,
            }
        );

        assert_eq!(
            tokenizer.next(),
            Token {
                token_type: TokenType::EndOfFile,
                repr: "",
                start: "move".len()
                    + ".".len()
                    + "dw".len()
                    + " ".len()
                    + "D0".len()
                    + ",".len()
                    + "A5".len(),
                end: "move".len()
                    + ".".len()
                    + "dw".len()
                    + " ".len()
                    + "D0".len()
                    + ",".len()
                    + "A5".len(),
                line: 1,
            }
        );
    }

    #[test]
    fn test_line_numbers() {
        let source = "hello\nworld";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Identifier,
                repr: "hello",
                start: 0,
                end: "hello".len(),
                line: 1,
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Identifier,
                repr: "world",
                start: "hello".len() + "\n".len(),
                end: "hello".len() + "\n".len() + "world".len(),
                line: 2,
            }
        )
    }

    #[test]
    fn test_move_immediate_value() {
        let source = "move.dw A0, #123456";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Move,
                repr: "move",
                start: 0,
                end: "move".len(),
                line: 1
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Dot,
                repr: ".",
                start: "move".len(),
                end: "move".len() + ".".len(),
                line: 1
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Dword,
                repr: "dw",
                start: "move".len() + ".".len(),
                end: "move".len() + ".".len() + "dw".len(),
                line: 1,
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::A0,
                repr: "A0",
                start: "move".len() + ".".len() + "dw".len() + " ".len(),
                end: "move".len() + ".".len() + "dw".len() + " ".len() + "A0".len(),
                line: 1
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::Comma,
                repr: ",",
                start: "move".len() + ".".len() + "dw".len() + " ".len() + "A0".len(),
                end: "move".len() + ".".len() + "dw".len() + " ".len() + "A0".len() + ",".len(),
                line: 1,
            }
        );

        let token = tokenizer.next();

        assert_eq!(
            token,
            Token {
                token_type: TokenType::DecimalNumber(123456),
                repr: "123456",
                start: "move".len()
                    + ".".len()
                    + "dw".len()
                    + " ".len()
                    + "A0".len()
                    + ",".len()
                    + " ".len()
                    + "#".len(),
                end: "move".len()
                    + ".".len()
                    + "dw".len()
                    + " ".len()
                    + "A0".len()
                    + ",".len()
                    + " ".len()
                    + "#123456".len(),
                line: 1
            }
        )
    }
}
