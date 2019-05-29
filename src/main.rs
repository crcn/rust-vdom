mod tokenizer;
mod scanner;
mod ast;
mod parser;
use parser::*;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {

    let then = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(_) => panic!("Cannot get time")
    };

    for i in 0..100 {
        match parse("<a b='a'></a>") {
            Ok(ast) => {
                // println!("{:?}", ast);
            },
            Err(err) => {
                // println!("ERR: {}", err);
            }
        }
    }

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(_) => panic!("ER")
    };

    println!("{:?}", now - then);
}
