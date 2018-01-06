extern crate lox;
extern crate rprompt;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

use lox::parser::Parser;
use lox::ast::AstPrint;
use lox::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_path) => interpret_file(file_path),
        None => repl(),
    }
}

fn interpret_file(path: &str) {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut interpreter = Interpreter::new();
    let mut scanner = lox::Scanner::new(contents);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    match parser.parse() {
        Ok(ast) => {
            // println!("{}", ast.pretty_print());
            match interpreter.interpret(ast) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn repl() {
    println!("Lox Repl");
    let mut interpreter = Interpreter::new();
    loop {
        let input = rprompt::prompt_reply_stdout(">").unwrap();

        let mut scanner = lox::Scanner::new(input);
        scanner.scan_tokens();

        let mut parser = Parser::new(scanner.tokens);
        match parser.parse() {
            Ok(output) => {
                println!("{}", output.pretty_print());
                match interpreter.interpret(output) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
