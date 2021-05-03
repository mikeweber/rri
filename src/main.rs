mod lexer;

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

fn main() {
    println!("Welcome to Rust's Ruby Interpreter");
    let mut running = true;
    while running {
        println!("{}", PROMPT);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut lex = Lexer::new(line);
        for tok in lex {
            println!("{:?}:{}", tok.token_type, tok.literal);
            if tok.literal == "exit".to_string() { running = false };
        }
    }

    println!("Goodbye!");
}
