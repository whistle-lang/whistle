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
use whistle_ast::Unary;
use whistle_ast::Primitive;

pub fn check_expr(checker: &mut Checker, expr: Expr) -> IdentType {
  match expr {
    Expr::Binary { op, rhs, lhs } => check_bin_expr(checker, op, *rhs, *lhs),
    Expr::Unary(expr) => check_unary(checker, expr),
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
    } => check_cond(checker, *cond, *then_expr, *else_expr),
  }
}

pub fn check_bool_expr(checker: &mut Checker, expr: Expr) -> IdentType {
  let ret_type = check_expr(checker, expr);
  checker
    .constraints
    .push((ret_type.clone(), IdentType::Primitive(Primitive::Bool)));
  ret_type
}

pub fn check_bin_expr(checker: &mut Checker, op: Operator, rhs: Expr, lhs: Expr) -> IdentType {
  if Operator::is_assign(&op) {
    if let Expr::Unary(Unary::Primary(Primary::IdentVal { ident, .. })) = lhs {
      let type1 = check_expr(checker, rhs);
      let sym = match checker.scope.get_sym(&ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          checker.throw(err, 0);
          IndexedSymbol(0, Symbol::default())
        }
      };

      let (rhs, lhs, ret) = binary_to_type_val(&op);
      checker.constraints.push((type1, lhs));
      checker.constraints.push((sym.1.types, rhs));

      if !sym.1.mutable {
        checker.throw(CompilerErrorKind::ImmutableAssign, 0);
      }

      ret
    } else {
      checker.throw(CompilerErrorKind::Unassignable, 0);
      IdentType::Error
    }
  } else {
    let type1 = check_expr(checker, lhs);
    let type2 = check_expr(checker, rhs);

    let (rhs, lhs, ret) = binary_to_type_val(&op);
    checker.constraints.push((type1, lhs));
    checker.constraints.push((type2, rhs));

    ret
  }
}

pub fn check_unary(checker: &mut Checker, expr: Unary) -> IdentType {
  match expr {
    Unary::Primary(expr) => check_primary(checker, expr),
    Unary::UnaryOp { op, expr } => {
      let val_type = check_unary(checker, *expr);

      let (ret, val) = unary_to_type_val(&op);
      checker.constraints.push((val_type, val));

      ret
    }
  }
}

pub fn check_primary(checker: &mut Checker, expr: Primary) -> IdentType {
  match expr {
    Primary::Literal(lit) => check_literal(checker, lit),
    Primary::IdentVal { ident, prim } => check_ident(checker, ident, prim),
    Primary::Grouping(expr) => check_expr(checker, *expr),
    Primary::Array(arr) => check_array(checker, arr),
  }
}

pub fn check_literal(checker: &mut Checker, mut lit: Literal) -> IdentType {
  match lit {
    Literal::Bool(_) => IdentType::Primitive(Primitive::Bool),
    Literal::Char(_) => IdentType::Primitive(Primitive::Char),
    Literal::Int(_) => {
      let lit_type = checker.new_type_val();
      checker.literals.push((checker.substitutions.len(), &mut lit));
      checker.constraints.push((lit_type.clone(), IdentType::Primitive(Primitive::Int)));
      lit_type
    },
    Literal::Float(_) => {
      let lit_type = checker.new_type_val();
      checker.literals.push((checker.substitutions.len(), &mut lit));
      checker.constraints.push((lit_type.clone(), IdentType::Primitive(Primitive::Int)));
      lit_type
    },
    Literal::Str(_) => IdentType::Primitive(Primitive::Str),
    Literal::None => IdentType::Primitive(Primitive::None),
    _ => unimplemented!()
  }
}

#[allow(mutable_borrow_reservation_conflict)]
pub fn check_ident(checker: &mut Checker, ident: String, prim: Vec<IdentVal>) -> IdentType {
  match checker.scope.get_sym(&ident) {
    Ok(sym) => check_ident_val(checker, sym.clone(), prim, 0),
    Err(err) => {
      checker.throw(err, 0);
      IdentType::Error
    }
  }
}

pub fn check_ident_val(
  checker: &mut Checker,
  sym: IndexedSymbol,
  prim: Vec<IdentVal>,
  index: usize,
) -> IdentType {
  if prim.is_empty() {
    sym.1.types
  } else {
    let types = match &prim[index] {
      IdentVal::Arguments(args) => check_arguments(checker, sym.clone(), args.clone()),
      IdentVal::Selector(ident) => check_selector(checker, sym.clone(), ident.clone()),
      _ => unimplemented!(),
    };
    if prim.len() > index + 1 {
      check_ident_val(checker, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn check_array(checker: &mut Checker, exprs: Vec<Expr>) -> IdentType {
  let ret_type = checker.new_type_val();
  let type1 = check_expr(checker, exprs[0].clone());
  for (_, expr) in exprs.into_iter().enumerate() {
    let type2 = check_expr(checker, expr);
    checker.constraints.push((type1.clone(), type2));
  }
  checker.constraints.push((ret_type.clone(), IdentType::Array(Box::new(type1))));
  ret_type
}

pub fn check_arguments(checker: &mut Checker, sym: IndexedSymbol, args: Vec<Expr>) -> IdentType {
  if let IdentType::Function { params, ret_type } = sym.1.types {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        let expr_type = check_expr(checker, args[i].clone());
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

pub fn check_selector(checker: &mut Checker, sym: IndexedSymbol, ident: String) -> IdentType {
  if let IdentType::Struct(props) = sym.1.types {
    for prop in props {
      if prop.ident == ident {
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
  cond: Expr,
  then_expr: Expr,
  else_expr: Expr,
) -> IdentType {
  let ret_type = checker.new_type_val();
  let type1 = check_expr(checker, then_expr);
  let type2 = check_expr(checker, else_expr);
  check_bool_expr(checker, cond);

  checker.constraints.push((type1, type2.clone()));
  checker.constraints.push((type2, ret_type.clone()));

  ret_type
}
