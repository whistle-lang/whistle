use crate::check_bool_expr;
use crate::check_expr;
use crate::errors::CompilerErrorKind;
use crate::Checker;
use crate::IndexedSymbol;
use crate::Symbol;
use crate::TypeVal;

use whistle_ast::Expr;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Stmt;

pub fn check_stmt(checker: &mut Checker, stmt: Stmt) {
  match stmt {
    Stmt::While { cond, do_stmt } => check_while(checker, cond, do_stmt),
    Stmt::ValDecl { ident_typed, val } => check_val_decl(checker, ident_typed, val),
    Stmt::VarDecl { ident_typed, val } => check_var_decl(checker, ident_typed, val),
    Stmt::Assign { op, rhs, ident } => check_assign(checker, op, rhs, ident),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
    } => check_if(checker, cond, then_stmt, else_stmt),
    Stmt::Expr(args) => check_expr_stmt(checker, args),
    Stmt::Block(args) => check_block(checker, args),
    Stmt::Return(expr) => check_return(checker, expr),
    _ => checker.throw(CompilerErrorKind::Unimplemented, 0),
  }
}

pub fn check_stmts(checker: &mut Checker, stmts: Vec<Stmt>) {
  checker.scope.enter_scope();
  for stmt in stmts {
    check_stmt(checker, stmt);
  }
  checker.scope.exit_scope();
}

pub fn check_while(checker: &mut Checker, cond: Expr, do_stmt: Vec<Stmt>) {
  check_bool_expr(checker, cond);
  check_stmts(checker, do_stmt);
}

pub fn check_if(
  checker: &mut Checker,
  cond: Expr,
  then_stmt: Vec<Stmt>,
  else_stmt: Option<Vec<Stmt>>,
) {
  check_bool_expr(checker, cond);
  check_stmts(checker, then_stmt);

  if let Some(stmt) = else_stmt {
    check_stmts(checker, stmt);
  }
}

pub fn check_val_decl(checker: &mut Checker, ident: IdentTyped, expr: Expr) {
  let ident_type = checker.new_type_val();
  checker.constraints.push((ident_type, TypeVal::Ident(ident.type_ident)));

  if let Err(err) = checker.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: false,
      types: ident.type_ident.clone(),
      type_val: ident_type
    },
  ) {
    checker.throw(err, 0);
  };

  let expr_type = check_expr(checker, expr);
  checker.constraints.push((expr_type, ident_type));
}

pub fn check_var_decl(checker: &mut Checker, ident: IdentTyped, expr: Expr) {
  let ident_type = checker.new_type_val();
  checker.constraints.push((ident_type, TypeVal::Ident(ident.type_ident)));

  if let Err(err) = checker.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: true,
      types: ident.type_ident.clone(),
      type_val: ident_type
    },
  ) {
    checker.throw(err, 0);
  };
  
  let expr_type = check_expr(checker, expr);
  checker.constraints.push((expr_type, ident_type));
}

pub fn check_block(checker: &mut Checker, stmts: Vec<Stmt>) {
  checker.scope.enter_scope();
  for stmt in stmts {
    check_stmt(checker, stmt)
  }
  checker.scope.exit_scope();
}

pub fn check_return(checker: &mut Checker, expr: Option<Expr>) {
  if let Some(expr) = expr {
    check_expr(checker, expr);
  }
}

pub fn check_assign(
  checker: &mut Checker,
  _op: Operator,
  expr: Expr,
  ident: String,
) {
  let sym = match checker.scope.get_sym(&ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      checker.throw(err, 0);
      IndexedSymbol(0, Symbol::default())
    }
  };
  if !sym.1.mutable {
    checker.throw(CompilerErrorKind::ImmutableAssign, 0)
  }
  
  let expr_type = check_expr(checker, expr);
  checker.constraints.push((expr_type, sym.1.type_val));
}

pub fn check_expr_stmt(checker: &mut Checker, expr: Expr) {
  check_expr(checker, expr);
}
