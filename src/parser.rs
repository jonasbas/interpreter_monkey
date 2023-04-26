use crate::{
    ast::Programm,
    error::ParsingError,
    lexer::Lexer,
    statements::{Expressions, Identifier, Statements},
    token::{Token, TokenType},
};

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<ParsingError>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors = vec![];

        Parser {
            lexer,
            cur_token,
            peek_token,
            errors,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_programm(&mut self) -> Option<Programm> {
        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();

            if let Err(e) = statement {
                self.errors.push(e);
            } else {
                statements.push(statement.expect("the world is ending"));
            }

            self.next_token();
        }

        Some(Programm { statements })
    }

    fn parse_statement(&mut self) -> Result<Statements, ParsingError> {
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
            TokenType::RETURN => self.parse_return_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statements, ParsingError> {
        let let_token = self.cur_token.clone();
        self.expect_peek(TokenType::IDENT)?;

        let identifier = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.to_owned(),
        };

        self.expect_peek(TokenType::ASSIGN)?;

        //TODO: expressions
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Statements::LetStatement(
            let_token,
            identifier,
            Expressions::Variant1,
        ))
    }

    fn parse_return_statement(&mut self) -> Result<Statements, ParsingError> {
        let cur_token = self.cur_token.clone();

        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Statements::ReturnStatement(
            cur_token,
            Expressions::Variant1,
        ))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParsingError> {
        if self.peek_token_is(token_type) {
            self.next_token();
            return Ok(());
        }

        Err(ParsingError(format!(
            "Expected {:?}, found {}",
            token_type,
            self.peek_token.literal.to_owned()
        )))
    }

    fn print_errors(&self) {
        for error in self.errors.iter() {
            println!("{}", error.0);
        }
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
        assert_eq!(programm.statements.len(), 3);

        let expected = vec!["x", "y", "foobar"];

        for (index, statement) in programm.statements.iter().enumerate() {
            if let Statements::LetStatement(_, identifier, _) = statement {
                assert_eq!(statement.token_literal(), "let");

                let name = expected[index].to_string();
                assert_eq!(identifier.value, name);
                assert_eq!(identifier.token_literal(), name);
            } else {
                panic!("statement is not a let statement");
            }
        }
    }

    #[test]
    fn test_return_statemetn() {
        let input = "
            return 5;
            return 10;
            return 993322;
            ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let programm = parser.parse_programm();

        parser.print_errors();

        let programm = programm.unwrap();

        assert_eq!(programm.statements.len(), 3);

        for statement in programm.statements.iter() {
            if let Statements::ReturnStatement(token, _) = statement {
                assert_eq!(token.literal, "return");
            } else {
                panic!("expected return statement");
            }
        }
    }
}
