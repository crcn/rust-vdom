use crate::scanner::*;
use crate::tokenizer::*;
use crate::ast::*;
// use crate::tokenizer;
// use scanner;




pub fn parse(source: &str) {
  let mut scanner = Scanner::new(source);
  let mut tokenizer = Tokenizer::new(scanner);
  parseRoot(&mut tokenizer);
}

fn parseRoot(tokenizer: &mut Tokenizer) {
  while !tokenizer.ended() {
    match tokenizer.next() {
      Some(token) => match token {
        Token::LessThan => {
          println!("<");
        },
        Token::Text(v) => {
          println!("{}", v);
        },
        Token::CloseTag => {
          println!(" />");
        }
        _ => {

        }
      },
      _ => {

      }
    }
  }
}
