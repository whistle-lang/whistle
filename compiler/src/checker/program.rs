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

pub fn check_program(checker: &mut Checker, program: ProgramStmt) {
  match program {
    ProgramStmt::FunDecl {
      export,
      ident,
      params,
      ret_type,
      stmt,
    } => check_fun(checker, export, ident, params, ret_type, stmt),
    ProgramStmt::ValDecl { ident_typed, val } => check_val(checker, ident_typed, val),
    ProgramStmt::VarDecl { ident_typed, val } => check_var(checker, ident_typed, val),
    _ => checker.throw(CompilerErrorKind::Unimplemented, 0),
  }
}

pub fn check_fun(
  checker: &mut Checker,
  _export: bool,
  ident: String,
  params: Vec<IdentTyped>,
  ret_type: IdentType,
  stmts: Vec<Stmt>,
) {
  if let Err(err) = checker.scope.set_fun_sym(
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
        types: param.type_ident,
      },
    ) {
      checker.throw(err, 0);
    }
  }

  // TODO: function return type
  check_stmts(checker, stmts);

  checker.scope.exit_scope();
}

pub fn check_val(checker: &mut Checker, mut ident_typed: IdentTyped, expr: Expr) {
  let ident_type = match ident_typed.type_ident.clone() {
    IdentType::Default => {
      checker.idents.push((checker.substitutions.len(), &mut ident_typed));
      checker.new_type_val()
    },
    _ => ident_typed.type_ident.clone(),
  };
  checker
    .constraints
    .push((ident_type.clone(), ident_typed.type_ident));

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
  checker.constraints.push((expr_type, ident_type));
}

pub fn check_var(checker: &mut Checker, mut ident_typed: IdentTyped, expr: Expr) {
  let ident_type = match ident_typed.type_ident.clone() {
    IdentType::Default => {
      checker.idents.push((checker.substitutions.len(), &mut ident_typed));
      checker.new_type_val()
    },
    _ => ident_typed.type_ident.clone(),
  };
  checker
    .constraints
    .push((ident_type.clone(), ident_typed.type_ident));

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
  checker.constraints.push((expr_type, ident_type));
}
