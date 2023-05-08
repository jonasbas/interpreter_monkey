use crate::{
    ast::Programm,
    error::ParsingError,
    lexer::Lexer,
    statements::{Expressions, Identifier, Statements},
    token::{Token, TokenType},
};

const BLANK: u8 = 0;
const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<ParsingError>,
}

//Main impl
impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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

    pub fn parse_programm(&mut self) -> Option<Programm> {
        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::Eof {
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
}

//Statements
impl Parser {
    fn parse_statement(&mut self) -> Result<Statements, ParsingError> {
        let token = &self.cur_token;
        match token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statements, ParsingError> {
        let let_token = self.cur_token.clone();
        self.expect_peek(TokenType::Ident)?;

        let identifier = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.to_owned(),
        };

        self.expect_peek(TokenType::Assign)?;

        //TODO: expressions
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Statements::Let(
            let_token,
            identifier,
            Expressions::Variant1,
        ))
    }

    fn parse_return_statement(&mut self) -> Result<Statements, ParsingError> {
        let cur_token = self.cur_token.clone();

        self.next_token();

        //TODO: expressions
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Statements::Return(cur_token, Expressions::Variant1))
    }

    fn parse_expression_statement(&mut self) -> Result<Statements, ParsingError> {
        let cur_token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        //TODO: no unwrap
        Ok(Statements::Expression(
            cur_token,
            expression.unwrap_or(Expressions::Variant1),
        ))
    }
}

//Expressions
impl Parser {
    fn parse_expression(&mut self, _lowest: u8) -> Option<Expressions> {
        self.prefix_parse()
    }

    pub fn parse_integer_literal(&self) -> Option<Expressions> {
        let cur_token = self.cur_token.clone();
        let value = cur_token.literal.parse();
        if value.is_err() {
            return None;
        }

        Some(Expressions::IntegerLiteral(cur_token, value.unwrap()))
    }

    pub fn prefix_parse(&mut self) -> Option<Expressions> {
        match self.cur_token.token_type {
            TokenType::Illegal => Some(Expressions::Variant1),
            TokenType::Int => self.parse_integer_literal(),
            _ => None,
        }
    }
}

//Helper
impl Parser {
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
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

    pub fn print_errors(&self) {
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

        if programm.is_none() {
            panic!("programm is none!");
        }

        let programm = programm.unwrap();
        assert_eq!(programm.statements.len(), 3);

        let expected = vec!["x", "y", "foobar"];

        for (index, statement) in programm.statements.iter().enumerate() {
            if let Statements::Let(_, identifier, _) = statement {
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
            if let Statements::Return(token, _) = statement {
                assert_eq!(token.literal, "return");
            } else {
                panic!("expected return statement");
            }
        }
    }

    #[test]
    fn parsing_prefix_expression_test() {
        let test_inputs = vec![("!5", "!", 5), ("-15", "-", 15)];

        for inputs in test_inputs.iter() {
            let lexer = Lexer::new(inputs.0);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_programm().unwrap();
            parser.print_errors();

            assert_eq!(1, program.statements.len());
            let statement = &program.statements[0];
            if let Statements::Expression(_, exp) = statement {
                if let Expressions::PrefixExpression(_, op, _) = exp {
                    assert_eq!(inputs.1, op.as_str());
                }
            }
        }
    }
}
