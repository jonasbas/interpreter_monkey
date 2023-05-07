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
    Variant2,
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
    use crate::{lexer::Lexer, parser::Parser, statements::Statements};

    #[test]
    fn identifier_test() {
        let input = "foobar";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_programm().expect("Error parsing programm");
        parser.print_errors();

        assert_eq!(1, program.statements.len());

        let test = &program.statements[0];
        if let Statements::ExpressionStatement(token, _) = test {
            assert_eq!("foobar", token.literal);
        } else {
            panic!("not an ExpressionStatement");
        }
    }
}
