use crate::token::Token;

#[derive(Debug)]
pub enum Statements {
    LetStatement(Token, Identifier, Expressions),
    Statement,
}

//Placeholder Expressions
#[derive(Debug)]
pub enum Expressions {
    Variant1,
    Variant2,
}

pub trait Node {
    fn token_literal(&self) -> String;
}

impl Node for Statements {
    fn token_literal(&self) -> String {
        println!("In Statement token_literal");
        match self {
            Statements::LetStatement(token, _, _) => token.literal.to_owned(),
            Statements::Statement => todo!("Placeholder"),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}
