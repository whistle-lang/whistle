use whistle_core::lexer::*;

fn main() {
  let mut lexer = Lexer::new("fun asd(123, 123.123)".to_string());

  loop {
    if let Ok(tok) = lexer.next() {
      println!("{:?}", tok);
      if let TokenValue::EOF = tok.value {
        break;
      }
    } else {
      break;
    }
  }
}
