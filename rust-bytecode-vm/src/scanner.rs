use crate::Token;

pub struct Scanner {
    source: Vec<char>,
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
        self.start = self.current;
    }
}
