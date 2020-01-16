use std::collections::HashMap;
use std::fs::File;

use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::{JSONError, JSONValue};

#[derive(Debug)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    ct: Token,
}

impl<'a> Parser<'a> {
    pub fn from_string(data: &'a String) -> Self {
        Parser {
            scanner: Scanner::from_string(&data),
            ct: Token::dummy(),
        }
    }

    pub fn from_file(file: File) -> Self {
        Parser {
            scanner: Scanner::from_file(file),
            ct: Token::dummy(),
        }
    }

    pub fn parse(&mut self) -> Result<JSONValue, JSONError> {
        let mut value = Err(JSONError::new("Empty json stream".to_string(), 0, 0));
        loop {
            let tk = self.consume();
            value = match tk.kind {
                TokenKind::Eof => break,
                _ => self.parse_value(),
            }
        }
        value
    }

    fn consume(&mut self) -> Token {
        self.ct = self.scanner.next_token();
        self.ct.clone()
    }

    fn parse_object(&mut self) -> Result<JSONValue, JSONError> {
        let mut values: HashMap<String, JSONValue> = HashMap::new();

        //consume LeftBrace
        self.consume();
        if self.ct.kind == TokenKind::RightBrace {
            self.consume();
            return Ok(JSONValue::Object(values));
        }

        while self.ct.kind != TokenKind::RightBrace && self.ct.kind != TokenKind::Eof {
            if self.ct.kind != TokenKind::String {
                return Err(JSONError::new(
                    format!("Expecting key name but found {:?}", self.ct.kind),
                    self.ct.line,
                    self.ct.col,
                ));
            }
            let key = self.ct.value.clone();
            self.consume(); //consume String

            if self.ct.kind != TokenKind::Collon {
                return Err(JSONError::new(
                    format!("Expecting collon but found {:?}", self.ct.kind),
                    self.ct.line,
                    self.ct.col,
                ));
            }
            self.consume(); //consume Collon

            let r = self.parse_value();
            match r {
                Ok(v) => values.insert(key, v),
                Err(_) => return r,
            };

            match self.ct.kind {
                TokenKind::Comma => {
                    self.consume();
                    if self.ct.kind == TokenKind::RightBrace {
                        return Err(JSONError::new(
                            format!("Trailing `{}`", self.ct.value),
                            self.ct.line,
                            self.ct.col,
                        ));
                    }
                }
                TokenKind::RightBrace => {
                    self.consume();
                    return Ok(JSONValue::Object(values));
                }
                _ => {
                    return Err(JSONError::new(
                        format!("Unexpected token {}", self.ct.value),
                        self.ct.line,
                        self.ct.col,
                    ))
                }
            };
        }

        Ok(JSONValue::Object(values))
    }

    fn parse_array(&mut self) -> Result<JSONValue, JSONError> {
        let mut values: Vec<JSONValue> = Vec::new();

        //consume LeftBracket
        self.consume();
        if self.ct.kind == TokenKind::RightBracket {
            self.consume();
            return Ok(JSONValue::Array(values));
        }

        while self.ct.kind != TokenKind::RightBracket && self.ct.kind != TokenKind::Eof {
            let rst = self.parse_value();
            match rst {
                Ok(v) => values.push(v),
                Err(err) => return Err(err),
            }

            match self.ct.kind {
                TokenKind::Comma => {
                    self.consume();
                    if self.ct.kind == TokenKind::RightBracket {
                        return Err(JSONError::new(
                            format!("Trailing `{}`", self.ct.value),
                            self.ct.line,
                            self.ct.col,
                        ));
                    }
                }
                TokenKind::RightBracket => {
                    self.consume();
                    return Ok(JSONValue::Array(values));
                }
                _ => {
                    return Err(JSONError::new(
                        format!("Unexpected token {}", self.ct.value),
                        self.ct.line,
                        self.ct.col,
                    ))
                }
            };
        }

        Ok(JSONValue::Array(values))
    }

    fn parse_value(&mut self) -> Result<JSONValue, JSONError> {
        let ct = &self.ct;
        let value = match ct.kind {
            TokenKind::Null => {
                self.consume();
                Ok(JSONValue::Null)
            }
            TokenKind::False => {
                self.consume();
                Ok(JSONValue::Boolean(false))
            }
            TokenKind::True => {
                self.consume();
                Ok(JSONValue::Boolean(true))
            }
            TokenKind::String => {
                let v = ct.value.clone();
                self.consume();
                Ok(JSONValue::String(v))
            }
            TokenKind::Number => {
                // TODO: distinct int, float
                match ct.value.parse::<f64>() {
                    Ok(n) => {
                        self.consume();
                        Ok(JSONValue::Number(n))
                    }
                    Err(_) => Err(JSONError::new(
                        format!("{} is NaN ", ct.value),
                        ct.line,
                        ct.col,
                    )),
                }
            }
            TokenKind::LeftBrace => self.parse_object(),
            TokenKind::LeftBracket => self.parse_array(),
            _ => {
                return Err(JSONError::new(
                    format!("Unexpected token {}", ct.value),
                    ct.line,
                    ct.col,
                ))
            }
        };

        // if self.ct.kind != TokenKind::Eof {
        //     println!("after value consume {:?}", self.ct);
        //     self.consume();
        // }
        value
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_parsing() {
        //TODO:
    }

    #[test]
    fn complex_parsing() {
        //TODO:
    }
}
