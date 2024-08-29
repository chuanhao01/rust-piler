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
        // Advances current by 1, and checks expected_char
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
