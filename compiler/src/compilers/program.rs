use crate::compile_stmts;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Function;
use crate::Symbol;

use wasm_encoder::Export;
use wasm_encoder::GlobalType;
use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;

pub fn compile_program(compiler: &mut Compiler, program: ProgramStmt) {
  match program {
    ProgramStmt::FunDecl {
      export,
      ident,
      params,
      ret_type,
      stmt,
    } => compile_fun(compiler, export, ident, params, ret_type, stmt),
    ProgramStmt::ValDecl { ident_typed, val } => compile_val(compiler, ident_typed, val),
    ProgramStmt::VarDecl { ident_typed, val } => compile_var(compiler, ident_typed, val),
    // ProgramStmt::Stmt(Stmt) =>
    // ProgramStmt::Import { .. } => panic!("Imports are not yet supported"),
    _ => (),
  }
}

pub fn compile_fun(
  compiler: &mut Compiler,
  export: bool,
  ident: String,
  params: Vec<IdentTyped>,
  ret_type: IdentType,
  stmts: Vec<Stmt>,
) {
  let idx = compiler
    .scope
    .set_fun_sym(
      &ident,
      Symbol {
        global: true,
        mutable: false,
        types: IdentType::Function {
          params: params.clone(),
          ret_type: Box::new(ret_type.clone()),
        },
      },
    )
    .unwrap();
  compiler.scope.enter_scope();

  let mut types = Vec::new();

  for param in params {
    compiler
      .scope
      .set_local_sym(
        &param.ident,
        Symbol {
          global: false,
          mutable: true,
          types: param.type_ident.clone(),
        },
      )
      .unwrap();

    types.push(ident_type_to_val_type(param.type_ident));
  }

  let ret_type = ident_type_to_val_type(ret_type);

  compiler.module.types.function(types, vec![ret_type]);
  compiler.module.funs.function(idx);

  if export {
    compiler
      .module
      .exports
      .export(&ident, Export::Function(idx));
  }

  let mut fun = Function::new();
  compile_stmts(compiler, &mut fun, stmts);
  fun.instruction(Instruction::End);
  compiler.module.code.function(&fun.into());
  compiler.scope.exit_scope();
}

pub fn compile_val(compiler: &mut Compiler, ident_typed: IdentTyped, _val: Expr) {
  compiler
    .scope
    .set_global_sym(
      &ident_typed.ident,
      Symbol {
        global: true,
        mutable: false,
        types: ident_typed.type_ident.clone(),
      },
    )
    .unwrap();

  compiler.module.globals.global(
    GlobalType {
      val_type: ident_type_to_val_type(ident_typed.type_ident),
      mutable: false,
    },
    Instruction::I32Const(0),
  );
}

pub fn compile_var(compiler: &mut Compiler, ident_typed: IdentTyped, _val: Expr) {
  compiler
    .scope
    .set_global_sym(
      &ident_typed.ident,
      Symbol {
        global: true,
        mutable: true,
        types: ident_typed.type_ident.clone(),
      },
    )
    .unwrap();

  compiler.module.globals.global(
    GlobalType {
      val_type: ident_type_to_val_type(ident_typed.type_ident),
      mutable: true,
    },
    Instruction::I32Const(0),
  );
}
