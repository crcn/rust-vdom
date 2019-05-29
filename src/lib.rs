
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod tokenizer;
mod scanner;
mod ast;
mod parser;
mod diff_patch;

#[wasm_bindgen]
pub fn parse(value: &str) -> String {
  parser::parse(value);
  println!("OK");
  return String::from("ok");
}