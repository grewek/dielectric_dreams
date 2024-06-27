//NOTE: Achievement for this project:
//      - Get a working Assembler that can produces executable code for my architecture
//      - Trying to get better in TDD but it's usage shouldn't be as dogmatic as many people practice it...

#[derive(Debug, Eq, PartialEq)]
enum Token<'a> {
    Identifier(TokenInfo<'a>),
}

#[derive(Debug, Eq, PartialEq)]
struct TokenInfo<'a> {
    repr: &'a str,
    start: usize,
    end: usize,
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
        while self.position < self.source.len()
            && self
                .source
                .chars()
                .nth(self.position)
                .unwrap()
                .is_alphabetic()
        {
            self.advance();
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

            if self
                .source
                .chars()
                .nth(self.position)
                .unwrap()
                .is_alphabetic()
            {
                let (start, end) = self.digest_identifier();

                return Some(Token::Identifier(TokenInfo {
                    repr: &self.source[start..end],
                    start,
                    end,
                }));
            }

            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
