use crate::scanner::*;
use crate::tokenizer::*;
use crate::ast::*;
// use crate::tokenizer;
// use scanner;




pub fn parse(source: &str) -> Result<Node, String> {
  let scanner = Scanner::new(source);
  let mut tokenizer = Tokenizer::new(scanner);
  parse_root(tokenizer)
}

fn parse_root(tokenizer: Tokenizer) -> Result<Node, String> {
  let mut children = match parse_nodes(tokenizer, |tokenizer| !tokenizer.ended()) {
    Ok(nodes) => nodes,
    Err(e) => {
      return Err(e);
    }
  };
  println!("{}", children.len());
  if children.len() == 1 {
    return match children.pop() {
      Some(v) => return Ok(v),
      None => return Err(String::from("Unexpected error"))
    };
  } else {
    return Ok(Node::Fragment(Fragment { children }));
  }
}


fn parse_nodes<TUntil>(tokenizer: Tokenizer, until: TUntil) -> Result<Vec<Node>, String>
  where TUntil: Fn(Tokenizer) -> bool
 {
  let mut nodes: Vec<Node> = vec![];
  while (until)(tokenizer) {
    match parse_node(tokenizer) {
      Ok(expr) => {
        nodes.push(expr);
      },
      Err(err) => {
        return Err(err);
      }
    }
  }
  Ok(nodes)
}

fn parse_node(tokenizer: Tokenizer) -> Result<Node, String> {
  println!("TOK: {}", String::from_utf8(vec![tokenizer.scanner.curr()]).unwrap());
  match &tokenizer.current {
    Some(token) => {
      match token {
        Token::LessThan => {
          parse_element(tokenizer)
        },
        Token::Unknown(_) => {
          Err(get_unexpected_token_error(&tokenizer))
        },
        _ => {
          Err(get_unexpected_token_error(&tokenizer))
        }
      }
    }
    _ => {
      Err(get_unexpected_token_error(&tokenizer))
    }
  }
}

fn parse_element(tokenizer: Tokenizer) -> Result<Node, String> {
  tokenizer.next(); // eat <
  println!("parse element");
  let tag_name = match &tokenizer.current {
    Some(token) => {
      match token {
        Token::Text(value) => {
          value
        },
        _ => {
          return Err(get_unexpected_token_error(&tokenizer));
        }
      }
    }
    _ => {
      return Err(get_unexpected_token_error(&tokenizer));
    }
  };

  let attributes: Vec<Attribute> = vec![];
  tokenizer.next();

  Ok(Node::Element(Element { tag_name: tag_name.to_owned(), attributes }))
}

fn get_unexpected_token_error(_tokenizer: &Tokenizer) -> String {
  String::from("Unexpected token")
}