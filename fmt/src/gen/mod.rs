use std::borrow::BorrowMut;

use dprint_core::formatting::PrintItems;
use whistle_ast::{IdentType, IdentTyped, ProgramStmt, Stmt};

use crate::{config::Configuration, ParsedSource};

pub fn generate(parsed_source: &ParsedSource, _config: &Configuration) -> PrintItems {
  let mut items = PrintItems::new();
  let ast = &parsed_source.grammar;
  for item in ast {
    match item {
      ProgramStmt::FunctionDecl {
        export,
        inline,
        ident,
        params,
        ret_type,
        stmt,
        ..
      } => gen_fn(
        items.borrow_mut(),
        export,
        inline,
        ident,
        params,
        ret_type,
        stmt,
      ),
      _ => panic!("Unimplemented"),
    };
  }
  items
}

pub fn gen_fn(
  _items: &mut PrintItems,
  _export: &bool,
  _inline: &bool,
  _ident: &str,
  _params: &Vec<IdentTyped>,
  _ret_type: &IdentType,
  _stmts: &Vec<Stmt>,
) {
}
