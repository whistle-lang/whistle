use crate::check_bool_expr;
use crate::check_expr;
use crate::errors::CompilerErrorKind;
use crate::Checker;
use crate::IndexedSymbol;
use crate::Symbol;

use whistle_ast::Expr;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Primitive;
use whistle_ast::Stmt;
use whistle_ast::Type;

pub fn check_stmt(checker: &mut Checker, stmt: &mut Stmt) -> Type {
  match stmt {
    Stmt::While { cond, do_stmt, .. } => check_while(checker, cond, do_stmt),
    Stmt::ValDecl {
      ident_typed, val, ..
    } => check_val_decl(checker, ident_typed, val),
    Stmt::VarDecl {
      ident_typed, val, ..
    } => check_var_decl(checker, ident_typed, val),
    Stmt::Assign { op, rhs, ident, .. } => check_assign(checker, op, rhs, ident),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
      ..
    } => check_if(checker, cond, then_stmt, else_stmt),
    Stmt::Expr { expr, .. } => check_expr_stmt(checker, expr),
    Stmt::Block { stmts, .. } => check_block(checker, stmts),
    Stmt::Return { ret_type, .. } => check_return(checker, ret_type),
    _ => {
      checker.throw(CompilerErrorKind::Unimplemented, 0);
      Type::Primitive(Primitive::None)
    }
  }
}

pub fn check_stmts(checker: &mut Checker, stmts: &mut Vec<Stmt>) -> Type {
  let mut ret_type = Type::Primitive(Primitive::None);
  checker.scope.enter_scope();
  for stmt in stmts {
    ret_type = check_stmt(checker, stmt);
  }
  checker.scope.exit_scope();
  ret_type
}

pub fn check_while(checker: &mut Checker, cond: &mut Expr, do_stmt: &mut Vec<Stmt>) -> Type {
  check_bool_expr(checker, cond);
  check_stmts(checker, do_stmt);
  Type::Primitive(Primitive::None)
}

pub fn check_if(
  checker: &mut Checker,
  cond: &mut Expr,
  then_stmt: &mut Vec<Stmt>,
  else_stmt: &mut Option<Vec<Stmt>>,
) -> Type {
  check_bool_expr(checker, cond);
  check_stmts(checker, then_stmt);

  if let Some(stmt) = else_stmt {
    check_stmts(checker, stmt);
  }
  Type::Primitive(Primitive::None)
}

pub fn check_val_decl(checker: &mut Checker, ident: &mut IdentTyped, expr: &mut Expr) -> Type {
  checker
    .idents
    .push((checker.substitutions.len(), &mut (*ident).type_ident));
  let ident_type = checker.new_type_val();
  if let Err(err) = checker.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: false,
      types: ident_type.clone(),
    },
  ) {
    checker.throw(err, 0);
  };

  let expr_type = check_expr(checker, expr);
  checker.constraints.push((ident_type.clone(), expr_type));
  if Type::Default != ident.type_ident.to_type() {
    checker
      .constraints
      .push((ident_type, ident.type_ident.to_type()));
  }
  Type::Primitive(Primitive::None)
}

pub fn check_var_decl(checker: &mut Checker, ident: &mut IdentTyped, expr: &mut Expr) -> Type {
  checker
    .idents
    .push((checker.substitutions.len(), &mut (*ident).type_ident));
  let ident_type = checker.new_type_val();
  if let Err(err) = checker.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: true,
      types: ident_type.clone(),
    },
  ) {
    checker.throw(err, 0);
  };
  let expr_type = check_expr(checker, expr);
  checker.constraints.push((ident_type.clone(), expr_type));
  if Type::Default != ident.type_ident.to_type() {
    checker
      .constraints
      .push((ident_type, ident.type_ident.to_type()));
  }
  Type::Primitive(Primitive::None)
}

pub fn check_block(checker: &mut Checker, stmts: &mut Vec<Stmt>) -> Type {
  checker.scope.enter_scope();
  for stmt in stmts {
    check_stmt(checker, stmt);
  }
  checker.scope.exit_scope();
  Type::Primitive(Primitive::None)
}

pub fn check_return(checker: &mut Checker, expr: &mut Option<Expr>) -> Type {
  if let Some(expr) = expr {
    return check_expr(checker, expr);
  }

  Type::Primitive(Primitive::None)
}

pub fn check_assign(
  checker: &mut Checker,
  _op: &mut Operator,
  expr: &mut Expr,
  ident: &mut str,
) -> Type {
  let sym = match checker.scope.get_sym(ident) {
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
  checker.constraints.push((expr_type, sym.1.types));
  Type::Primitive(Primitive::None)
}

pub fn check_expr_stmt(checker: &mut Checker, expr: &mut Expr) -> Type {
  check_expr(checker, expr)
}
