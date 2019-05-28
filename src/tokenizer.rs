use crate::scanner::Scanner;

#[derive(Debug)]
pub enum Token {
  Comment(String),      // /* comment */
  LessThan,     // <
  Backslash,    // /
  GreaterThan,  // >
  Colon,        // :
  CloseTag,     // />
  Text(String), // a-z0-9
  Unknown(u8)   
}

pub struct Tokenizer<'a> {
  pub scanner: Scanner<'a>,
  pub current: Option<Token>
}

impl<'a> Tokenizer<'a> {
  pub fn new(scanner: Scanner<'a>) -> Tokenizer {
    Tokenizer { scanner, current: None }
  }
  pub fn ended(&self) -> bool {
    self.scanner.ended()
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
      b'<' => Token::LessThan,
      b'>' => Token::GreaterThan,
      b'/' => {
        match self.scanner.peek(1) {
          Some(c) => {
            match c {
              b'>' => {
                self.scanner.next(); // eat }
                Token::CloseTag
              }
              _ => {
                Token::Backslash
              }
            }
          },
          None => {
            Token::Backslash
          }
        }
      },
      b'a'...b'z' => {
        let mut buffer = vec![c];
        buffer.append(&mut self.scanner.scan(|c| c != b' '));
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