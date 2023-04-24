use text_io::try_read;

use crate::{lexer::Lexer, token::TokenType};
const PROMPT: &str = ">>";

pub fn start_repl() {
    println!("Monkey REPL v.0.1");
    println!("To exit CTRL-C");
    loop {
        print!("{}", PROMPT);

        let read_line: Result<String, text_io::Error> = try_read!("{}\n");

        if let Err(x) = read_line {
            println!("Error reading input: {:?}", x);
            continue;
        }

        if let Ok(input) = read_line {
            let mut lexer = Lexer::new(input.as_str());

            loop {
                let token = lexer.next_token();
                if token.token_type == TokenType::EOF {
                    println!("Encountered EOF.");
                    break;
                }

                println!("{:?}", token);
            }
        }
    }
}
