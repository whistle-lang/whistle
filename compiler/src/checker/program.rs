use crate::check_expr;
use crate::check_stmts;
use crate::Checker;
use crate::CompilerErrorKind;
use crate::Symbol;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;

pub fn check_program(checker: &mut Checker, program: &mut ProgramStmt) {
  match program {
    ProgramStmt::FunctionDecl {
      export,
      ident,
      params,
      ret_type,
      stmt,
      ..
    } => check_fn(checker, export, ident, params, ret_type, stmt),
    ProgramStmt::ValDecl { ident_typed, val } => check_val(checker, ident_typed, val),
    ProgramStmt::VarDecl { ident_typed, val } => check_var(checker, ident_typed, val),
    _ => checker.throw(CompilerErrorKind::Unimplemented, 0),
  }
}

pub fn check_fn(
  checker: &mut Checker,
  _export: &mut bool,
  ident: &mut String,
  params: &mut Vec<IdentTyped>,
  ret_type: &mut IdentType,
  stmts: &mut Vec<Stmt>,
) {
  if let Err(err) = checker.scope.set_function_sym(
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
    checker.throw(err, 0);
  }

  checker.scope.enter_scope();

  for param in params {
    if let Err(err) = checker.scope.set_local_sym(
      &param.ident,
      Symbol {
        global: false,
        mutable: true,
        types: param.type_ident.clone(),
      },
    ) {
      checker.throw(err, 0);
    }
  }

  let ret = check_stmts(checker, stmts);
  checker.constraints.push((ret, ret_type.clone()));

  checker.scope.exit_scope();
}

pub fn check_val(checker: &mut Checker, ident_typed: &mut IdentTyped, expr: &mut Expr) {
  checker
    .idents
    .push((checker.substitutions.len(), &mut (*ident_typed).type_ident));
  let ident_type = checker.new_type_val();

  if let Err(err) = checker.scope.set_global_sym(
    &ident_typed.ident,
    Symbol {
      global: true,
      mutable: false,
      types: ident_type.clone(),
    },
  ) {
    checker.throw(err, 0);
  };

  let expr_type = check_expr(checker, expr);
  checker.constraints.push((ident_type.clone(), expr_type));
  if IdentType::Default != ident_typed.type_ident {
    checker
      .constraints
      .push((ident_type, ident_typed.type_ident.clone()));
  }
}

pub fn check_var(checker: &mut Checker, ident_typed: &mut IdentTyped, expr: &mut Expr) {
  checker
    .idents
    .push((checker.substitutions.len(), &mut (*ident_typed).type_ident));
  let ident_type = checker.new_type_val();

  if let Err(err) = checker.scope.set_global_sym(
    &ident_typed.ident,
    Symbol {
      global: true,
      mutable: true,
      types: ident_type.clone(),
    },
  ) {
    checker.throw(err, 0);
  };

  let expr_type = check_expr(checker, expr);
  checker.constraints.push((ident_type.clone(), expr_type));
  if IdentType::Default != ident_typed.type_ident {
    checker
      .constraints
      .push((ident_type, ident_typed.type_ident.clone()));
  }
}
