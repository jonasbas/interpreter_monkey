use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: char::MAX,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::MAX;
        } else {
            self.ch = self
                .input
                .chars()
                .enumerate()
                .find(|(index, _c)| index == &self.read_position)
                .unwrap_or((0, char::MAX))
                .1;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let char_string = self.ch.to_string();
        let token = match self.ch {
            '=' => Token {
                token_type: TokenType::ASSIGN,
                literal: char_string,
            },
            ';' => Token {
                token_type: TokenType::SEMICOLON,
                literal: char_string,
            },
            '(' => Token {
                token_type: TokenType::LPAREN,
                literal: char_string,
            },
            ')' => Token {
                token_type: TokenType::RPAREN,
                literal: char_string,
            },
            ',' => Token {
                token_type: TokenType::COMMA,
                literal: char_string,
            },
            '+' => Token {
                token_type: TokenType::PLUS,
                literal: char_string,
            },
            '{' => Token {
                token_type: TokenType::LBRACE,
                literal: char_string,
            },
            '}' => Token {
                token_type: TokenType::RBRACE,
                literal: char_string,
            },
            char::MAX => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            _ => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn next_token_test() {
        let string_to_parse = "=+(){},;";
        let expected = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
        ];

        let mut lexer = Lexer::new(string_to_parse);

        for x in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token.token_type, x);
        }
    }
}
