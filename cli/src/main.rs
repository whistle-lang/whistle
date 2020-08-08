use whistle_core::lexer::*;

fn main() {
  let lexer = Lexer::new("fun asd(123, 123.123)".to_string());

  for tok in lexer {
    println!("{:?}", tok);
  }
}
