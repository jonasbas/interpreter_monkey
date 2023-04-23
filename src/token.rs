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
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
