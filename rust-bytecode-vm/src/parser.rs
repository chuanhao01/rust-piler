use crate::{Scanner, Token, TokenType};

pub struct Parser {
    scanner: Scanner,
    current: Option<Token>,
    previous: Option<Token>,
    pub had_error: bool,
    panic_mode: bool,
}
impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Self {
            scanner,
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }
    pub fn advance(&mut self) {
        // Takes the next token, and if error, send it up
        self.previous = self.current.clone();
        // Not sure why we loop for now
        loop {
            self.current = Some(self.scanner.scan_token());
            if let Some(token) = &self.current {
                match token {
                    Token::ErrorToken { line: _, msg } => {
                        self.error_current(msg.to_owned());
                    }
                    Token::NormalToken {
                        _type: _,
                        length: _,
                        line: _,
                        start: _,
                    } => {
                        break;
                    }
                }
            }
        }
    }
    pub fn consume(&mut self, token_type: TokenType, err_msg: String) {
        // Checks the current token based on type
        if let Some(Token::NormalToken {
            _type,
            start: _,
            length: _,
            line: _,
        }) = &self.current
        {
            if token_type == *_type {
                self.advance();
                return;
            }
        };
        self.error_current(err_msg);
    }
    fn error_current(&mut self, msg: String) {
        if let Some(token) = &self.current {
            self.error_at(token, msg);
            self.had_error = true;
            self.panic_mode = true;
        }
    }
    fn error_previous(&mut self, msg: String) {
        if let Some(token) = &self.previous {
            self.error_at(token, msg);
            self.had_error = true;
            self.panic_mode = true;
        }
    }
    /// Use util methods, [self.error_current] or [self.error_previous]
    fn error_at(&self, token: &Token, msg: String) {
        if self.panic_mode {
            return;
        }
        let pre_err_str = match token {
            Token::ErrorToken { line, msg: _ } => {
                format!("Line {} Error: ", line)
            }
            Token::NormalToken {
                _type,
                start,
                length,
                line,
            } => {
                format!(
                    "Line {} Error, at {}: ",
                    line,
                    match _type {
                        TokenType::Eof => {
                            String::from("end")
                        }
                        _ => {
                            format!(
                                "'{}'",
                                self.scanner.source[*start..*start + *length]
                                    .iter()
                                    .collect::<String>()
                            )
                        }
                    }
                )
            }
        };
        println!("{}{}", pre_err_str, msg);
    }
}
