use std::str;
//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
#[derive(Debug, Eq, PartialEq)]
enum Operator {
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
}

#[derive(Debug, Eq, PartialEq)]
enum Mnemoics {
    //Keywords
    Move,
    Lea,
    //Opcode Size
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
}

#[derive(Debug, Eq, PartialEq)]
enum Token<'a> {
    Identifier(TokenInfo<'a>),
    Keyword(Mnemoics, TokenInfo<'a>),
    DecimalNumber(TokenInfo<'a>, i32),
    HexNumber(TokenInfo<'a>, u32),
    BinaryNumber(TokenInfo<'a>, u32),
    Operator(Operator, TokenInfo<'a>),
}

impl<'a> Token<'a> {
    fn new_identifier(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        //NOTE: I will __not__ implement string interning yet...as i just started a course
        //      on performance oriented programming and i want to get better at profiling stuff
        //      i should really stay away from doing any optimization...
        let repr: &str = str::from_utf8(repr).unwrap();
        let possible_keyword = repr.to_lowercase();

        let mnemomic = match possible_keyword.as_str() {
            "move" => Some(Mnemoics::Move),
            "lea" => Some(Mnemoics::Lea),
            "b" => Some(Mnemoics::Byte),
            "w" => Some(Mnemoics::Word),
            "dw" => Some(Mnemoics::Dword),

            "d0" => Some(Mnemoics::D0),
            "d1" => Some(Mnemoics::D1),
            "d2" => Some(Mnemoics::D2),
            "d3" => Some(Mnemoics::D3),
            "d4" => Some(Mnemoics::D4),
            "d5" => Some(Mnemoics::D5),
            "d6" => Some(Mnemoics::D6),
            "d7" => Some(Mnemoics::D7),
            "d8" => Some(Mnemoics::D8),
            "d9" => Some(Mnemoics::D9),
            "d10" => Some(Mnemoics::D10),
            "d11" => Some(Mnemoics::D11),
            "d12" => Some(Mnemoics::D12),
            "d13" => Some(Mnemoics::D13),
            "d14" => Some(Mnemoics::D14),
            "d15" => Some(Mnemoics::D15),

            "a0" => Some(Mnemoics::A0),
            "a1" => Some(Mnemoics::A1),
            "a2" => Some(Mnemoics::A2),
            "a3" => Some(Mnemoics::A3),
            "a4" => Some(Mnemoics::A4),
            "a5" => Some(Mnemoics::A5),
            "a6" => Some(Mnemoics::A6),
            "a7" => Some(Mnemoics::A7),
            "a8" => Some(Mnemoics::A8),
            "a9" => Some(Mnemoics::A9),
            "a10" => Some(Mnemoics::A10),
            "a11" => Some(Mnemoics::A11),
            "a12" => Some(Mnemoics::A12),
            "a13" => Some(Mnemoics::A13),
            "a14" => Some(Mnemoics::A14),
            "a15" => Some(Mnemoics::A15),

            _ => None,
        };

        if let Some(keyword) = mnemomic {
            return Self::Keyword(keyword, TokenInfo::new(repr, start, end, line));
        }

        Self::Identifier(TokenInfo::new(repr, start, end, line))
    }

    fn new_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        //NOTE: Due to the way we parse numbers i am pretty sure that i can just unwrap here! and do not need to
        //      care about the error state! But i might be wrong so lets add a panic in case anything goes haywire...
        let repr = str::from_utf8(repr).unwrap();
        Self::DecimalNumber(
            TokenInfo::new(repr, start, end, line),
            repr.parse().unwrap_or_else(|_| {
                panic!("ERROR: Scanned Item was apparently not a value {}", repr)
            }),
        )
    }

    fn new_binary_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        let repr = str::from_utf8(repr).unwrap();

        Self::BinaryNumber(
            TokenInfo::new(repr, start, end, line),
            u32::from_str_radix(repr, 2).unwrap_or_else(|_| {
                panic!(
                    "Error: Scanned item could not be converted into a hexadecimal value: {}",
                    repr
                )
            }),
        )
    }
    fn new_hex_number(repr: &'a [u8], start: usize, end: usize, line: usize) -> Self {
        let repr = str::from_utf8(repr).unwrap();
        Self::HexNumber(
            TokenInfo::new(repr, start, end, line),
            u32::from_str_radix(repr, 16).unwrap_or_else(|_| {
                panic!(
                    "Error: Scanned Item could not be converted into a hexadecimal value: {}",
                    repr
                );
            }),
        )
    }

    fn new_operator(
        operator: Operator,
        repr: &'a [u8],
        start: usize,
        end: usize,
        line: usize,
    ) -> Self {
        let repr = str::from_utf8(repr).unwrap();
        Self::Operator(operator, TokenInfo::new(repr, start, end, line))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TokenInfo<'a> {
    repr: &'a str,
    start: usize,
    end: usize,
    line: usize,
}

impl<'a> TokenInfo<'a> {
    fn new(repr: &'a str, start: usize, end: usize, line: usize) -> Self {
        Self {
            repr,
            start,
            end,
            line,
        }
    }
}

struct Tokenizer<'a> {
    source: &'a [u8],
    line: usize,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
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

    fn digest_operator(&mut self) -> (Operator, usize, usize) {
        let start = self.position;

        //TODO(Kay): From?
        let operator = match self.source.get(self.position) {
            Some(b'#') => Operator::PoundSign,
            Some(b'%') => Operator::Percent,
            Some(b'$') => Operator::DollarSign,
            Some(b'(') => Operator::OpenParen,
            Some(b')') => Operator::CloseParen,
            Some(b',') => Operator::Comma,
            Some(b'.') => Operator::Dot,
            Some(b':') => Operator::Colon,
            Some(b'+') => Operator::Plus,
            Some(b'-') => Operator::Minus,
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

    fn next(&mut self) -> Option<Token<'a>> {
        self.digest_whitespace();

        if self.position == self.source.len() {
            return None;
        }

        match self.source.get(self.position) {
            Some(ch) if ch.is_ascii_alphabetic() || *ch == b'_' => {
                let (start, end) = self.digest_identifier();
                return Some(Token::new_identifier(
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                ));
            }
            Some(ch)
                if *ch == b'#'
                    && (self.peek().unwrap().is_ascii_digit() || *self.peek().unwrap() == b'-') =>
            {
                self.advance();
                let (start, end) = self.digest_decmial_number();
                return Some(Token::new_number(
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                ));
            }
            Some(ch) if *ch == b'$' && Tokenizer::is_hexadecimal(*self.peek().unwrap()) => {
                self.advance();

                let (start, end) = self.digest_hex_number();
                return Some(Token::new_hex_number(
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                ));
            }
            Some(ch) if *ch == b'%' && Tokenizer::is_binary(*self.peek().unwrap()) => {
                self.advance();
                let (start, end) = self.digest_binary_number();
                return Some(Token::new_binary_number(
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                ));
            }
            Some(_) => {
                //NOTE: Scanner is desperate it does not know what the next symbol is so it __must__
                //      be a operator!
                let (operator, start, end) = self.digest_operator();

                return Some(Token::new_operator(
                    operator,
                    &self.source[start..end],
                    start,
                    end,
                    self.line,
                ));
            }
            _ => unreachable!(),
        };
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

        assert_eq!(token, None);
    }

    #[test]
    fn test_discards_whitespace() {
        let whitespace = "   ";
        let mut tokenizer = Tokenizer::new(whitespace);

        let token = tokenizer.next();

        assert_eq!(token, None);
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
        assert_eq!(token, None);
        assert_eq!(tokenizer.position, whitespace.len());
    }

    #[test]
    fn test_generate_identifiers() {
        let source = "move";
        let mut tokenizer = Tokenizer::new(source);

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Keyword(
                Mnemoics::Move,
                TokenInfo {
                    repr: "move",
                    start: 0,
                    end: source.len(),
                    line: 1,
                }
            )
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_generate_identifiers_underscore() {
        let source = "_move";
        let mut tokenizer = Tokenizer::new(source);

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "_move",
                start: 0,
                end: source.len(),
                line: 1,
            })
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_identifier_whitespace_mix() {
        let source = "move dest src";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Keyword(
                Mnemoics::Move,
                TokenInfo {
                    repr: "move",
                    start: 0,
                    end: "move".len(),
                    line: 1,
                }
            )
        );

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "dest",
                start: "move".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len(),
                line: 1,
            })
        );

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "src",
                start: "move".len() + " ".len() + "dest".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len() + " ".len() + "src".len(),
                line: 1,
            })
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_decimal_numbers() {
        let source = "#1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::DecimalNumber(
                TokenInfo {
                    repr: "1337",
                    start: 1,
                    end: source.len(),
                    line: 1,
                },
                1337
            )
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_negative_decimal_numbers() {
        let source = "#-1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::DecimalNumber(
                TokenInfo {
                    repr: "-1337",
                    start: 1,
                    end: source.len(),
                    line: 1,
                },
                -1337
            )
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn parse_operators() {
        let source = "#%$(),.:+-";

        let mut tokenizer = Tokenizer::new(source);

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::PoundSign,
                TokenInfo {
                    repr: "#",
                    start: 0,
                    end: 1,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Percent,
                TokenInfo {
                    repr: "%",
                    start: 1,
                    end: 2,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::DollarSign,
                TokenInfo {
                    repr: "$",
                    start: 2,
                    end: 3,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::OpenParen,
                TokenInfo {
                    repr: "(",
                    start: 3,
                    end: 4,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::CloseParen,
                TokenInfo {
                    repr: ")",
                    start: 4,
                    end: 5,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Comma,
                TokenInfo {
                    repr: ",",
                    start: 5,
                    end: 6,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Dot,
                TokenInfo {
                    repr: ".",
                    start: 6,
                    end: 7,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Colon,
                TokenInfo {
                    repr: ":",
                    start: 7,
                    end: 8,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Plus,
                TokenInfo {
                    repr: "+",
                    start: 8,
                    end: 9,
                    line: 1,
                }
            )
        );

        assert_eq!(
            tokenizer.next().unwrap(),
            Token::Operator(
                Operator::Minus,
                TokenInfo {
                    repr: "-",
                    start: 9,
                    end: 10,
                    line: 1,
                }
            )
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
            token.unwrap(),
            Token::HexNumber(
                TokenInfo {
                    repr: "65BBCCDD",
                    start: 1,
                    end: source.len(),
                    line: 1,
                },
                1706806493
            )
        );
    }

    #[test]
    fn test_binary_numbers() {
        let source = "%10110000";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::BinaryNumber(
                TokenInfo {
                    repr: "10110000",
                    start: 1,
                    end: source.len(),
                    line: 1,
                },
                176
            )
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
            opcode_token.unwrap(),
            Token::Keyword(
                Mnemoics::Move,
                TokenInfo {
                    repr: "move",
                    start: 0,
                    end: "move".len(),
                    line: 1,
                }
            )
        );

        assert_eq!(
            dot_token.unwrap(),
            Token::Operator(
                Operator::Dot,
                TokenInfo {
                    repr: ".",
                    start: "move".len(),
                    end: "move".len() + ".".len(),
                    line: 1,
                }
            )
        );

        assert_eq!(
            size_token.unwrap(),
            Token::Keyword(
                Mnemoics::Dword,
                TokenInfo {
                    repr: "dw",
                    start: "move".len() + ".".len(),
                    end: "move".len() + ".".len() + "dw".len(),
                    line: 1,
                }
            )
        );

        assert_eq!(
            dest_token.unwrap(),
            Token::Keyword(
                Mnemoics::D0,
                TokenInfo {
                    repr: "D0",
                    start: "move".len() + ".".len() + "dw".len() + " ".len(),
                    end: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len(),
                    line: 1,
                }
            )
        );

        assert_eq!(
            comma_token.unwrap(),
            Token::Operator(
                Operator::Comma,
                TokenInfo {
                    repr: ",",
                    start: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len(),
                    end: "move".len() + ".".len() + "dw".len() + " ".len() + "D0".len() + ",".len(),
                    line: 1,
                }
            )
        );

        assert_eq!(
            source_token.unwrap(),
            Token::Keyword(
                Mnemoics::A5,
                TokenInfo {
                    repr: "A5",
                    start: "move".len()
                        + ".".len()
                        + "dw".len()
                        + " ".len()
                        + "D0".len()
                        + ",".len(),
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
            )
        );

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_line_numbers() {
        let source = "hello\nworld";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "hello",
                start: 0,
                end: "hello".len(),
                line: 1,
            })
        );

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "world",
                start: "hello".len() + "\n".len(),
                end: "hello".len() + "\n".len() + "world".len(),
                line: 2,
            })
        )
    }
}
