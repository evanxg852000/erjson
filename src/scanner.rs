use std::fs::File;

use crate::buffer::{LineBuffer, StringIterator};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Collon,
    Comma,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    String,
    Number,
    Null,
    False,
    True,
    Eof,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(k: TokenKind, val: String, l: usize, c: usize) -> Self {
        Token {
            kind: k,
            value: val,
            line: l,
            col: c,
        }
    }

    pub fn dummy() -> Self {
        Token {
            kind: TokenKind::Eof,
            value: "".to_string(),
            line: 0,
            col: 0,
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    lines: LineBuffer<'a>,
    line: Option<StringIterator>,
    ch: Option<char>,
    pk: Option<char>,
    lin: usize,
    pos: usize,
}

impl<'a> Scanner<'a> {
    pub fn from_string(data: &'a String) -> Scanner<'a> {
        Scanner {
            lines: LineBuffer::from_string(data),
            line: None,
            ch: None,
            pk: None,
            lin: 0,
            pos: 0,
        }
    }

    pub fn from_file(file: File) -> Scanner<'a> {
        Scanner {
            lines: LineBuffer::from_file(file),
            line: None,
            ch: None,
            pk: None,
            lin: 0,
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        // first time: move to first line
        if self.ch == None && self.pk == None {
            let _ = self.consume();
        }

        while let Some(c) = self.ch {
            if c.is_whitespace() {
                self.consume();
                continue;
            } else if c == ':' {
                self.consume();
                return Token::new(TokenKind::Collon, c.to_string(), self.lin, self.pos);
            } else if c == ',' {
                self.consume();
                return Token::new(TokenKind::Comma, c.to_string(), self.lin, self.pos);
            } else if c == '{' {
                self.consume();
                return Token::new(TokenKind::LeftBrace, c.to_string(), self.lin, self.pos);
            } else if c == '}' {
                self.consume();
                return Token::new(TokenKind::RightBrace, c.to_string(), self.lin, self.pos);
            } else if c == '[' {
                self.consume();
                return Token::new(TokenKind::LeftBracket, c.to_string(), self.lin, self.pos);
            } else if c == ']' {
                self.consume();
                return Token::new(TokenKind::RightBracket, c.to_string(), self.lin, self.pos);
            } else if c == '"' {
                let pos = self.pos;
                let word = self.scan_str();
                return match word {
                    Some(w) => Token::new(TokenKind::String, w, self.lin, pos),
                    None => Token::new(TokenKind::Error, "".to_string(), self.lin, pos),
                };
            } else if c.is_digit(10) || c == '-' {
                let pos = self.pos;
                let num = self.scan_num();
                return Token::new(TokenKind::Number, num, self.lin, pos);
            } else if c.is_alphabetic() {
                let pos = self.pos;
                let word = self.scan_word();
                return match word {
                    Some(w) => match w.as_ref() {
                        "null" => Token::new(TokenKind::Null, w, self.lin, pos),
                        "true" => Token::new(TokenKind::True, w, self.lin, pos),
                        "false" => Token::new(TokenKind::False, w, self.lin, pos),
                        _ => Token::new(TokenKind::Error, w, self.lin, self.pos),
                    },
                    None => Token::new(TokenKind::Error, "".to_string(), self.lin, self.pos),
                };
            }
            // unrecognised token
            self.consume();
            return Token::new(TokenKind::Error, c.to_string(), self.lin, self.pos);
        }

        Token::new(TokenKind::Eof, "".to_string(), self.lin, self.pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.line.is_none() {
            self.line = match self.lines.next() {
                Some(line) => {
                    self.lin = 1;
                    Some(StringIterator::new(line))
                }
                None => None,
            };
            if self.line.is_none() {
                // mark end of file
                self.ch = None;
                return None;
            } else {
                //first time: move pk
                self.pk = match self.line {
                    Some(ref mut chrs) => chrs.next(),
                    None => None,
                };
            }
        }

        match self.ch {
            Some(c) if c == '\n' => {
                self.pos = 1;
                self.lin += 1;
            }
            _ => self.pos += 1,
        };

        self.ch = self.pk;
        self.pk = match self.line {
            Some(ref mut chrs) => match chrs.next() {
                Some(c) => Some(c),
                None => {
                    self.line = match self.lines.next() {
                        Some(line) => Some(StringIterator::new(line)),
                        None => None,
                    };
                    match self.line {
                        Some(ref mut chrs) => chrs.next(),
                        None => None,
                    }
                }
            },
            None => None,
        };
        self.ch
    }

    fn scan_str(&mut self) -> Option<String> {
        let mut word = "".to_string();
        let c = self.consume(); // consume opening "
        if c == Some('"') {
            // check if empty string
            self.consume();
            return Some(word);
        }

        while let Some(p) = self.pk {
            let c = self.ch.unwrap();
            if p == '"' && c != '\\' {
                self.consume();
                word.push(c);
                self.consume(); // consume closing "
                return Some(word);
            }
            word.push(c);
            self.consume();
        }
        None
    }

    fn scan_word(&mut self) -> Option<String> {
        let mut word = "".to_string();
        while let Some(p) = self.pk {
            let c = self.ch.unwrap();
            if !p.is_alphabetic() {
                self.consume();
                word.push(c);
                return Some(word);
            }
            word.push(c);
            self.consume();
        }
        None
    }

    fn scan_num(&mut self) -> String {
        // -10.23e+12
        let mut num = "".to_string();

        //scan num part
        while let Some(p) = self.pk {
            let c = self.ch.unwrap();
            if p.is_digit(10) {
                num.push(c);
                self.consume();
            } else if p == '.' {
                num.push(c);
                self.consume();
                break;
            } else {
                num.push(c);
                self.consume();
                return num;
            }
        }

        //scan decimal part
        while let Some(p) = self.pk {
            let c = self.ch.unwrap();
            if p.is_digit(10) {
                num.push(c);
                self.consume();
            } else if p == 'e' {
                num.push(c);
                self.consume();
                break;
            } else {
                num.push(c);
                self.consume();
                return num;
            }
        }

        // scan expoent part
        let e = self.ch.unwrap();
        num.push(e);
        self.consume();
        match self.ch {
            Some(c) if c == '-' || c == '+' => {
                num.push(c);
                self.consume();
            }
            _ => (),
        };
        while let Some(p) = self.pk {
            let c = self.ch.unwrap();
            if p.is_digit(10) {
                num.push(c);
                self.consume();
            } else {
                num.push(c);
                self.consume();
                return num;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner() {
        let json = String::from(include_str!("../fixtures/person.json"));
        let mut scanner = Scanner::from_string(&json);
        let mut tokens: Vec<Token> = vec![];
        loop {
            let t = scanner.next_token();
            if t.kind == TokenKind::Eof {
                tokens.push(t);
                break;
            }
            tokens.push(t);
        }

        let t = &tokens[0];
        assert_eq!(t.kind, TokenKind::LeftBrace);

        let t = &tokens[1];
        assert_eq!(t.kind, TokenKind::String);
        assert_eq!(t.value, "name".to_string());
        assert_eq!(t.line, 2);
        assert_eq!(t.col, 5);

        let t = &tokens[19];
        assert_eq!(t.kind, TokenKind::True);
        assert_eq!(t.value, "true".to_string());
        assert_eq!(t.line, 5);
        assert_eq!(t.col, 14);

        let t = &tokens[31];
        assert_eq!(t.kind, TokenKind::Null);
        assert_eq!(t.value, "null".to_string());
        assert_eq!(t.line, 8);
        assert_eq!(t.col, 15);

        let t = &tokens[41];
        assert_eq!(t.kind, TokenKind::Eof);
        assert_eq!(t.value, "".to_string());
        assert_eq!(t.line, 11);
        assert_eq!(t.col, 2);

        assert_eq!(tokens.len(), 42);
    }

    #[test]
    fn scanner_error() {
        let json = "[ 1, 2, _ ]".to_string();
        let mut scanner = Scanner::from_string(&json);
        let mut tokens: Vec<Token> = vec![];
        loop {
            let t = scanner.next_token();
            if t.kind == TokenKind::Eof {
                tokens.push(t);
                break;
            }
            tokens.push(t);
        }

        let t = &tokens[5];
        assert_eq!(t.kind, TokenKind::Error);
        assert_eq!(t.value, "_".to_string());
        assert_eq!(t.line, 1);
        assert_eq!(t.col, 10);

        assert_eq!(tokens.len(), 8);
    }
}
