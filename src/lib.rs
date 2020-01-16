mod buffer;
mod parser;
mod scanner;
mod value;

use parser::Parser;
use std::fs::File;
pub use value::{JSONError, JSONValue};

#[derive(Debug)]
pub struct JSONDocument {
    pub value: Option<JSONValue>,
}

impl JSONDocument {
    pub fn new() -> JSONDocument {
        JSONDocument { value: None }
    }

    pub fn parse_string(&mut self, content: String) -> Result<JSONValue, JSONError> {
        let mut parser = Parser::from_string(&content);
        match parser.parse() {
            Ok(p) => {
                self.value = Some(p.clone());
                Ok(p)
            }
            Err(e) => Err(e),
        }
    }

    pub fn parse_file(&mut self, file: File) -> Result<JSONValue, JSONError> {
        let mut parser = Parser::from_file(file);
        match parser.parse() {
            Ok(p) => {
                self.value = Some(p.clone());
                Ok(p)
            }
            Err(e) => Err(e),
        }
    }

    pub fn to_string(&mut self) -> Result<String, JSONError> {
        match &self.value {
            Some(v) => Ok(v.to_string()),
            None => Ok(String::from("null"))
        } 
    }

    pub fn pretify(&mut self) -> String {
        "null".to_string()
        // match &self.value {
        //     Some(value) => value.to_pretty_string(),
        //     None => String::from("null")
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_string_interface() {
        let mut doc = JSONDocument::new();
        assert!(doc.parse_string("test".to_string()).is_err());
    }

    #[test]
    fn library_file_interface() {
        let mut doc = JSONDocument::new();
        let file = File::open("fixtures/sample.json").unwrap();
        assert!(doc.parse_file(file).is_ok());
        // println!("t {:?}", doc.value);
        // assert_eq!(1, 3);
    }

    #[test]
    fn single_value() {
        let mut doc = JSONDocument::new();

        // Null
        let v = doc.parse_string("null".to_string()).unwrap();
        assert_eq!(v, JSONValue::Null);

        // 12.03e+3 -> 12030
        let v = doc.parse_string("12.03e+3".to_string()).unwrap();
        assert_eq!(v, JSONValue::Number(12030f64));

        // False
        let v = doc.parse_string("false".to_string()).unwrap();
        assert_eq!(v, JSONValue::Boolean(false));

        // True
        let v = doc.parse_string("true".to_string()).unwrap();
        assert_eq!(v, JSONValue::Boolean(true));

        // "jhon"
        let v = doc.parse_string("\"jhon\"".to_string()).unwrap();
        assert_eq!(v, JSONValue::String("jhon".to_string()));
    }
}
