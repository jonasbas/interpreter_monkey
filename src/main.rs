use crate::repl::start_repl;

mod lexer;
mod repl;
mod token;

fn main() {
    println!("Monkey REPL v.0.1");
    start_repl();
}
