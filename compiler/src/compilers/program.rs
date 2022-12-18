use crate::compile_stmts;
use crate::ident_type_to_val_type;
use crate::setup_extern;
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
use whistle_ast::IdentExternFn;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;
use whistle_ast::Type;

pub fn compile_program(compiler: &mut Compiler, program: ProgramStmt) {
  match program {
    ProgramStmt::Extern {
      idents, namespace, ..
    } => compile_extern(compiler, idents, namespace),
    ProgramStmt::FunctionDecl {
      export,
      inline,
      ident,
      params,
      ret_type,
      stmt,
      ..
    } => compile_fn(compiler, export, inline, ident, params, ret_type, stmt),
    ProgramStmt::ValDecl {
      ident_typed, val, ..
    } => compile_val(compiler, ident_typed, val),
    ProgramStmt::VarDecl {
      ident_typed, val, ..
    } => compile_var(compiler, ident_typed, val),
    // ProgramStmt::Stmt(Stmt) =>
    ProgramStmt::Import {
      idents: _idents,
      from: _from,
      imp_type: _imp_type,
      ..
    } => {}
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
      types: Type::Function {
        params: IdentTyped::vec_to_type(&params),
        ret_type: Box::new(ret_type.to_type()),
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
        types: param.type_ident.to_type(),
      },
    ) {
      compiler.throw(err, 0);
    }

    types.push(ident_type_to_val_type(param.type_ident.to_type()));
  }

  let encoded_ret_type = if let IdentType::Primitive { .. } = ret_type {
    vec![]
  } else {
    vec![ident_type_to_val_type(ret_type.to_type())]
  };

  compiler.module.types.function(types, encoded_ret_type);
  compiler.module.fns.function(idx);
  if export {
    compiler.module.exports.export(
      if &ident == "main" { "_start" } else { &ident },
      ExportKind::Func,
      idx,
    );
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
//   _types: Type,
// ) {
// }

pub fn compile_extern(compiler: &mut Compiler, idents: Vec<IdentExternFn>, namespace: String) {
  for external_fn in &idents {
    let types = Type::Function {
      params: IdentTyped::vec_to_type(&external_fn.params),
      ret_type: Box::new(external_fn.ret_type.to_type()),
    };
    setup_extern(compiler, &namespace, external_fn.ident.as_str(), types)
  }
}

pub fn compile_val(compiler: &mut Compiler, ident_typed: IdentTyped, _val: Expr) {
  if let Err(err) = compiler.scope.set_global_sym(
    &ident_typed.ident,
    Symbol {
      global: true,
      mutable: false,
      types: ident_typed.type_ident.to_type(),
    },
  ) {
    compiler.throw(err, 0);
  }

  let val_type = ident_type_to_val_type(ident_typed.type_ident.to_type());

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
      types: ident_typed.type_ident.to_type(),
    },
  ) {
    compiler.throw(err, 0);
  }

  let val_type = ident_type_to_val_type(ident_typed.type_ident.to_type());

  compiler.module.globals.global(
    GlobalType {
      val_type,
      mutable: true,
    },
    &ConstExpr::i32_const(0),
  );
}
