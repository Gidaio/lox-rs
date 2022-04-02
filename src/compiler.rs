use crate::prelude::*;

pub fn compile(source: &str) {
    let mut scanner = init_scanner(source);
    let mut line = 0;
    loop {
        let token = scan_token(&mut scanner);
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:?} '{}'", token.token_type, token.token);

        if token.token_type == TokenType::EoF {
            break;
        }
    }
}
