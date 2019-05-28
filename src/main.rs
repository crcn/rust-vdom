mod tokenizer;
mod scanner;
mod ast;
mod parser;
use parser::*;

fn main() {
    match parse("<a />") {
        Ok(ast) => {
            println!("{:?}", ast);
        },
        Err(err) => {
            println!("ERR: {}", err);
        }
    }
}

