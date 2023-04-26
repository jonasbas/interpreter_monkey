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
                        token_type: TokenType::EQ,
                        literal: tmp_char.to_string() + self.ch.to_string().as_str(),
                    }
                } else {
                    Token {
                        token_type: TokenType::ASSIGN,
                        literal: char_string,
                    }
                }
            }
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
            '-' => Token {
                token_type: TokenType::MINUS,
                literal: char_string,
            },
            '/' => Token {
                token_type: TokenType::SLASH,
                literal: char_string,
            },
            '!' => {
                if self.peek_char() == '=' {
                    let tmp_char = self.ch;
                    self.read_char();
                    Token {
                        token_type: TokenType::NOT_EQ,
                        literal: tmp_char.to_string() + self.ch.to_string().as_str(),
                    }
                } else {
                    Token {
                        token_type: TokenType::BANG,
                        literal: char_string,
                    }
                }
            }
            '*' => Token {
                token_type: TokenType::ASTERISK,
                literal: char_string,
            },
            '<' => Token {
                token_type: TokenType::LT,
                literal: char_string,
            },
            '>' => Token {
                token_type: TokenType::GT,
                literal: char_string,
            },
            char::MAX => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            _ => {
                if is_letter(token_char) {
                    return self.read_identifier();
                } else if is_digit(token_char) {
                    return Token {
                        token_type: TokenType::INT,
                        literal: self.read_number(),
                    };
                } else {
                    return Token {
                        token_type: TokenType::ILLEGAL,
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
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            _ => TokenType::IDENT,
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
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9' && ch != ';'
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
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::INT, "10"),
            (TokenType::NOT_EQ, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(string_to_parse);

        for x in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(x, &(token.token_type, token.literal.as_str()))
        }
    }
}
