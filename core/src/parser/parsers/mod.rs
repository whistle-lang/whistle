use crate::parser::ast::*;
use crate::parser::Parser;

mod expr;
pub use expr::*;
mod ident;
pub use ident::*;
mod literal;
pub use literal::*;
mod stmt;
pub use stmt::*;
mod operator;
pub use operator::*;

pub fn parse_grammar(parser: &mut Parser) -> Vec<Stmt> {
  let mut stmts: Vec<Stmt> = Vec::new();

  while parser.within() {
    if let Some(result) = parse_stmt(parser) {
      stmts.push(result)
    } else {
      println!("ALERT: ParseError at {}", parser.index);
      break;
    }
  }

  // println!(
  //   "
  // Result:
  // {:?}",
  //   stmts
  // );

  stmts
}
