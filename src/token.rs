use crate::statements::Expressions;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //Identifiers and literals
    IDENT,
    INT,

    //Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOT_EQ,

    //Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl TokenType {
    pub fn prefix_parse(&self) -> Option<Expressions> {
        return match self {
            TokenType::ILLEGAL => Some(Expressions::Variant1),
            _ => None,
        };
    }
}
