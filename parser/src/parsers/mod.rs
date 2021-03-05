mod expr;
pub use expr::*;
mod ident;
pub use ident::*;
mod literal;
pub use literal::*;
mod stmt;
pub use stmt::*;
mod program;
pub use program::*;
mod types;
pub use types::*;

use crate::Parser;
use crate::ParserError;
use whistle_ast::Grammar;

pub fn parse_all(parser: &mut Parser) -> Result<Grammar, ParserError> {
  let mut stmts = Vec::new();
  let mut errs = ParserError { err: Vec::new() };
  while parser.within() {
    let res = parse_program(parser);
    if let Ok(val) = res {
      stmts.push(val);
    } else if let Err(val) = res {
      errs.extend(val);
      parser.step();
    }
  }
  if errs.err.len() > 0 {
    return Err(errs);
  }
  Ok(stmts)
}
