use crate::{Token, TokenType};

pub struct Scanner {
    pub source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        // After skipping whitespace, we could end up at the end of the file
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        self.start = self.current;
        let c = self.advance();
        if self.is_alpha(c) {
            return self.identifier();
        }
        if self.is_digit(c) {
            return self.number();
        }
        match c {
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
                if self.match_next('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '"' => self.string(),

            _ => self.error_token(String::from("Unexpected Character")),
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
    fn match_next(&mut self, expected_char: char) -> bool {
        // Checks char, and only advances current if matched
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected_char {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        self.source[self.current]
    }
    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }
            let c = self.peek();
            match c {
                // Manually forward current, since advanced will run into index out of range, since we don't have the null byte terminator
                ' ' | '\t' | '\r' => {
                    self.current += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                '/' => {
                    if self.is_at_end() {
                        return;
                    }
                    // Manual peek forward
                    if self.source[self.current + 1] == '/' {
                        self.current += 2; // Skip the //

                        // Ensure that we have not hit the end of the file, or the end of the line
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.current += 1;
                        }
                    } else {
                        // Is a divide, pass back control flow
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token {
        // Take until the ending ", since to enter here, we already took a "
        while !self.is_at_end() && self.peek() != '"' {
            // No newline strings for now
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token(String::from("Unterminated String"));
        }
        // Consume the last quote
        self.advance();
        self.make_token(TokenType::String)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_numeric()
    }
    fn number(&mut self) -> Token {
        while !self.is_at_end() && self.is_digit(self.peek()) {
            self.advance();
        }
        if self.match_next('.') {
            // Could be end, so we manually advance
            while !self.is_at_end() && self.is_digit(self.peek()) {
                self.advance();
            }
        }
        // Edge case of reaching the end or not followed by .
        self.make_token(TokenType::Number)
    }

    fn is_alpha(&self, c: char) -> bool {
        // only a-z and underscores
        c.is_alphabetic() || c == '_'
    }
    fn identifier(&mut self) -> Token {
        // Enters, after already consuming the first character
        // Handles user defined tokens as well as reserved
        while !self.is_at_end() && (self.is_alpha(self.peek()) || self.is_digit(self.peek())) {
            self.advance();
        }
        // Consume until the end of the identifier
        // Pass control to identifier_type, DFA to figure out token type
        self.make_token(self.identifier_type())
    }
    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_reserved_keyword(1, String::from("nd"), TokenType::And),
            'c' => self.check_reserved_keyword(1, String::from("lass"), TokenType::Class),
            'e' => self.check_reserved_keyword(1, String::from("lse"), TokenType::Else),
            'i' => self.check_reserved_keyword(1, String::from("f"), TokenType::If),
            'n' => self.check_reserved_keyword(1, String::from("il"), TokenType::Nil),
            'o' => self.check_reserved_keyword(1, String::from("r"), TokenType::Or),
            'p' => self.check_reserved_keyword(1, String::from("rint"), TokenType::Print),
            'r' => self.check_reserved_keyword(1, String::from("eturn"), TokenType::Return),
            's' => self.check_reserved_keyword(1, String::from("uper"), TokenType::Super),
            'v' => self.check_reserved_keyword(1, String::from("ar"), TokenType::Var),
            'w' => self.check_reserved_keyword(1, String::from("hile"), TokenType::Var),
            'f' => {
                // Needs atleast 2 more chars
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'a' => {
                            self.check_reserved_keyword(2, String::from("lse"), TokenType::False)
                        }
                        'o' => self.check_reserved_keyword(2, String::from("r"), TokenType::For),
                        'u' => self.check_reserved_keyword(2, String::from("n"), TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'h' => self.check_reserved_keyword(2, String::from("is"), TokenType::This),
                        'r' => self.check_reserved_keyword(2, String::from("ue"), TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            _ => TokenType::Identifier,
        }
    }
    fn check_reserved_keyword(
        &self,
        start: usize,
        rest: String,
        token_type: TokenType,
    ) -> TokenType {
        // Given control flow in checking the identifier, with reference of start from self.start
        // Check the rest of the identifier

        // Length check
        let length = rest.len();
        if self.current - self.start == start + length
            && self.source[(self.start + start)..(self.start + start + length)]
                == rest.chars().collect::<Vec<char>>()
        {
            return token_type;
        }
        TokenType::Identifier
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::NormalToken {
            _type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
    fn error_token(&self, msg: String) -> Token {
        Token::ErrorToken {
            line: self.line,
            msg,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
