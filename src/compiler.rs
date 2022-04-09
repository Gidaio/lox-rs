use crate::prelude::*;

struct Parser<'source> {
    current: Token<'source>,
    previous: Token<'source>,
    had_error: bool,
    panic_mode: bool,

    // temp
    compiling_chunk: &'source mut Chunk,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None = 0,
    Assignment, // =
    Or, // or
    And, // and
    Equality, // == !=
    Comparison, // < > <= >=
    Term, // + -
    Factor, // * /
    Unary, // ! -
    Call, // . ()
    Primary,
}

fn increment_precedence(precedence: &Precedence) -> Precedence {
    match precedence {
        Precedence::None => Precedence::Assignment,
        Precedence::Assignment => Precedence::Or,
        Precedence::Or => Precedence::And,
        Precedence::And => Precedence::Equality,
        Precedence::Equality => Precedence::Comparison,
        Precedence::Comparison => Precedence::Term,
        Precedence::Term => Precedence::Factor,
        Precedence::Factor => Precedence::Unary,
        Precedence::Unary => Precedence::Call,
        Precedence::Call => Precedence::Primary,
        Precedence::Primary => panic!("Can't increment Primary precedence!"),
    }
}

type ParseFn = for<'source, 'r, 's> fn(&'r mut Parser<'source>, &'s mut Scanner<'source>);

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

fn current_chunk<'chunk>(parser: &'chunk mut Parser) -> &'chunk mut Chunk {
    parser.compiling_chunk
}

fn error_at(parser: &mut Parser, token: Token, message: &str) {
    if parser.panic_mode {
        return;
    }

    parser.panic_mode = true;
    eprint!("[line {}] Error", token.line);

    if token.token_type == TokenType::EoF {
        eprint!(" at end");
    } else if token.token_type == TokenType::Error {
        // noop!
    } else {
        eprint!(" at '{}'", token.token);
    }

    eprintln!(": {}", message);
    parser.had_error = true;
}

fn error(parser: &mut Parser, message: &str) {
    error_at(parser, parser.previous.clone(), message);
}

fn error_at_current(parser: &mut Parser, message: &str) {
    error_at(parser, parser.current.clone(), message);
}

fn advance<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>) {
    parser.previous = parser.current.clone();

    loop {
        parser.current = scan_token(scanner);
        if parser.current.token_type != TokenType::Error {
            break;
        }

        error_at_current(parser, parser.current.token);
    }
}

fn consume<'source>(
    parser: &mut Parser<'source>,
    scanner: &mut Scanner<'source>,
    token_type: TokenType,
    message: &str,
) {
    if parser.current.token_type == token_type {
        advance(parser, scanner);
    } else {
        error_at_current(parser, message);
    }
}

fn emit_byte(parser: &mut Parser, byte: u8) {
    let previous_token = parser.previous.clone();
    write_chunk(current_chunk(parser), byte, previous_token.line);
}

fn emit_bytes(parser: &mut Parser, byte_1: u8, byte_2: u8) {
    emit_byte(parser, byte_1);
    emit_byte(parser, byte_2);
}

fn emit_return(parser: &mut Parser) {
    emit_byte(parser, OP_RETURN);
}

fn make_constant(parser: &mut Parser, value: Value) -> u8 {
    let constant = add_constant(parser.compiling_chunk, value);
    if constant > u8::MAX as usize {
        error(parser, "Too many constants in one chunk.");
        0
    } else {
        constant as u8
    }
}

fn emit_constant(parser: &mut Parser, value: Value) {
    let constant = make_constant(parser, value);
    emit_bytes(parser, OP_CONSTANT, constant);
}

fn end_compiler(parser: &mut Parser) {
    emit_return(parser);
    if DEBUG_PRINT_CODE && !parser.had_error {
        disassemble_chunk(parser.compiling_chunk, "code");
    }
}

fn binary<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous.token_type.clone();
    let rule = get_rule(operator_type.clone());
    parse_precedence(parser, scanner, increment_precedence(&rule.precedence));

    match operator_type {
        TokenType::Plus => emit_byte(parser, OP_ADD),
        TokenType::Minus => emit_byte(parser, OP_SUBTRACT),
        TokenType::Star => emit_byte(parser, OP_MULTIPLY),
        TokenType::Slash => emit_byte(parser, OP_DIVIDE),
        _ => ()
    }
}

fn grouping<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>) {
    expression(parser, scanner);
    consume(parser, scanner, TokenType::RightParen, "Expect ')' after expression.");
}

fn number(parser: &mut Parser, _scanner: &mut Scanner) {
    let value = parser.previous.token.parse::<Value>().expect("Couldn't parse number token.");
    emit_constant(parser, value);
}

fn unary<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>) {
    let operator_type = parser.previous.token_type.clone();

    parse_precedence(parser, scanner, Precedence::Unary);

    match operator_type {
        TokenType::Minus => emit_byte(parser, OP_NEGATE),
        _ => ()
    }
}

const RULES: [ParseRule; 40] = [
    ParseRule { prefix: Some(grouping), infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term },
    ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Term },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
    ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },

    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },

    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: Some(number), infix: None, precedence: Precedence::None },
    
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },

    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
];

fn parse_precedence<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>, precedence: Precedence) {
    advance(parser, scanner);
    let prefix_rule = get_rule(parser.previous.token_type.clone()).prefix; 
    if let Some(func) = prefix_rule {
        func(parser, scanner);
    } else {
        error(parser, "Expect expression.");
        return;
    }

    while precedence <= get_rule(parser.current.token_type.clone()).precedence {
        advance(parser, scanner);
        let infix_rule = get_rule(parser.previous.token_type.clone()).infix;
        if let Some(func) = infix_rule {
            func(parser, scanner);
        } else {
            panic!("Unreachable.");
        }
    }
}

fn get_rule(token_type: TokenType) -> &'static ParseRule {
    &RULES[token_type as usize]
}

fn expression<'source>(parser: &mut Parser<'source>, scanner: &mut Scanner<'source>) {
    parse_precedence(parser, scanner, Precedence::Assignment);
}

pub fn compile(source: &str, chunk: &mut Chunk) -> bool {
    let mut scanner = init_scanner(source);
    let mut parser = Parser {
        current: Token {
            line: 0,
            token: "",
            token_type: TokenType::Error,
        },
        previous: Token {
            line: 0,
            token: "",
            token_type: TokenType::Error,
        },
        had_error: false,
        panic_mode: false,

        compiling_chunk: chunk,
    };
    advance(&mut parser, &mut scanner);
    expression(&mut parser, &mut scanner);
    consume(
        &mut parser,
        &mut scanner,
        TokenType::EoF,
        "Expect end of expression.",
    );
    end_compiler(&mut parser);
    !parser.had_error
}
