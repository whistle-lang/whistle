use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Var;

use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;

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

pub fn bind_struct(compiler: &mut Compiler, ident: String, params: Vec<IdentTyped>) {
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
  ret_type: IdentType,
) {
  if compiler.table.vars.get(&ident).is_none() {
    let mut params = Vec::new();
    for mut arg in args {
      arg.type_ident = compiler.no_implicit_any(arg.type_ident);
      params.push(arg);
    }
    let ret_type = Box::new(compiler.no_implicit_any(ret_type));
    let func = IdentType::Function { params, ret_type };
    let var = Var {
      mutable: false,
      types: func,
    };
    compiler.table.vars.insert(ident, var);
  } else {
    compiler.throw(CompilerErrorKind::FuncRedefinition, 0)
  }
}
