mod ast;
pub use ast::*;
mod parser;
pub use parser::*;

pub fn asd() {
  let parser = Parser::new(vec![]);

  match parser.peek() {
    Some(Token::Keyword(keyword)) => {
      println!("{:?}", keyword);
    }
    Some(Token::Operator(op)) => {
      println!("{:?}", op);
    }
    _ => (),
  }
}
