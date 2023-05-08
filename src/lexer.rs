use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: char::MAX,
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token_char = self.ch;
        let char_string = token_char.to_string();
        let token = match token_char {
            '=' => {
                if self.peek_char() == '=' {
                    let tmp_char = self.ch;
                    self.read_char();
                    Token {
                        token_type: TokenType::Equals,
                        literal: tmp_char.to_string() + self.ch.to_string().as_str(),
                    }
                } else {
                    Token {
                        token_type: TokenType::Assign,
                        literal: char_string,
                    }
                }
            }
            ';' => Token {
                token_type: TokenType::Semicolon,
                literal: char_string,
            },
            '(' => Token {
                token_type: TokenType::LParen,
                literal: char_string,
            },
            ')' => Token {
                token_type: TokenType::RParen,
                literal: char_string,
            },
            ',' => Token {
                token_type: TokenType::Comma,
                literal: char_string,
            },
            '+' => Token {
                token_type: TokenType::Plus,
                literal: char_string,
            },
            '{' => Token {
                token_type: TokenType::LBrace,
                literal: char_string,
            },
            '}' => Token {
                token_type: TokenType::RBrace,
                literal: char_string,
            },
            '-' => Token {
                token_type: TokenType::Minus,
                literal: char_string,
            },
            '/' => Token {
                token_type: TokenType::Slash,
                literal: char_string,
            },
            '!' => {
                if self.peek_char() == '=' {
                    let tmp_char = self.ch;
                    self.read_char();
                    Token {
                        token_type: TokenType::NotEquals,
                        literal: tmp_char.to_string() + self.ch.to_string().as_str(),
                    }
                } else {
                    Token {
                        token_type: TokenType::Bang,
                        literal: char_string,
                    }
                }
            }
            '*' => Token {
                token_type: TokenType::Asterisk,
                literal: char_string,
            },
            '<' => Token {
                token_type: TokenType::Lt,
                literal: char_string,
            },
            '>' => Token {
                token_type: TokenType::Gt,
                literal: char_string,
            },
            char::MAX => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
            _ => {
                if is_letter(token_char) {
                    return self.read_identifier();
                } else if is_digit(token_char) {
                    return Token {
                        token_type: TokenType::Int,
                        literal: self.read_number(),
                    };
                } else {
                    return Token {
                        token_type: TokenType::Illegal,
                        literal: char_string,
                    };
                }
            }
        };

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = char::MAX;
        } else {
            self.ch = self
                .input
                .char_indices()
                .find(|(index, _c)| index == &self.read_position)
                .map(|x| x.1)
                .unwrap_or(char::MAX);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let literal = &self.input[position..self.position];

        let token_type = match literal {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            _ => TokenType::Ident,
        };

        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            char::MAX
        } else {
            self.input
                .char_indices()
                .find(|x| x.0 == self.read_position)
                .map(|x| x.1)
                .unwrap_or(char::MAX)
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch) && ch != ';'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn next_token_test() {
        let string_to_parse = "=+(){},;";
        let expected = vec![
            TokenType::Assign,
            TokenType::Plus,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::Comma,
            TokenType::Semicolon,
        ];

        let mut lexer = Lexer::new(string_to_parse);

        for x in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token.token_type, x);
        }
    }

    #[test]
    fn bigger_token_test() {
        let string_to_parse = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;";

        let expected = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Equals, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEquals, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(string_to_parse);

        for x in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(x, &(token.token_type, token.literal.as_str()))
        }
    }
}
