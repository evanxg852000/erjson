use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JSONValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(HashMap<String, JSONValue>),
    Array(Vec<JSONValue>),
}

impl JSONValue {
    pub fn as_str(&self) -> String {
        match self {
            JSONValue::String(s) => s.clone(),
            _ => "".to_string(),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            JSONValue::Number(n) => *n as i64,
            _ => 0,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            JSONValue::Number(n) => *n,
            _ => 0f64,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            JSONValue::Boolean(b) => *b,
            _ => false,
        }
    }

    pub fn as_array(&self) -> Vec<JSONValue> {
        match self {
            JSONValue::Array(vc) => vc.clone(),
            _ => Vec::new(),
        }
    }

    pub fn as_map(&self) -> HashMap<String, JSONValue> {
        match self {
            JSONValue::Object(hm) => hm.clone(),
            _ => HashMap::new(),
        }
    }

    pub fn get(&self, k: &str) -> Option<JSONValue> {
        match self {
            JSONValue::Object(hm) => match hm.get(k) {
                Some(v) => Some(v.clone()),
                None => None,
            },
            _ => None,
        }
    }

    pub fn exists() -> bool {
        false
    }

    pub fn is_number(&self) -> bool {
        match self {
            JSONValue::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            JSONValue::String(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            JSONValue::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            JSONValue::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            JSONValue::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JSONValue::Null => true,
            _ => false,
        }
    }
}

impl fmt::Display for JSONValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JSONValue::Null => write!(f, "null"),
            JSONValue::Boolean(b) => write!(f, "{}", b),
            JSONValue::Number(n) => write!(f, "{}", n),
            JSONValue::String(s) => write!(f, "\"{}\"", s),
            JSONValue::Object(hm) => {
                let mut ctr = 0;
                write!(f, "{{")?;
                for (k, v) in hm {
                    if ctr < hm.len() - 1 {
                        write!(f, "\"{}\":{},", k, v)?
                    } else {
                        write!(f, "\"{}\":{}", k, v)?
                    };
                    ctr += 1;
                }
                write!(f, "}}")
            }
            JSONValue::Array(vc) => {
                let mut ctr = 0;
                write!(f, "[")?;
                for v in vc {
                    if ctr < vc.len() - 1 {
                        write!(f, "{},", v)?
                    } else {
                        write!(f, "{}", v)?
                    };
                    ctr += 1;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug)]
pub struct JSONError(String, usize, usize);

impl JSONError {
    pub fn new(err: String, lin: usize, col: usize) -> Self {
        JSONError(err, lin, col)
    }
}

impl fmt::Display for JSONError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSONError: {} - @ ({}, {})", self.0, self.1, self.2)
    }
}
