//NOTE: Achievement for this project:
//      - Get a working Assembler that can produce executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...
//      - create a lib instead of a binary, i usually always create binaries but i should learn to work more with libs :)
#[derive(Debug, Eq, PartialEq)]
enum Token<'a> {
    Identifier(TokenInfo<'a>),
    Number(TokenInfo<'a>, i32),
}

impl<'a> Token<'a> {
    fn new_identifier(repr: &'a str, start: usize, end: usize) -> Self {
        Self::Identifier(TokenInfo::new(repr, start, end))
    }

    fn new_number(repr: &'a str, start: usize, end: usize) -> Self {
        //NOTE: Due to the way we parse numbers i am pretty sure that i can just unwrap here! and do not need to
        //      care about the error state! But i might be wrong so lets add a panic in case anything goes haywire...
        Self::Number(
            TokenInfo::new(repr, start, end),
            repr.parse().unwrap_or_else(|_| {
                panic!("ERROR: Scanned Item was apparently not a value {}", repr)
            }),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TokenInfo<'a> {
    repr: &'a str,
    start: usize,
    end: usize,
}

impl<'a> TokenInfo<'a> {
    fn new(repr: &'a str, start: usize, end: usize) -> Self {
        Self { repr, start, end }
    }
}

struct Tokenizer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn advance(&mut self) {
        if self.position < self.source.len() {
            self.position += 1;
        }
    }

    fn digest_identifier(&mut self) -> (usize, usize) {
        let start = self.position;
        while let Some(ch) = self.source.chars().nth(self.position) {
            if ch.is_alphabetic() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }

    fn digest_number(&mut self) -> (usize, usize) {
        let start = self.position;

        while let Some(ch) = self.source.chars().nth(self.position) {
            if ch.is_ascii_digit() || ch == '-' {
                self.advance();
            } else {
                break;
            }
        }

        (start, self.position)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.source.len() {
                return None;
            }

            match self.source.chars().nth(self.position) {
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    let (start, end) = self.digest_identifier();
                    return Some(Token::new_identifier(&self.source[start..end], start, end));
                }
                Some(ch) if ch.is_ascii_digit() || ch == '-' => {
                    let (start, end) = self.digest_number();
                    return Some(Token::new_number(&self.source[start..end], start, end));
                }
                Some(_) => self.advance(),
                _ => unreachable!(),
            };
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
            Token::Identifier(TokenInfo {
                repr: "move",
                start: 0,
                end: source.len(),
            })
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
            Token::Identifier(TokenInfo {
                repr: "move",
                start: 0,
                end: "move".len(),
            })
        );

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "dest",
                start: "move".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len(),
            })
        );

        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Identifier(TokenInfo {
                repr: "src",
                start: "move".len() + " ".len() + "dest".len() + " ".len(),
                end: "move".len() + " ".len() + "dest".len() + " ".len() + "src".len(),
            })
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_decimal_numbers() {
        let source = "1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Number(
                TokenInfo {
                    repr: "1337",
                    start: 0,
                    end: source.len(),
                },
                1337
            )
        );

        assert_eq!(tokenizer.position, source.len());
    }

    #[test]
    fn test_negative_decimal_numbers() {
        let source = "-1337";

        let mut tokenizer = Tokenizer::new(source);
        let token = tokenizer.next();

        assert_eq!(
            token.unwrap(),
            Token::Number(
                TokenInfo {
                    repr: "-1337",
                    start: 0,
                    end: source.len(),
                },
                -1337
            )
        );

        assert_eq!(tokenizer.position, source.len());
    }
}
