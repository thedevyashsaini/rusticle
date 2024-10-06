mod lexer;

use crate::lexer::lexer::Lexer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rusticle <filename.lin>");
        return;
    }
    
    let filename: &String = &args[1];

    let source: String = match fs::read_to_string(filename) {
        Ok(content) => content.to_string(),
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e);
            return;
        }
    };

    let mut lexer: Lexer = Lexer::new(source);
    let tokens: &Vec<lexer::token::Token> = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}