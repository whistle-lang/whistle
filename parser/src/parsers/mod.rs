mod expr;
mod ident;
mod literal;
mod program;
mod stmt;
mod types;

use crate::Parser;
use crate::ParserError;

pub use expr::*;
pub use ident::*;
pub use literal::*;
pub use program::*;
pub use stmt::*;
pub use types::*;

use whistle_ast::Grammar;

pub fn parse_all(parser: &mut Parser) -> Result<Grammar, ParserError> {
  let mut ok = true;
  let mut stmts = Vec::new();
  let mut errs = ParserError { err: Vec::new() };
  while parser.within() {
    let res = parse_program(parser);
    if let Ok(val) = res {
      ok = true;
      stmts.push(val);
    } else if let Err(val) = res {
      if ok {
        errs.extend(val);
      } else {
        errs.range(val.index().end);
      }
      parser.step();
      ok = false;
    }
  }
  if !errs.err.is_empty() {
    Err(errs)
  } else {
    Ok(stmts)
  }
}
