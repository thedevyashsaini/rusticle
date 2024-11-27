use std::fs;
use crate::commands::Command;

use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::print_ast::AstPrinter;
use crate::utils::token::Token;

pub struct Interpret {
    pub filename: String,
    pub tokens: Option<bool>,
    pub ast: Option<bool>,
    pub ast_raw: Option<bool>,
}

impl Command for Interpret {
    fn execute(&self) {
        let source: String = match fs::read_to_string(self.filename.clone()) {
            Ok(content) => content.to_string(),
            Err(e) => {
                eprintln!("Error reading {}: {}", self.filename, e);
                return;
            }
        };

        interpret(source, self);
    }
}

fn begin(name: &str) -> String {
    format!("\n\n----------{} BEGIN----------", name)
}

fn end(name: &str) -> String {
    format!("----------{} END----------\n", name)
}

fn interpret(source: String, props: &Interpret) {

    // initialize lexer with source code
    let mut lexer: Lexer = Lexer::new(source);

    // tokenize source code 
    let tokens: &Vec<Token> = match lexer.scan_tokens() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error scanning tokens: {}", e);
            return;
        }
    };

    // print tokens if flag is set
    if props.tokens.unwrap_or(false) {
        println!("{}", begin("TOKENS"));
        for token in tokens {
            println!("> {:?}", token);
        }
        println!("{}", end("TOKENS"));
    }

    // initialize parser with tokens
    let mut parser: Parser = Parser::new(tokens.to_vec());

    // parse tokens
    let statements: Vec<_> = parser.parse();

    // print ast if flag is set
    if props.ast.unwrap_or(false) {
        println!("{}", begin("AST"));
        let mut printer: AstPrinter = AstPrinter::new();
        printer.print(&statements);
        println!("{}", end("AST"));
    }

    // print raw statements if flag is set
    if props.ast_raw.unwrap_or(false) {
        println!("{}", begin("STATEMENTS"));
        for stmt in &statements {
            println!("> {:?}", stmt);
        }
        println!("{}", end("STATEMENTS"));
    }

    // interpret the statements
    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements.clone());
}