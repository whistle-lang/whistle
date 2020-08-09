use std::time::{Instant};

use whistle_core::lexer::*;

fn main() {
  let lexer = Lexer::new(r#"// hello world
  /* hello world */
  h3110 w0rld
  Import As From Export Fun Return If Else While Break Continue Var Val For In Match Type Struct Trait
  ~ ! + - * / % ** == != <= < > >= && || << >> & | ^ += -= * /= %= **= &&= ||= <<= >>= &= |= ^=
  123.123 123e123 123.123e123
  0b01 0o01234567 0x0123456789abcdef 0123456789
  "hello world" "\""
  'c' '\''
  true false
  none
  #(asd) asd
  #( asd ) {
    asd
  }
  ,:.[]{}()
  "#.to_string());
  
  let now = Instant::now();

  for tok in lexer {
    println!("{:?}", tok);
    if tok.is_err() {
      break;
    }
  }

  println!("{} s", now.elapsed().as_secs_f64());
}
