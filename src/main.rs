extern crate lox;
extern crate rprompt;

use lox::parser::Parser;
fn main() {
    let input = rprompt::prompt_reply_stdout(">").unwrap();

    let mut scanner = lox::Scanner::new(input);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    let output = parser.parse();
    println!("{}", lox::ast::pretty_print(&output));
    println!("{}", lox::interpreter::interpret(output))
}
