use crate::scanner::{Scanner, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::init(source);
    let mut line = 0;

    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:?} '{}'", token.token_type, token.token);

        if let TokenType::EOF = token.token_type {
            break;
        }
    }
}
