use crate::parser::Parser;
use crate::ParserError;
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

pub fn parse_grammar(parser: &mut Parser) -> Result<Grammar, ParserError> {
  let mut stmts = Vec::new();

  while parser.within() {
    match parse_program(parser) {
      Ok(stmt) => stmts.push(stmt),
      Err(err) => return Err(err),
    }
  }

  Ok(stmts)
}
