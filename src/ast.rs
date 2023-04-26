use crate::statements::{Node, Statements};

pub struct Programm {
    pub statements: Vec<Statements>,
}

impl Node for Programm {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
