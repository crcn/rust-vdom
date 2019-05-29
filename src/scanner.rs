use std::cmp::min;
use std::cmp::max;

pub struct Scanner<'a> {
  source: &'a [u8],
  index: usize
}

impl<'a> Scanner<'a> {
   pub fn new(source: &'a str) -> Scanner {
    Scanner { source: source.as_bytes(), index: 0 }
  }

  pub fn curr(&self) -> u8 {
    self.source[self.index]
  }

  pub fn next(&mut self) -> Option<u8> {
    self.index = min(self.source.len(), self.index + 1);
    if self.index < self.source.len() {
      Some(self.source[self.index])
    } else {
      None
    }
  }

  pub fn prev(&mut self) -> Option<u8> {
    self.index = max(0, self.index - 1);
    if self.index > 0 {
      Some(self.source[self.index])
    } else {
      None
    }
  }

  pub fn peek(&mut self, step: usize) -> Option<u8> {
    let index = self.index;
    self.index = index + (step - 1);
    let c = self.next();
    self.index = index;
    return c;
  }

  pub fn scan<TFun>(&mut self, filter: TFun) -> Vec<u8>
    where TFun: Fn(u8) -> bool
   {
     let mut buffer: Vec<u8> = vec![];
     loop {
       match self.next() {
         Some(c) => {
          if filter(c) {
            buffer.push(c);
          } else {
            self.prev();
            break;
          }
         },
         None => {
           break;
         },
       };
     }

     return buffer;
  }

  pub fn ended(&self) -> bool {
    self.index == self.source.len()
  }
}