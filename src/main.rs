extern crate rprompt;
extern crate lox;

fn main() {
    let input = rprompt::prompt_reply_stdout(">").unwrap();

    let mut scanner = lox::Scanner::new(input);
    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);
}
