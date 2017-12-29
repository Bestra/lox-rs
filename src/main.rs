extern crate lox;
extern crate rprompt;

use lox::parser::Parser;
use lox::ast::AstPrint;
use lox::interpreter::Interpreter;
fn main() {
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
                    Ok(o) => println!("{}", o),
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
