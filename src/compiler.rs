use crate::scanner::{Scanner, TokenType};

pub struct Parser {
    scanner: Scanner
}

impl Parser {
    pub fn init(source: String) -> Self {
        Self { scanner: Scanner::init(source) }
    }

    pub fn compile(&mut self) {
        let mut line = 0;

        loop {
            let token = self.scanner.scan_token();
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
}
