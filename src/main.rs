extern crate lox;
extern crate rprompt;

use lox::parser::Parser;
use lox::ast::AstPrint;
fn main() {
    let input = rprompt::prompt_reply_stdout(">").unwrap();

    let mut scanner = lox::Scanner::new(input);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    let output = parser.parse();
    println!("{}", output.pretty_print());

    match lox::interpreter::interpret(output) {
        Ok(o) => println!("{}", o),
        Err(o) => eprintln!("{}", o),
    }
}
