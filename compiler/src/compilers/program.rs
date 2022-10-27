use crate::compile_builtins_sys;
use crate::compile_stmts;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Function;
use crate::Symbol;

// use wasm_encoder::EntityType;
use wasm_encoder::ConstExpr;
use wasm_encoder::ExportKind;
use wasm_encoder::GlobalType;
use wasm_encoder::Instruction;
use whistle_ast::Expr;
use whistle_ast::IdentBuiltin;
// use whistle_ast::IdentImport;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
// use whistle_ast::Primitive;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;

pub fn compile_program(compiler: &mut Compiler, program: ProgramStmt) {
  match program {
    ProgramStmt::FunctionDecl {
      export,
      inline,
      ident,
      params,
      ret_type,
      stmt,
    } => compile_fn(compiler, export, inline, ident, params, ret_type, stmt),
    ProgramStmt::ValDecl { ident_typed, val } => compile_val(compiler, ident_typed, val),
    ProgramStmt::VarDecl { ident_typed, val } => compile_var(compiler, ident_typed, val),
    // ProgramStmt::Stmt(Stmt) =>
    // ProgramStmt::Import {
    //   idents,
    //   from,
    //   imp_type,
    // } => compile_import(
    //   compiler,
    //   idents,
    //   from,
    //   imp_type,
    //   IdentType::Function {
    //     params: vec![IdentTyped {
    //       ident: String::from("value"),
    //       type_ident: IdentType::Primitive(Primitive::I32),
    //     }],
    //     ret_type: Box::new(IdentType::Primitive(Primitive::None)),
    //   },
    // ),
    ProgramStmt::Builtin { idents, from } => compile_builtins(compiler, idents, from),
    _ => compiler.throw(CompilerErrorKind::Unimplemented, 0),
  }
}

pub fn compile_fn(
  compiler: &mut Compiler,
  export: bool,
  _inline: bool,
  ident: String,
  params: Vec<IdentTyped>,
  ret_type: IdentType,
  stmts: Vec<Stmt>,
) {
  // TODO: Inline functions, would be done with a new field in the Compiler struct

  let idx = match compiler.scope.set_function_sym(
    &ident,
    Symbol {
      global: true,
      mutable: false,
      types: IdentType::Function {
        params: params.clone(),
        ret_type: Box::new(ret_type.clone()),
      },
    },
  ) {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, 0);
      0
    }
  };
  compiler.scope.enter_scope();

  let mut types = Vec::new();

  for param in params {
    if let Err(err) = compiler.scope.set_local_sym(
      &param.ident,
      Symbol {
        global: false,
        mutable: true,
        types: param.type_ident.clone(),
      },
    ) {
      compiler.throw(err, 0);
    }

    types.push(ident_type_to_val_type(param.type_ident));
  }

  let ret_type = ident_type_to_val_type(ret_type);

  compiler.module.types.function(types, vec![ret_type]);
  compiler.module.fns.function(idx);
  if export {
    compiler
      .module
      .exports
      .export(&ident, ExportKind::Func, idx);
  }

  let mut fun = Function::new(ident);
  compile_stmts(compiler, &mut fun, stmts);
  fun.instruction(Instruction::End);
  compiler.module.code.function(&fun.into());
  compiler.scope.exit_scope();
}

// pub fn compile_import(
//   _compiler: &mut Compiler,
//   _idents: Vec<IdentImport>,
//   _from: String,
//   _imp_type: String,
//   _types: IdentType,
// ) {

// }

pub fn compile_builtins(compiler: &mut Compiler, idents: Vec<IdentBuiltin>, from: String) {
  match from.as_str() {
    "sys" => compile_builtins_sys(compiler, idents),
    _ => unimplemented!()
  }
}

pub fn compile_val(compiler: &mut Compiler, ident_typed: IdentTyped, _val: Expr) {
  if let Err(err) = compiler.scope.set_global_sym(
    &ident_typed.ident,
    Symbol {
      global: true,
      mutable: false,
      types: ident_typed.type_ident.clone(),
    },
  ) {
    compiler.throw(err, 0);
  }

  let val_type = ident_type_to_val_type(ident_typed.type_ident);

  compiler.module.globals.global(
    GlobalType {
      val_type,
      mutable: false,
    },
    &ConstExpr::i32_const(0),
  );
}

pub fn compile_var(compiler: &mut Compiler, ident_typed: IdentTyped, _val: Expr) {
  if let Err(err) = compiler.scope.set_global_sym(
    &ident_typed.ident,
    Symbol {
      global: true,
      mutable: true,
      types: ident_typed.type_ident.clone(),
    },
  ) {
    compiler.throw(err, 0);
  }

  let val_type = ident_type_to_val_type(ident_typed.type_ident);

  compiler.module.globals.global(
    GlobalType {
      val_type,
      mutable: true,
    },
    &ConstExpr::i32_const(0),
  );
}
