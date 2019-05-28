mod tokenizer;
mod scanner;
mod ast;
mod parser;
use parser::*;

fn main() {
    parse("<a />");
}

