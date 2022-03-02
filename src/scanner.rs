#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
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
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
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
    // Other stuff
    Error,
    EOF,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub line: usize,
    pub token: &'a str,
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn init(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_alpha(character: char) -> bool {
        (character >= 'a' && character <= 'z')
        || (character >= 'A' && character <= 'Z')
        || character == '_'
    }

    fn is_digit(character: char) -> bool {
        character >= '0' && character <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.current < self.source.len() {
            self.source.chars().nth(self.current).unwrap()
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            token: &self.source[self.start..self.current],
        }
    }

    fn error_token<'a>(&self, message: &'a str) -> Token<'a> {
        Token {
            token_type: TokenType::Error,
            line: self.line,
            token: message,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let character = self.peek();
            match character {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + rest.len() && &self.source[self.start + start..self.start + start + rest.len()] == rest {
            token_type
        } else {
            TokenType::Identifier
        }
    }

    fn identifier_type(&self) -> TokenType {
        let start_character = self.source.chars().nth(self.start).unwrap();
        match start_character {
            'a' => self.check_keyword(1, "nd", TokenType::And),
            'c' => self.check_keyword(1, "lass", TokenType::Class),
            'e' => self.check_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    let next_character = self.source.chars().nth(self.start + 1).unwrap();
                    match next_character {
                        'a' => self.check_keyword(2, "lse", TokenType::False),
                        'o' => self.check_keyword(2, "r", TokenType::For),
                        'u' => self.check_keyword(2, "m", TokenType::Fun),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, "f", TokenType::If),
            'n' => self.check_keyword(1, "il", TokenType::Nil),
            'o' => self.check_keyword(1, "r", TokenType::Or),
            'p' => self.check_keyword(1, "rint", TokenType::Print),
            'r' => self.check_keyword(1, "eturn", TokenType::Return),
            's' => self.check_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    let next_character = self.source.chars().nth(self.start + 1).unwrap();
                    match next_character {
                        'h' => self.check_keyword(2, "is", TokenType::This),
                        'r' => self.check_keyword(2, "ue", TokenType::True),
                        _ => TokenType::Identifier
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, "ar", TokenType::Var),
            'w' => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier
        }
    }

    fn identifier(&mut self) -> Token {
        while Scanner::is_alpha(self.peek()) || Scanner::is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // Nom nom dot.
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.");
        }

        // Nom nom closing quotes.
        self.advance();
        self.make_token(TokenType::String)
    }

    pub fn scan_token(& mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let character = self.advance();

        if Scanner::is_alpha(character) {
            return self.identifier();
        }

        if Scanner::is_digit(character) {
            return self.number();
        }

        match character {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let next_char_matches = self.match_char('=');
                self.make_token(if next_char_matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let next_char_matches = self.match_char('=');
                self.make_token(if next_char_matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let next_char_matches = self.match_char('=');
                self.make_token(if next_char_matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let next_char_matches = self.match_char('=');
                self.make_token(if next_char_matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }
}
