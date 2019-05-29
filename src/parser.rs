use crate::scanner::*;
use crate::tokenizer::*;
use crate::ast::*;
// use crate::tokenizer;
// use scanner;




pub fn parse(source: &str) -> Result<Node, String> {
  let scanner = Scanner::new(source);
  let mut tokenizer = Tokenizer::new(scanner);
  parse_root(&mut tokenizer)
}

fn parse_root(tokenizer: &mut Tokenizer) -> Result<Node, String> {
  let mut children = match parse_nodes(tokenizer, |tokenizer| !tokenizer.ended()) {
    Ok(nodes) => nodes,
    Err(e) => {
      return Err(e);
    }
  };
  if children.len() == 1 {
    return match children.pop() {
      Some(v) => return Ok(v),
      None => return Err(String::from("Unexpected error"))
    };
  } else {
    return Ok(Node::Fragment(Fragment { children }));
  }
}


fn parse_nodes<TUntil>(tokenizer: &mut Tokenizer, until: TUntil) -> Result<Vec<Node>, String>
  where TUntil: Fn(&mut Tokenizer) -> bool
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

fn parse_node(tokenizer: &mut Tokenizer) -> Result<Node, String> {
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

fn parse_element(tokenizer: &mut Tokenizer) -> Result<Node, String> {
  tokenizer.next(); // eat <
  let tag_name = match &tokenizer.current {
    Some(token) => {
      match token {
        Token::Text(value) => {
          value.to_owned()
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

  tokenizer.next(); // eat tag name

  let attributes: Vec<Attribute> = match parse_attributes(tokenizer) {
    Ok(attrs) => attrs,
    Err(err) => return Err(err)
  };

  tokenizer.next();  // eat />

  Ok(Node::Element(Element { tag_name: tag_name.to_owned(), attributes }))
}

fn parse_attributes(tokenizer: &mut Tokenizer) -> Result<Vec<Attribute>, String> {
  let mut attrs: Vec<Attribute> = vec![];
  loop {
    tokenizer.eat_whitespace();
    match &tokenizer.current {
      Some(token) => {
        match token {
          Token::SelfCloseTag | Token::GreaterThan => {
            break;
          }
          _ => {
            match parse_attribute(tokenizer) {
              Ok(attr) => attrs.push(attr),
              Err(err) => return Err(err)
            }
          }
        }
      }
      None => {
        break;
      }
    }
  }
  Ok(attrs)
}

fn parse_attribute(tokenizer: &mut Tokenizer) -> Result<Attribute, String> {
  let name = match &tokenizer.current {
    Some(token) => match token {
      Token::Text(value) => value.to_owned(),
      _ => return Err(get_unexpected_token_error(tokenizer))
    },
    None => return Err(get_unexpected_token_error(tokenizer))
  };


  tokenizer.next(); // eat name
  println!("{:?}", tokenizer.current);
  
  let value = match &tokenizer.current {
    Some(token) => match token {
      Token::Equals => {
        match &tokenizer.next() {
          Some(token) => match token {
            Token::String(value) => {
              Some(value.to_owned())
            },
            _ => {
              return Err(get_unexpected_token_error(tokenizer))
            }
          },
          _ => {
            return Err(get_unexpected_token_error(tokenizer));
          }
        }
      },
      _ => {
        None
      }
    },
    None => {
      None
    }
  };

  match value {
    Some(value) => {
      tokenizer.next();
      Ok(Attribute { name, value })
    },
    None => Ok(Attribute { name, value: String::from("")})
  }
}

fn get_unexpected_token_error(_tokenizer: &Tokenizer) -> String {
  String::from("Unexpected token")
}