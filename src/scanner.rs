pub struct Scanner<'source> {
    source: &'source str,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(usize)]
pub enum TokenType {
    LeftParen = 0,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    EoF,
}

#[derive(Clone)]
pub struct Token<'source> {
    pub token_type: TokenType,
    pub line: usize,
    pub token: &'source str,
}

pub fn init_scanner(source: &str) -> Scanner {
    Scanner {
        source,
        start: 0,
        current: 0,
        line: 1,
    }
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_at_end(scanner: &Scanner) -> bool {
    scanner.current == scanner.source.len()
}

fn advance(scanner: &mut Scanner) -> char {
    scanner.current += 1;
    scanner
        .source
        .chars()
        .nth(scanner.current - 1)
        .expect("Couldn't advance.")
}

fn peek(scanner: &Scanner) -> char {
    scanner.source.chars().nth(scanner.current).unwrap_or('\0')
}

fn peek_next(scanner: &Scanner) -> char {
    if is_at_end(scanner) {
        '\0'
    } else {
        scanner
            .source
            .chars()
            .nth(scanner.current + 1)
            .unwrap_or('\0')
    }
}

fn match_char(scanner: &mut Scanner, expected: char) -> bool {
    if is_at_end(scanner) {
        return false;
    }

    if scanner
        .source
        .chars()
        .nth(scanner.current)
        .expect("Couldn't get current character.")
        != expected
    {
        return false;
    }

    scanner.current += 1;
    true
}

fn make_token<'source>(scanner: &Scanner<'source>, token_type: TokenType) -> Token<'source> {
    Token {
        token_type,
        token: &scanner.source[scanner.start..scanner.current],
        line: scanner.line,
    }
}

fn error_token(scanner: &Scanner, message: &'static str) -> Token<'static> {
    Token {
        token_type: TokenType::Error,
        token: message,
        line: scanner.line,
    }
}

fn skip_whitespace(scanner: &mut Scanner) {
    loop {
        let c = peek(scanner);
        match c {
            ' ' | '\r' | '\t' => {
                advance(scanner);
            }
            '\n' => {
                scanner.line += 1;
                advance(scanner);
            }
            '/' => {
                if peek_next(scanner) == '/' {
                    while peek(scanner) != '\n' && !is_at_end(scanner) {
                        advance(scanner);
                    }
                } else {
                    return;
                }
            }
            _ => return,
        }
    }
}

fn check_keyword(scanner: &Scanner, start: usize, rest: &str, token_type: TokenType) -> TokenType {
    if scanner.current - scanner.start == start + rest.len()
        && &scanner.source[scanner.start + start..scanner.start + start + rest.len()] == rest
    {
        token_type
    } else {
        TokenType::Identifier
    }
}

fn identifier_type(scanner: &Scanner) -> TokenType {
    let start_char = scanner
        .source
        .chars()
        .nth(scanner.start)
        .expect("Couldn't get start character.");
    match start_char {
        'a' => check_keyword(scanner, 1, "nd", TokenType::And),
        'c' => check_keyword(scanner, 1, "lass", TokenType::Class),
        'e' => check_keyword(scanner, 1, "lse", TokenType::Else),
        'f' => {
            if scanner.current - scanner.start > 1 {
                let next_char = scanner
                    .source
                    .chars()
                    .nth(scanner.start + 1)
                    .expect("Couldn't get next character.");
                match next_char {
                    'a' => check_keyword(scanner, 2, "lse", TokenType::False),
                    'o' => check_keyword(scanner, 2, "r", TokenType::For),
                    'u' => check_keyword(scanner, 2, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            }
        }
        'i' => check_keyword(scanner, 1, "f", TokenType::If),
        'n' => check_keyword(scanner, 1, "il", TokenType::Nil),
        'o' => check_keyword(scanner, 1, "r", TokenType::Or),
        'p' => check_keyword(scanner, 1, "rint", TokenType::Print),
        'r' => check_keyword(scanner, 1, "eturn", TokenType::Return),
        's' => check_keyword(scanner, 1, "uper", TokenType::Super),
        't' => {
            if scanner.current - scanner.start > 1 {
                let next_char = scanner
                    .source
                    .chars()
                    .nth(scanner.start + 1)
                    .expect("Couldn't get next character.");
                match next_char {
                    'h' => check_keyword(scanner, 2, "is", TokenType::This),
                    'r' => check_keyword(scanner, 2, "ue", TokenType::True),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            }
        }
        'v' => check_keyword(scanner, 1, "ar", TokenType::Var),
        'w' => check_keyword(scanner, 1, "hile", TokenType::While),

        _ => TokenType::Identifier,
    }
}

fn identifier<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    while is_alpha(peek(scanner)) || is_digit(peek(scanner)) {
        advance(scanner);
    }

    make_token(scanner, identifier_type(scanner))
}

fn number<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    while is_digit(peek(scanner)) {
        advance(scanner);
    }

    if peek(scanner) == '.' && is_digit(peek_next(scanner)) {
        // Consume the ".".
        advance(scanner);

        while is_digit(peek(scanner)) {
            advance(scanner);
        }
    };

    make_token(scanner, TokenType::Number)
}

fn string<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    while peek(scanner) != '"' && !is_at_end(scanner) {
        if peek(scanner) == '\n' {
            scanner.line += 1;
        }
        advance(scanner);
    }

    if is_at_end(scanner) {
        error_token(scanner, "Unterminated string.")
    } else {
        // The closing quote.
        advance(scanner);
        make_token(scanner, TokenType::String)
    }
}

pub fn scan_token<'source>(scanner: &mut Scanner<'source>) -> Token<'source> {
    skip_whitespace(scanner);
    scanner.start = scanner.current;

    if is_at_end(scanner) {
        return make_token(scanner, TokenType::EoF);
    }

    let c = advance(scanner);
    if is_alpha(c) {
        return identifier(scanner);
    }
    if is_digit(c) {
        return number(scanner);
    }

    match c {
        '(' => make_token(scanner, TokenType::LeftParen),
        ')' => make_token(scanner, TokenType::RightParen),
        '{' => make_token(scanner, TokenType::LeftBrace),
        '}' => make_token(scanner, TokenType::RightBrace),
        ';' => make_token(scanner, TokenType::Semicolon),
        ',' => make_token(scanner, TokenType::Comma),
        '.' => make_token(scanner, TokenType::Dot),
        '-' => make_token(scanner, TokenType::Minus),
        '+' => make_token(scanner, TokenType::Plus),
        '/' => make_token(scanner, TokenType::Slash),
        '*' => make_token(scanner, TokenType::Star),

        '!' => {
            let char_matches = match_char(scanner, '=');
            make_token(
                scanner,
                if char_matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                },
            )
        }
        '=' => {
            let char_matches = match_char(scanner, '=');
            make_token(
                scanner,
                if char_matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                },
            )
        }
        '<' => {
            let char_matches = match_char(scanner, '=');
            make_token(
                scanner,
                if char_matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                },
            )
        }
        '>' => {
            let char_matches = match_char(scanner, '=');
            make_token(
                scanner,
                if char_matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                },
            )
        }

        '"' => string(scanner),

        _ => error_token(scanner, "Unexpected character."),
    }
}
