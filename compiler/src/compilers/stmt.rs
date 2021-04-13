use crate::compile_bool_expr;
use crate::compile_expr;
use crate::errors::CompilerErrorKind;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Function;
use crate::IndexedSymbol;
use crate::Symbol;

use wasm_encoder::BlockType;
use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Stmt;

pub fn compile_stmt(compiler: &mut Compiler, fun: &mut Function, stmt: Stmt) {
  match stmt {
    Stmt::While { cond, do_stmt } => compile_while(compiler, fun, cond, do_stmt),
    Stmt::ValDecl { ident_typed, val } => compile_val_decl(compiler, fun, ident_typed, val),
    Stmt::VarDecl { ident_typed, val } => compile_var_decl(compiler, fun, ident_typed, val),
    Stmt::Assign { op, rhs, ident } => compile_assign(compiler, fun, op, rhs, ident),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
    } => compile_if(compiler, fun, cond, then_stmt, else_stmt),
    Stmt::Expr(args) => compile_expr_stmt(compiler, fun, args),
    Stmt::Block(args) => compile_block(compiler, fun, args),
    Stmt::Return(expr) => compile_return(compiler, fun, expr),
    _ => compiler.throw(CompilerErrorKind::Unimplemented, 0),
  }
}

pub fn compile_stmts(compiler: &mut Compiler, fun: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_scope();
  for stmt in stmts {
    compile_stmt(compiler, fun, stmt);
  }
  compiler.scope.exit_scope();
}

pub fn compile_while(compiler: &mut Compiler, fun: &mut Function, cond: Expr, do_stmt: Vec<Stmt>) {
  fun.instruction(Instruction::Block(BlockType::Empty));
  fun.instruction(Instruction::Loop(BlockType::Empty));
  compile_bool_expr(compiler, fun, cond);
  fun.instruction(Instruction::BrIf(1));
  compile_stmts(compiler, fun, do_stmt);
  fun.instruction(Instruction::Br(0));
  fun.instruction(Instruction::End);
  fun.instruction(Instruction::End);
}

pub fn compile_if(
  compiler: &mut Compiler,
  fun: &mut Function,
  cond: Expr,
  then_stmt: Vec<Stmt>,
  else_stmt: Option<Vec<Stmt>>,
) {
  compile_bool_expr(compiler, fun, cond);
  fun.instruction(Instruction::If(BlockType::Empty));
  compile_stmts(compiler, fun, then_stmt);

  if let Some(stmt) = else_stmt {
    fun.instruction(Instruction::Else);
    compile_stmts(compiler, fun, stmt);
  }

  fun.instruction(Instruction::End);
}

pub fn compile_val_decl(compiler: &mut Compiler, fun: &mut Function, ident: IdentTyped, val: Expr) {
  compiler.scope.expr_type = ident.type_ident.clone();
  let types = compile_expr(compiler, fun, val);
  if ident.type_ident != types {
    compiler.throw(CompilerErrorKind::TypeMismatch, 0)
  }

  let idx = match compiler.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: false,
      types: types.clone(),
    },
  ) {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, 0);
      0
    }
  };

  fun.local(idx, ident_type_to_val_type(types));
  fun.instruction(Instruction::LocalSet(idx));
}

pub fn compile_var_decl(compiler: &mut Compiler, fun: &mut Function, ident: IdentTyped, val: Expr) {
  compiler.scope.expr_type = ident.type_ident.clone();
  let types = compile_expr(compiler, fun, val);
  if ident.type_ident != types {
    compiler.throw(CompilerErrorKind::TypeMismatch, 0)
  }

  let idx = match compiler.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: true,
      types: types.clone(),
    },
  ) {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, 0);
      0
    }
  };

  fun.local(idx, ident_type_to_val_type(types));
  fun.instruction(Instruction::LocalSet(idx));
}

pub fn compile_block(compiler: &mut Compiler, fun: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_scope();
  fun.instruction(Instruction::Loop(BlockType::Empty));
  for stmt in stmts {
    compile_stmt(compiler, fun, stmt)
  }
  fun.instruction(Instruction::End);
  compiler.scope.exit_scope();
}

pub fn compile_return(compiler: &mut Compiler, fun: &mut Function, expr: Option<Expr>) {
  let ident_type = match compiler.scope.get_sym(&fun.ident) {
    Ok(IndexedSymbol(_, symbol)) => symbol.types.clone(),
    Err(err) => {
      compiler.throw(err, 0);
      IdentType::Error
    }
  };

  if let Some(expr) = expr {
    compiler.scope.expr_type = ident_type;
    compile_expr(compiler, fun, expr);
  }

  fun.instruction(Instruction::Return);
}

pub fn compile_assign(
  compiler: &mut Compiler,
  fun: &mut Function,
  _op: Operator,
  rhs: Expr,
  ident: String,
) {
  let sym = match compiler.scope.get_sym(&ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      compiler.throw(err, 0);
      IndexedSymbol(0, Symbol::default())
    }
  };
  compiler.scope.expr_type = sym.1.types.clone();
  let expr = compile_expr(compiler, fun, rhs);
  if !sym.1.mutable {
    compiler.throw(CompilerErrorKind::ImmutableAssign, 0)
  }

  if sym.1.types != expr {
    compiler.throw(CompilerErrorKind::TypeMismatch, 0)
  }

  if sym.1.global {
    fun.instruction(Instruction::GlobalSet(sym.0));
  } else {
    fun.instruction(Instruction::LocalSet(sym.0));
  }
}

pub fn compile_expr_stmt(compiler: &mut Compiler, fun: &mut Function, expr: Expr) {
  compiler.scope.expr_type = IdentType::Default;
  compile_expr(compiler, fun, expr);
}
