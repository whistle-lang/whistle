use std::time::Instant;

use whistle_core::lexer::*;

fn main() {
  let source = r#"
  // hello world
  /* hello world */
  h3110 w0rld abcdefghijklmnopqrstuvwxyz asd1_asd åäöÅÄÖ 你好吗
  import as from export fun return if else while break continue var val for in match type struct trait
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
  , : . [ ] { } ( )
  "#;

  let lexer = Lexer::new(source);

  let mut toks = vec![];

  let now = Instant::now();

  for tok in lexer {
    // println!("{:?}", tok);
    toks.push(tok.clone());
    if tok.is_err() {
      break;
    }
  }

  // println!("{:?}", toks);
  println!(
    "{} chars parsed into {} tokens in {} s",
    source.len(),
    toks.len(),
    now.elapsed().as_secs_f64()
  );
}
