use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statements {
    LetStatement(Token, Identifier, Expressions),
    ReturnStatement(Token, Expressions),
    ExpressionStatement(Token, Expressions),
}

//Placeholder Expressions
#[derive(Debug, PartialEq)]
pub enum Expressions {
    Variant1,
    IntegerLiteral(Token, usize),
    PrefixExpression(Token, String, Box<Expressions>),
}

pub trait Node {
    fn token_literal(&self) -> String;
}

impl Node for Statements {
    fn token_literal(&self) -> String {
        match self {
            Statements::LetStatement(token, _, _) => token.literal.to_owned(),
            Statements::ReturnStatement(token, _) => token.literal.to_owned(),
            Statements::ExpressionStatement(token, _) => token.literal.to_owned(),
        }
    }
}

impl Node for Expressions {
    fn token_literal(&self) -> String {
        match self {
            Expressions::Variant1 => todo!(),
            Expressions::IntegerLiteral(token, _) => token.literal.to_owned(),
            Expressions::PrefixExpression(_, _, _) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        parser::Parser,
        statements::{Expressions, Node, Statements},
    };

    #[test]
    fn identifier_test() {
        let input = "foobar";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_programm().expect("Error parsing programm");
        parser.print_errors();

        assert_eq!(1, program.statements.len());

        let expression = &program.statements[0];
        if let Statements::ExpressionStatement(token, _) = expression {
            assert_eq!("foobar", token.literal);
            assert_eq!("foobar", expression.token_literal())
        } else {
            panic!("not an ExpressionStatement");
        }
    }

    #[test]
    fn integer_identifier_test() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_programm().unwrap();
        parser.print_errors();

        assert_eq!(1, program.statements.len());

        let expression = &program.statements[0];
        if let Statements::ExpressionStatement(_, exp) = expression {
            if let Expressions::IntegerLiteral(_, value) = exp {
                assert_eq!(&5, value);
                assert_eq!("5", exp.token_literal());
            } else {
                panic!("expression is not an integer literal");
            }
        } else {
            panic!("statement is not an expression statement");
        }
    }
}
