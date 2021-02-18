use crate::Compiler;
use crate::CompilerErrorKind;

use std::collections::HashMap;

use whistle_ast::Var;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::IdentTypedStrict;
use whistle_ast::ProgramStmt;
use whistle_common::Primitive;

pub fn bind_program(compiler: &mut Compiler, program: ProgramStmt) {
  match program {
    ProgramStmt::FunDecl {
      ident,
      params,
      ret_type,
      ..
    } => bind_function(compiler, ident, params, ret_type),
    ProgramStmt::StructDecl { ident, params, .. } => bind_struct(compiler, ident, params),
    ProgramStmt::TypeDecl { ident, types, .. } => bind_type(compiler, ident, types),
    // ProgramStmt::ValDecl { ident_typed, val: _ } =>
    //   Ok(bind_vars(&mut table.vars, ident_typed, true)),
    // ProgramStmt::VarDecl { ident_typed, val: _ } =>
    //   Ok(bind_vars(&mut table.vars, ident_typed, false)),
    // ProgramStmt::Stmt(Stmt) =>
    // ProgramStmt::Import { .. } => panic!("Imports are not yet supported"),
    _ => (),
  }
}

pub fn bind_struct(compiler: &mut Compiler, ident: String, params: Vec<IdentTypedStrict>) {
  if compiler.table.types.get(&ident).is_none() {
    let types = IdentType::Struct(params);
    compiler.table.types.insert(ident, types);
  }
  compiler.throw(CompilerErrorKind::TypeRedefinition, 0)
}

pub fn bind_type(compiler: &mut Compiler, ident: String, types: IdentType) {
  if compiler.table.types.get(&ident).is_none() {
    compiler.table.types.insert(ident, types);
  }
  compiler.throw(CompilerErrorKind::TypeRedefinition, 0)
}

pub fn bind_function(
  compiler: &mut Compiler,
  ident: String,
  args: Vec<IdentTyped>,
  ret_type: Option<IdentType>,
) {
  if compiler.table.vars.get(&ident).is_none() {
    let mut params = HashMap::new();
    for param in args {
      if let Some(types) = param.type_ident {
        let mutable = false;
        let var = Var { mutable, types };
        params.insert(param.ident, var);
      } else {
        //noImplicitAny
        compiler.throw(CompilerErrorKind::ExpectedParamType, 0)
      }
    }
    let ret_type = if let Some(ret_type) = ret_type {
      Box::new(ret_type)
    } else {
      Box::new(IdentType::Primitive(Primitive::Void))
    };
    let func = IdentType::Function {params, ret_type};
    let var = Var {mutable: false, types: func};
    compiler.table.vars.insert(ident, var);
  } else {
    compiler.throw(CompilerErrorKind::FuncRedefinition, 0)
  }
}
