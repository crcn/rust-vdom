mod tokenizer;
mod scanner;
mod ast;
mod parser;
mod diff_patch;
use parser::*;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {

    let then = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(_) => panic!("Cannot get time")
    };

    let source = "
        <div style='color: red;'>
            Hello
        </div>
    ";

    match parse(source) {
        Ok(ast) => {
            println!("{:?}", ast);
        },
        Err(err) => {
            println!("ERR: {}", err);
        }
    }

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(_) => panic!("ER")
    };

    println!("{:?}", now - then);
}
