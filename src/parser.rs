use crate::{
    ast::Programm,
    lexer::Lexer,
    statements::{Expressions, Identifier, Statements},
    token::{Token, TokenType},
};

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_programm(&mut self) -> Option<Programm> {
        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::EOF {
            let statement = self.parse_statement().expect("no statement found"); //TODO: No unwrap
            statements.push(statement);
            self.next_token();
        }

        Some(Programm { statements })
    }

    fn parse_statement(&mut self) -> Option<Statements> {
        let token = &self.cur_token;
        match token.token_type {
            TokenType::ILLEGAL => todo!(),
            TokenType::EOF => todo!(),
            TokenType::IDENT => todo!(),
            TokenType::INT => todo!(),
            TokenType::ASSIGN => todo!(),
            TokenType::PLUS => todo!(),
            TokenType::MINUS => todo!(),
            TokenType::BANG => todo!(),
            TokenType::ASTERISK => todo!(),
            TokenType::SLASH => todo!(),
            TokenType::LT => todo!(),
            TokenType::GT => todo!(),
            TokenType::EQ => todo!(),
            TokenType::NOT_EQ => todo!(),
            TokenType::COMMA => todo!(),
            TokenType::SEMICOLON => todo!(),
            TokenType::LPAREN => todo!(),
            TokenType::RPAREN => todo!(),
            TokenType::LBRACE => todo!(),
            TokenType::RBRACE => todo!(),
            TokenType::FUNCTION => todo!(),
            TokenType::LET => self.parse_let_statement(),
            TokenType::TRUE => todo!(),
            TokenType::FALSE => todo!(),
            TokenType::IF => todo!(),
            TokenType::ELSE => todo!(),
            TokenType::RETURN => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statements> {
        let let_token = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let identifier = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.to_owned(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statements::LetStatement(
            let_token,
            identifier,
            Expressions::Variant1,
        ))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer::Lexer,
        statements::{Node, Statements},
    };

    #[test]
    fn test_let_statement() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
            ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let programm = parser.parse_programm();

        if let None = programm {
            panic!("programm is none!");
        }

        let programm = programm.unwrap();
        assert!(programm.statements.len() == 3);

        let expected = vec!["x", "y", "foobar"];

        for (index, statement) in programm.statements.iter().enumerate() {
            if let Statements::LetStatement(_, identifier, _) = statement {
                assert!(statement.token_literal() == "let");

                let name = expected[index].to_string();
                assert!(identifier.value == name);
                assert!(identifier.token_literal() == name);
            } else {
                panic!("statement is not a let statement");
            }
        }
    }
}
