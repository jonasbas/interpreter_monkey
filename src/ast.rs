use crate::statements::{Node, Statements};

pub struct Programm {
    pub statements: Vec<Statements>,
}

impl Node for Programm {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
