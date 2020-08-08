use whistle_core::lexer::*;

fn main() {
  let lexer = Lexer::new("export fun add(a: i32, a: i32): i32 {
    return a + b
  }".to_string());

  for tok in lexer {
    println!("{:?}", tok);
  }
}
