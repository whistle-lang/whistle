mod expr;
mod ident;
mod literal;
mod program;
mod stmt;
mod types;

use crate::Parser;
use whistle_common::ParserHandler;

pub use expr::*;
pub use ident::*;
pub use literal::*;
pub use program::*;
pub use stmt::*;
pub use types::*;

use whistle_ast::Grammar;

pub fn parse_all(parser: &mut Parser) -> Grammar {
  let mut ok = true;
  let mut stmts = Vec::new();
  while parser.within() {
    let res = parse_program(parser);
    if let Ok(val) = res {
      ok = true;
      stmts.push(val);
    } else if let Err(val) = res {
      if ok {
        parser.handler.throw(val.kind, val.span);
      }
      parser.step();
      ok = false;
    }
  }
  stmts
}
