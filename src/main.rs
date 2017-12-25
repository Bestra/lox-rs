extern crate lox;
extern crate rprompt;

fn main() {
    let input = rprompt::prompt_reply_stdout(">").unwrap();

    let mut scanner = lox::Scanner::new(input);
    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);
}
