#![allow(dead_code)]
use crate::repl::start_repl;

mod ast;
mod error;
mod lexer;
mod parser;
mod repl;
mod statements;
mod token;

fn main() {
    start_repl();
}
