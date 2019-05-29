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


fn parse_nodes<TTest>(tokenizer: &mut Tokenizer, test: TTest) -> Result<Vec<Node>, String>
  where TTest: Fn(&mut Tokenizer) -> bool
 {
  let mut nodes: Vec<Node> = vec![];
  while (test)(tokenizer) {
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
        Token::Text(_) => {
          parse_text(tokenizer)
        }
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

  let children: Option<Vec<Node>> = match &tokenizer.current {
    Some(token) => match token {
      Token::GreaterThan => {
        tokenizer.next();
        match parse_children(&tag_name, tokenizer) {
          Ok(children) => Some(children),
          Err(err) => return Err(err)
        }
      },
      Token::SelfCloseTag => {
        tokenizer.next();  // eat />
        None
      },
      _ => {
        return Err(get_unexpected_token_error(tokenizer));
      }
    },
    None => {
      return Err(get_unexpected_token_error(tokenizer));
    }
  };

  match children {
    Some(children) => {
      match &tokenizer.current {
        Some(token) => match token {
          Token::StartCloseTag => {
            tokenizer.next(); // eat </
          }
          _ => {
            println!("TOK: {:?}", token);
            return Err(get_unexpected_token_error(tokenizer));
          }
        },
        None => {
          return Err(get_unexpected_token_error(tokenizer));
        }
      }
      tokenizer.next(); // eat tag name
      tokenizer.next(); // eat >
      Ok(Node::Element(Element { tag_name: tag_name.to_owned(), attributes, children }))
    },
    None => {
      Ok(Node::Element(Element { tag_name: tag_name.to_owned(), attributes, children: vec![] }))
    }
  }
  
}

fn parse_text(tokenizer: &mut Tokenizer) -> Result<Node, String> {
  let mut buffer = String::from("");
  loop {
    match &tokenizer.current {
      Some(token) => {
        match token {
          Token::Text(value) => {
            buffer.push_str(value);
          }
          Token::LessThan | Token::StartCloseTag => {
            break;
          }
          _a => {
            println!("Unhandled token found {:?}", _a);
          }
        }
      },
      None => {
        break;
      }
    }
    tokenizer.next();
  }

  Ok(Node::Text(Text { value: buffer }))
}

fn parse_children(tag_name: &String, tokenizer: &mut Tokenizer) -> Result<Vec<Node>, String> {
  let test = |tokenizer: &mut Tokenizer| match &tokenizer.current {
    Some(token) => {
      token != &Token::StartCloseTag
    },
    None => true
  };
  
  match parse_nodes(tokenizer, test) {
    Ok(children) => Ok(children),
    Err(err) => Err(err)
  }
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
  println!("Token: {:?}", tokenizer.current);
  
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