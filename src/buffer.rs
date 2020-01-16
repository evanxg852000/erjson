use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::iter::Iterator;
use std::str;

#[derive(Debug)]
pub enum LineBuffer<'a> {
    Text(str::Lines<'a>),
    Reader(Lines<BufReader<File>>),
}

impl<'a> LineBuffer<'a> {
    pub fn from_string(text: &'a String) -> Self {
        LineBuffer::Text(text.lines())
    }

    pub fn from_file(file: File) -> Self {
        LineBuffer::Reader(BufReader::new(file).lines())
    }
}

impl<'a> Iterator for LineBuffer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        return match self {
            LineBuffer::Text(it) => match it.next() {
                Some(ref s) => {
                    let mut s = s.to_string();
                    s.push('\n');
                    Some(s)
                }
                None => None,
            },
            LineBuffer::Reader(it) => match it.next() {
                Some(r) => match r {
                    Ok(mut s) => {
                        s.push('\n');
                        Some(s)
                    }
                    _ => None,
                },
                None => None,
            },
        };
    }
}

//TODO: unicode support
#[derive(Debug)]
pub struct StringIterator {
    chars: Vec<char>,
    pos: usize,
}

impl StringIterator {
    pub fn new(text: String) -> Self {
        StringIterator {
            chars: text.chars().collect(),
            pos: 0,
        }
    }
}

impl Iterator for StringIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let v = self.chars[self.pos];
            self.pos += 1;
            Some(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_iterator() {
        let mut si = StringIterator::new("jhone".to_string());
        assert_eq!(Some('j'), si.next());
        assert_eq!(Some('h'), si.next());
        assert_eq!(Some('o'), si.next());
        assert_eq!(Some('n'), si.next());
        assert_eq!(Some('e'), si.next());
        assert_eq!(None, si.next());
    }

    #[test]
    fn string_line_buffer() {
        let text = String::from(include_str!("../fixtures/person.json"));
        let lb = LineBuffer::from_string(&text);
        let v: Vec<String> = lb.collect();
        assert_eq!(v.len(), 11);
    }
}
