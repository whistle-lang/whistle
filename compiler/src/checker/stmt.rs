use crate::check_expr;
use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Var;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Primitive;
use whistle_ast::Stmt;
use whistle_ast::Tip;

pub fn check_stmt(compiler: &mut Compiler, stmt: Stmt) {
  match stmt {
    // Stmt::Tip(args) => check_tip(args),
    Stmt::While { cond, do_stmt } => check_while(compiler, cond, do_stmt),
    Stmt::ValDecl { ident_typed, val } => check_var_decl(compiler, ident_typed, val),
    Stmt::VarDecl { ident_typed, val } => check_var_decl(compiler, ident_typed, val),
    Stmt::Assign { op, rhs, ident } => check_assign(compiler, op, rhs, ident),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
    } => check_if(compiler, cond, then_stmt, else_stmt),
    Stmt::Expr(args) => check_expr_stmt(compiler, args),
    Stmt::Block(args) => check_block(compiler, args),
    // Stmt::Return
    _ => panic!("stmt"),
  }
}

pub fn check_bool(compiler: &mut Compiler, expr: Expr) {
  if IdentType::Primitive(Primitive::Bool) != check_expr(compiler, expr) {
    compiler.throw(CompilerErrorKind::ExpectedBooleanExpr, 0)
  }
}

pub fn check_stmts(compiler: &mut Compiler, stmts: Vec<Stmt>) {
  compiler.enter_scope();
  for stmt in stmts {
    check_stmt(compiler, stmt);
  }
  compiler.exit_scope();
}

pub fn check_tip(_tip: Tip) {}

pub fn check_while(compiler: &mut Compiler, cond: Expr, do_stmt: Vec<Stmt>) {
  check_bool(compiler, cond);
  check_stmts(compiler, do_stmt);
}

pub fn check_if(
  compiler: &mut Compiler,
  cond: Expr,
  then_stmt: Vec<Stmt>,
  else_stmt: Option<Vec<Stmt>>,
) {
  check_bool(compiler, cond);
  check_stmts(compiler, then_stmt);
  if let Some(stmt) = else_stmt {
    check_stmts(compiler, stmt);
  }
}

pub fn check_var_decl(compiler: &mut Compiler, ident: IdentTyped, val: Expr) {
  let types = check_expr(compiler, val);
  let var = Var {
    mutable: false,
    types: types.clone(),
  };
  if ident.type_ident != types {
    compiler.throw(CompilerErrorKind::IncompatibleTypes, 0)
  }
  compiler.set_var(ident.ident, var);
}

pub fn check_block(compiler: &mut Compiler, stmts: Vec<Stmt>) {
  compiler.enter_scope();
  for stmt in stmts {
    check_stmt(compiler, stmt)
  }
  compiler.exit_scope();
}

pub fn check_assign(compiler: &mut Compiler, _op: Operator, rhs: Expr, ident: String) {
  if let Some(types) = compiler.get_var(ident) {
    let expr = check_expr(compiler, rhs);
    if types.types != expr {
      compiler.throw(CompilerErrorKind::IncompatibleTypes, 0)
    }
  }
}

pub fn check_expr_stmt(compiler: &mut Compiler, expr: Expr) {
  check_expr(compiler, expr);
}
