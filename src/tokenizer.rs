use crate::scanner::Scanner;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
  Comment(String),      // /* comment */
  LessThan,     // <
  Backslash,    // /
  GreaterThan,  // >
  Colon,        // :
  Whitespace,   // \s\r\n\t
  Equals,       // =
  String(String),       // "aa" | 'aa'
  SelfCloseTag,     // />
  StartCloseTag,     // </
  Text(String), // a-z0-9
  Unknown(u8)   
}

pub struct Tokenizer<'a> {
  pub scanner: Scanner<'a>,
  pub current: Option<Token>
}

impl<'a> Tokenizer<'a> {
  pub fn new(scanner: Scanner<'a>) -> Tokenizer {
    let mut tokenizer = Tokenizer { scanner, current: None };
    tokenizer.next();
    tokenizer
  }
  pub fn ended(&self) -> bool {
    self.scanner.ended()
  }
  pub fn eat_whitespace(&mut self) {
    loop {
      match &self.current {
        Some(token) => match token {
          Token::Whitespace => {
            self.next();
          },
          _ => {
            break;
          }
        },
        None => {
          break;
        }
      }
    }
  }
  pub fn next<'b>(&mut self) -> &Option<Token> {
    if self.scanner.ended() {
      self.current = None;
      return &self.current;
    }
    // let scanner = self.scanner;
    let c = self.scanner.curr();

    let token = match c {
      b':' => Token::Colon,
      b'<' => {
        match self.scanner.peek(1) {
          Some(c) => {
            if c == b'/' {
              self.scanner.next(); // eat /
              Token::StartCloseTag
            } else {
              Token::LessThan
            }
          }
          None => Token::LessThan
        }
      },
      b'>' => Token::GreaterThan,
      b'=' => Token::Equals,
      b'\'' | b'"' => {
        let mut buffer = vec![];
        buffer.append(&mut self.scanner.scan(|c2| c2 != c));
        self.scanner.next(); // eat quote
        Token::String(String::from_utf8(buffer).unwrap())
      },
      b' ' | b'\r' | b'\n' | b'\t' => {
        self.scanner.scan(|c| is_whitespace(c));
        Token::Whitespace
      }
      b'/' => {
        match self.scanner.peek(1) {
          Some(c) => {
            if c == b'>' {
              self.scanner.next(); // eat >
              Token::SelfCloseTag
            } else {
              Token::Backslash
            }
          },
          None => {
            Token::Backslash
          }
        }
      },
      b'a'...b'z' => {
        let mut buffer = vec![c];
        buffer.append(&mut self.scanner.scan(|c| c != b' ' && c != b'=' && c != b'>' && c != b'<'));
        let buffer: String = String::from_utf8(buffer).unwrap();
        Token::Text(buffer)
      }
      _ => Token::Unknown(c)
    };

    // queue up the next character
    self.scanner.next(); 

    self.current = Some(token);
    &self.current
  }
}

fn is_whitespace(c: u8) -> bool {
  c == b' ' || c == b'\r' || c == b'\n' || c == b'\t'
}