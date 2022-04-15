use crate::binary_to_type_val;
use crate::unary_to_type_val;
use crate::Checker;
use crate::CompilerErrorKind;
use crate::IndexedSymbol;
use crate::Symbol;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Primitive;
use whistle_ast::Unary;

pub fn check_expr(checker: &mut Checker, expr: &mut Expr) -> IdentType {
  match expr {
    Expr::Binary { op, rhs, lhs } => check_bin_expr(checker, op, rhs, lhs),
    Expr::Unary(expr) => check_unary(checker, expr),
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
    } => check_cond(checker, cond, then_expr, else_expr),
  }
}

pub fn check_bool_expr(checker: &mut Checker, expr: &mut Expr) -> IdentType {
  let ret_type = check_expr(checker, expr);
  checker
    .constraints
    .push((ret_type.clone(), IdentType::Primitive(Primitive::Bool)));
  ret_type
}

pub fn check_bin_expr(
  checker: &mut Checker,
  op: &mut Operator,
  rhs: &mut Expr,
  lhs: &mut Expr,
) -> IdentType {
  if op == &Operator::Assign {
    if let Expr::Unary(Unary::Primary(Primary::IdentVal { ident, .. })) = lhs {
      let ret_type = checker.new_type_val();
      let type1 = check_expr(checker, rhs);
      let sym = match checker.scope.get_sym(ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          checker.throw(err, 0);
          IndexedSymbol(0, Symbol::default())
        }
      };

      checker.constraints.push((sym.1.types.clone(), type1));
      checker.constraints.push((ret_type.clone(), sym.1.types));

      if !sym.1.mutable {
        checker.throw(CompilerErrorKind::ImmutableAssign, 0);
      }

      ret_type
    } else {
      checker.throw(CompilerErrorKind::Unassignable, 0);
      IdentType::Error
    }
  } else {
    let ret_type = checker.new_type_val();
    let type1 = check_expr(checker, lhs);
    let type2 = check_expr(checker, rhs);

    let expected = binary_to_type_val(op);
    checker.constraints.push((type2, type1.clone()));
    checker.constraints.push((ret_type.clone(), type1.clone()));
    checker.constraints.push((type1, expected));

    ret_type
  }
}

pub fn check_unary(checker: &mut Checker, expr: &mut Unary) -> IdentType {
  match expr {
    Unary::Primary(expr) => check_primary(checker, expr),
    Unary::UnaryOp { op, expr } => {
      let ret_type = checker.new_type_val();
      let val_type = check_unary(checker, expr);

      let expected = unary_to_type_val(op);
      checker
        .constraints
        .push((ret_type.clone(), val_type.clone()));
      checker.constraints.push((val_type, expected));

      ret_type
    }
  }
}

pub fn check_primary(checker: &mut Checker, expr: &mut Primary) -> IdentType {
  match expr {
    Primary::Literal(lit) => check_literal(checker, lit),
    Primary::IdentVal { ident, prim } => check_ident(checker, ident, prim),
    Primary::Grouping(expr) => check_expr(checker, expr),
    Primary::Array { exprs, type_ident } => check_array(checker, exprs, type_ident),
  }
}

pub fn check_literal(checker: &mut Checker, lit: &mut Literal) -> IdentType {
  match lit {
    Literal::Bool(_) => IdentType::Primitive(Primitive::Bool),
    Literal::Char(_) => IdentType::Primitive(Primitive::Char),
    Literal::Int(_) => {
      checker
        .literals
        .push((checker.substitutions.len(), &mut *lit));
      let lit_type = checker.new_type_val();
      checker
        .constraints
        .push((lit_type.clone(), IdentType::Primitive(Primitive::Int)));
      lit_type
    }
    Literal::Float(_) => {
      checker
        .literals
        .push((checker.substitutions.len(), &mut *lit));
      let lit_type = checker.new_type_val();
      checker
        .constraints
        .push((lit_type.clone(), IdentType::Primitive(Primitive::Float)));
      lit_type
    }
    Literal::Str(_) => IdentType::Primitive(Primitive::Str),
    Literal::None => IdentType::Primitive(Primitive::None),
    _ => unimplemented!(),
  }
}

pub fn check_ident(checker: &mut Checker, ident: &mut str, prim: &mut Vec<IdentVal>) -> IdentType {
  let sym = match checker.scope.get_sym(ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      checker.throw(err, 0);
      IndexedSymbol(0, Symbol::default())
    }
  };
  check_ident_val(checker, &sym, prim, 0)
}

pub fn check_ident_val(
  checker: &mut Checker,
  sym: &IndexedSymbol,
  prim: &mut Vec<IdentVal>,
  index: usize,
) -> IdentType {
  if prim.is_empty() {
    sym.1.types.clone()
  } else {
    let types = match &mut prim[index] {
      IdentVal::Arguments(args) => check_arguments(checker, sym, args),
      IdentVal::Selector(ident) => check_selector(checker, sym, ident),
      _ => unimplemented!(),
    };
    if prim.len() > index + 1 {
      check_ident_val(checker, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn check_array(
  checker: &mut Checker,
  exprs: &mut Vec<Expr>,
  type_ident: &mut IdentType,
) -> IdentType {
  checker
    .idents
    .push((checker.substitutions.len(), &mut *type_ident));
  let ret_type = checker.new_type_val();
  let type1;
  if !exprs.is_empty() {
    type1 = check_expr(checker, &mut exprs[0]);
    for (_, expr) in exprs.iter_mut().skip(1).enumerate() {
      let type2 = check_expr(checker, expr);
      checker.constraints.push((type2, type1.clone()));
    }
  } else {
    type1 = checker.new_type_val();
  }
  checker
    .constraints
    .push((ret_type.clone(), IdentType::Array(Box::new(type1))));
  ret_type
}

pub fn check_arguments(
  checker: &mut Checker,
  sym: &IndexedSymbol,
  args: &mut Vec<Expr>,
) -> IdentType {
  if let IdentType::Function { params, ret_type } = sym.1.types.clone() {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        let expr_type = check_expr(checker, &mut args[i]);
        checker.constraints.push((expr_type, param.type_ident));
      } else {
        checker.throw(CompilerErrorKind::MissingParameters, 0);
      }
    }
    *ret_type
  } else {
    checker.throw(CompilerErrorKind::MissingCallSignature, 0);
    IdentType::Error
  }
}

pub fn check_selector(checker: &mut Checker, sym: &IndexedSymbol, ident: &mut String) -> IdentType {
  if let IdentType::Struct(props) = sym.1.types.clone() {
    for prop in props {
      if prop.ident == *ident {
        return prop.type_ident;
      }
    }

    checker.throw(CompilerErrorKind::MissingProperty, 0);
  }
  checker.throw(CompilerErrorKind::NoProperties, 0);
  IdentType::Error
}

pub fn check_cond(
  checker: &mut Checker,
  cond: &mut Expr,
  then_expr: &mut Expr,
  else_expr: &mut Expr,
) -> IdentType {
  let ret_type = checker.new_type_val();
  let type1 = check_expr(checker, then_expr);
  let type2 = check_expr(checker, else_expr);
  check_bool_expr(checker, cond);

  checker.constraints.push((type1, type2.clone()));
  checker.constraints.push((type2, ret_type.clone()));

  ret_type
}
