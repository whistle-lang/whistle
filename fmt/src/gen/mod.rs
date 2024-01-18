use std::borrow::BorrowMut;

use dprint_core::formatting::PrintItems;
use whistle_ast::*;

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
        ident.clone(),
        params.to_vec(),
        &mut ret_type.clone(),
        stmt,
      ),
      _ => panic!("Unimplemented"),
    };
  }
  items
}

pub fn gen_fn(
  items: &mut PrintItems,
  export: &bool,
  _inline: &bool,
  ident: String,
  params: Vec<IdentTyped>,
  ret_type: &mut IdentType,
  stmts: &Vec<Stmt>,
) {
  if *export {
    items.push_str("export ");
  }
  items.push_str("fn ");
  items.push_string(ident);
  items.push_str("(");
  for (i, param) in params.iter().enumerate() {
    if i > 0 {
      items.push_str(", ");
    }
    items.push_string(param.ident.clone());
    items.push_str(": ");
    match param.type_ident.to_type() {
      Type::Ident(ident) => {
        items.push_string(ident);
      }
      _ => panic!("Unimplemented"),
    }
  }
  items.push_str(")");
  items.push_str(": ");
  match ret_type.borrow_mut().to_type() {
    Type::Ident(ident) => {
      items.push_string(ident);
    }
    _ => panic!("Unimplemented"),
  }
  // items.push_str(&ret_type.to_type().type_id());

  items.push_str(" {\n");
  for stmt in stmts {
    gen_stmt(items, stmt);
  }
  items.push_str("}\n");
}

pub fn gen_stmt(_items: &mut PrintItems, stmt: &Stmt) {
  match stmt {
    _ => panic!("Unimplemented"),
  }
}
