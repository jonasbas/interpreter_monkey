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

//helper function
fn get_precedences(token_type: TokenType) -> u8 {
    match token_type {
        TokenType::Equals => EQUALS,
        TokenType::NotEquals => EQUALS,
        TokenType::Lt => LESSGREATER,
        TokenType::Gt => LESSGREATER,
        TokenType::Plus => SUM,
        TokenType::Minus => SUM,
        TokenType::Slash => PRODUCT,
        TokenType::Asterisk => PRODUCT,
        _ => LOWEST,
    }
}

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
        println!("Token to parse : {:?}", token);
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
    fn parse_expression(&mut self, precedence: u8) -> Option<Expressions> {
        let prefix = self.prefix_parse();
        if prefix.is_none() {
            return None;
        }
        let mut left_exp = prefix;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            println!("left exp : {:?}", left_exp);
            let infix = self.infix_parse(left_exp.clone().unwrap());
            println!("parsed infix : {:?}", infix);

            if infix.is_none() {
                return left_exp;
            }

            // Maybe necessary later on
            // self.next_token();

            left_exp = infix;
        }

        left_exp
    }

    fn parse_integer_literal(&self) -> Option<Expressions> {
        let cur_token = self.cur_token.clone();
        let value = cur_token.literal.parse();
        if value.is_err() {
            return None;
        }

        Some(Expressions::IntegerLiteral(cur_token, value.unwrap()))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expressions> {
        let cur_token = self.cur_token.clone();
        let operator = cur_token.literal.to_owned();
        self.next_token();

        //TODO: Safe unwrap
        let exp = self.parse_expression(PREFIX).expect("Should not fail");

        Some(Expressions::PrefixExpression(
            cur_token,
            operator,
            Box::new(exp),
        ))
    }

    fn parse_boolean(&self) -> Option<Expressions> {
        Some(Expressions::BooleanExpression(
            self.cur_token.clone(),
            self.cur_token_is(TokenType::True),
        ))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expressions> {
        self.next_token();

        let exp = self.parse_expression(LOWEST);

        if self.expect_peek(TokenType::RParen).is_err() {
            None
        } else {
            exp
        }
    }

    fn prefix_parse(&mut self) -> Option<Expressions> {
        match self.cur_token.token_type {
            TokenType::Illegal => Some(Expressions::Variant1),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang => self.parse_prefix_expression(),
            TokenType::Minus => self.parse_prefix_expression(),
            TokenType::True => self.parse_boolean(),
            TokenType::False => self.parse_boolean(),
            TokenType::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Expressions) -> Option<Expressions> {
        //Remember this is not like in the book
        self.next_token();
        let cur_token = self.cur_token.clone();
        let operator = cur_token.literal.to_owned();
        let cur_precedence = self.cur_precedence();
        self.next_token();

        //TODO: safe unwrap
        let right = self.parse_expression(cur_precedence).unwrap();

        Some(Expressions::InfixExpression(
            cur_token,
            Box::new(left),
            operator,
            Box::new(right),
        ))
    }

    fn infix_parse(&mut self, left: Expressions) -> Option<Expressions> {
        match self.peek_token.token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Asterisk
            | TokenType::Equals
            | TokenType::NotEquals
            | TokenType::Lt
            | TokenType::Gt => self.parse_infix_expression(left),
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

    fn peek_precedence(&self) -> u8 {
        get_precedences(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> u8 {
        get_precedences(self.cur_token.token_type)
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
                if let Expressions::PrefixExpression(_, op, right) = exp {
                    assert_eq!(inputs.1, op.as_str());
                    assert!(test_integer_literal(right.clone(), inputs.2))
                } else {
                    panic!("Should be PrefixExpression");
                }
            } else {
                panic!("Should be Expression Statement");
            }
        }
    }

    #[test]
    fn parsing_infix_expressions_test() {
        let test_inputs = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for input in test_inputs.iter() {
            let lexer = Lexer::new(input.0);
            let mut parser = Parser::new(lexer);

            let program = parser.parse_programm().unwrap();
            parser.print_errors();

            assert_eq!(1, program.statements.len());
            let expression = &program.statements[0];

            if let Statements::Expression(_, exp) = expression {
                if let Expressions::InfixExpression(_, left, op, right) = exp {
                    assert!(test_integer_literal(left.clone(), input.1));
                    assert_eq!(input.2, op);
                    assert!(test_integer_literal(right.clone(), input.3));
                }
            }
        }
    }

    //helper
    fn test_integer_literal(expression: Box<Expressions>, value: usize) -> bool {
        if let Expressions::IntegerLiteral(_, val) = *expression {
            if value != val {
                return false;
            }
            if !expression
                .token_literal()
                .to_owned()
                .eq(&format!("{}", value))
            {
                return false;
            }
            true
        } else {
            false
        }
    }
}
