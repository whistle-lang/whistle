mod expression;
mod ident;
mod literal;
mod statement;
pub use super::*;
pub use crate::lexer::*;
pub use statement::*;

pub fn parse_grammar(parser: &mut Parser) {
  //TODO: parse_functions, codeblocks
  let mut stmts: Vec<Stmt> = Vec::new();
  while parser.within() {
    if let Some(result) = parse_stmt(parser) {
      stmts.push(result)
    } else {
      println!("ALERT: ParseError");
      break;
    }
  }
  println!(
    "
  Result: 
  {:?}",
    stmts
  );
}
