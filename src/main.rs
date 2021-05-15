use std::io;
use std::io::Write;

mod ast;
mod lexer;
mod parser;
mod program;

use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">> ";

fn main() {
    println!("Welcome to Rust's Ruby Interpreter");
    let mut running = true;
    while running {
        print!("{}", PROMPT);
        io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let lex = Lexer::new(line);
        for tok in lex {
            println!("{:?}:{}", tok.token_type, tok.literal);
            if tok.literal == "exit".to_string() { running = false };
        }
    }

    println!("Goodbye!");
}
