use crate::compile_expr;
use crate::errors::CompilerErrorKind;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Function;
use crate::Symbol;

use wasm_encoder::BlockType;
use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Stmt;

use whistle_common::Primitive;

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
    _ => panic!("stmt {:?}", stmt),
  }
}

pub fn compile_bool(compiler: &mut Compiler, fun: &mut Function, expr: Expr) {
  if IdentType::Primitive(Primitive::Bool) != compile_expr(compiler, fun, expr) {
    compiler.throw(CompilerErrorKind::ExpectedBooleanExpr, 0)
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
  compile_bool(compiler, fun, cond);
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
  compile_bool(compiler, fun, cond);
  fun.instruction(Instruction::If(BlockType::Empty));
  compile_stmts(compiler, fun, then_stmt);

  if let Some(stmt) = else_stmt {
    fun.instruction(Instruction::Else);
    compile_stmts(compiler, fun, stmt);
  }

  fun.instruction(Instruction::End);
}

pub fn compile_val_decl(compiler: &mut Compiler, fun: &mut Function, ident: IdentTyped, val: Expr) {
  let types = compile_expr(compiler, fun, val);
  if ident.type_ident != types {
    compiler.throw(CompilerErrorKind::IncompatibleTypes, 0)
  }

  let idx = compiler
    .scope
    .set_local_sym(
      &ident.ident,
      Symbol {
        global: false,
        mutable: false,
        types: types.clone(),
      },
    )
    .unwrap();

  fun.local(idx, ident_type_to_val_type(types));
  fun.instruction(Instruction::LocalSet(idx));
}

pub fn compile_var_decl(compiler: &mut Compiler, fun: &mut Function, ident: IdentTyped, val: Expr) {
  let types = compile_expr(compiler, fun, val);
  if ident.type_ident != types {
    compiler.throw(CompilerErrorKind::IncompatibleTypes, 0)
  }

  let idx = compiler
    .scope
    .set_local_sym(
      &ident.ident,
      Symbol {
        global: false,
        mutable: true,
        types: types.clone(),
      },
    )
    .unwrap();

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
  if let Some(expr) = expr {
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
  let sym = compiler.scope.get_sym(&ident).unwrap().clone();
  let expr = compile_expr(compiler, fun, rhs);
  if !sym.1.mutable {
    compiler.throw(CompilerErrorKind::ImmutableAssign, 0)
  }

  if sym.1.types != expr {
    compiler.throw(CompilerErrorKind::IncompatibleTypes, 0)
  }

  if sym.1.global {
    fun.instruction(Instruction::GlobalSet(sym.0));
  } else {
    fun.instruction(Instruction::LocalSet(sym.0));
  }
}

pub fn compile_expr_stmt(compiler: &mut Compiler, fun: &mut Function, expr: Expr) {
  compile_expr(compiler, fun, expr);
}
