use crate::check_expr;
use crate::check_stmts;
use crate::Checker;
use crate::Symbol;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;
use whistle_ast::Type;
use whistle_common::Range;

pub fn check_program(checker: &mut Checker, program: &mut ProgramStmt) {
  match program {
    ProgramStmt::FunctionDecl {
      export,
      ident,
      params,
      ret_type,
      stmt,
      range,
      ..
    } => check_fn(checker, export, ident, params, ret_type, stmt, range),
    ProgramStmt::ValDecl {
      ident_typed,
      val,
      range,
    } => check_val(checker, ident_typed, val, range),
    ProgramStmt::VarDecl {
      ident_typed,
      val,
      range,
    } => check_var(checker, ident_typed, val, range),
    _ => unimplemented!(),
  }
}

pub fn check_fn(
  checker: &mut Checker,
  _export: &mut bool,
  ident: &mut str,
  params: &mut Vec<IdentTyped>,
  ret_type: &mut IdentType,
  stmts: &mut Vec<Stmt>,
  range: &mut Range,
) {
  if let Err(err) = checker.scope.set_function_sym(
    ident,
    Symbol {
      global: true,
      mutable: false,
      types: Type::Function {
        params: IdentTyped::vec_to_type(params),
        ret_type: Box::new(ret_type.to_type()),
      },
    },
  ) {
    checker.throw(err, range.clone());
  }

  checker.scope.enter_scope();

  for param in params {
    if let Err(err) = checker.scope.set_local_sym(
      &param.ident,
      Symbol {
        global: false,
        mutable: true,
        types: param.type_ident.to_type(),
      },
    ) {
      checker.throw(err, param.range.unwrap().clone());
    }
  }

  let ret = check_stmts(checker, stmts);
  let range = stmts[stmts.len() - 1].range();
  checker.constraint(ret, ret_type.to_type(), Some(range));

  checker.scope.exit_scope();
}

pub fn check_val(
  checker: &mut Checker,
  ident_typed: &mut IdentTyped,
  expr: &mut Expr,
  range: &mut Range,
) {
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
    checker.throw(err, range.clone());
  };

  let expr_type = check_expr(checker, expr);
  checker.constraint(ident_type.clone(), expr_type, Some(expr.range()));
  if Type::Default != ident_typed.type_ident.to_type() {
    checker.constraint(ident_type, ident_typed.type_ident.to_type(), None);
  }
}

pub fn check_var(
  checker: &mut Checker,
  ident_typed: &mut IdentTyped,
  expr: &mut Expr,
  range: &mut Range,
) {
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
    checker.throw(err, range.clone());
  };

  let expr_type = check_expr(checker, expr);
  checker.constraint(ident_type.clone(), expr_type, Some(expr.range()));
  if Type::Default != ident_typed.type_ident.to_type() {
    checker.constraint(ident_type, ident_typed.type_ident.to_type(), None);
  }
}
