use crate::parser::Parser;
use whistle_ast::Grammar;

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
mod program;
pub use program::*;

pub fn parse_grammar(parser: &mut Parser) -> Grammar {
  let mut stmts = Vec::new();

  while parser.within() {
    if let Some(result) = parse_program(parser) {
      stmts.push(result)
    } else {
      println!("ALERT: ParseError at {}", parser.index);
      break;
    }
  }

  stmts
}
