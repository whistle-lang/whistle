use crate::binary_to_type_val;
use crate::unary_to_type_val;
use crate::Checker;
use crate::CompilerErrorKind;
use crate::IndexedSymbol;
use crate::Symbol;

use whistle_ast::Expr;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Primitive;
use whistle_ast::Type;
use whistle_ast::Unary;
use whistle_common::Span;

pub fn check_expr(checker: &mut Checker, expr: &mut Expr) -> Type {
  match expr {
    Expr::Binary { op, rhs, lhs, .. } => check_bin_expr(checker, op, rhs, lhs),
    Expr::Unary { unary, .. } => check_unary(checker, unary),
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
      ..
    } => check_cond(checker, cond, then_expr, else_expr),
  }
}

pub fn check_bool_expr(checker: &mut Checker, expr: &mut Expr) -> Type {
  let ret_type = check_expr(checker, expr);
  checker.constraint(
    ret_type.clone(),
    Type::Primitive(Primitive::Bool),
    Some(expr.span()),
  );
  ret_type
}

pub fn check_bin_expr(
  checker: &mut Checker,
  op: &mut Operator,
  rhs: &mut Expr,
  lhs: &mut Expr,
) -> Type {
  if op == &Operator::Assign {
    if let Expr::Unary {
      unary: Unary::Primary {
        prim: Primary::IdentVal { ident, .. },
        ..
      },
      span,
    } = lhs
    {
      let ret_type = checker.new_type_val();
      let type1 = check_expr(checker, rhs);
      let sym = match checker.scope.get_sym(ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          checker.throw(err, span.clone());
          IndexedSymbol(0, Symbol::default())
        }
      };

      checker.constraint(sym.1.types.clone(), type1, Some(rhs.span()));
      checker.constraint(ret_type.clone(), sym.1.types, None);

      if !sym.1.mutable {
        checker.throw(CompilerErrorKind::ImmutableAssign, span.clone());
      }

      return ret_type;
    }
    checker.throw(CompilerErrorKind::Unassignable, rhs.span());
    Type::Error
  } else {
    let ret_type = checker.new_type_val();
    let type1 = check_expr(checker, lhs);
    let type2 = check_expr(checker, rhs);

    let expected = binary_to_type_val(op);
    checker.constraint(type2, type1.clone(), Some(rhs.span()));
    checker.constraint(ret_type.clone(), type1.clone(), None);
    checker.constraint(type1, expected, Some(lhs.span()));

    ret_type
  }
}

pub fn check_unary(checker: &mut Checker, expr: &mut Unary) -> Type {
  match expr {
    Unary::Primary { prim, .. } => check_primary(checker, prim),
    Unary::UnaryOp { op, expr, .. } => {
      let ret_type = checker.new_type_val();
      let val_type = check_unary(checker, expr);

      let expected = unary_to_type_val(op);
      let span = match **expr {
        Unary::Primary { span, .. } => span,
        Unary::UnaryOp { span, .. } => span,
      };
      checker.constraint(ret_type.clone(), val_type.clone(), None);
      checker.constraint(val_type, expected, Some(span));

      ret_type
    }
  }
}

pub fn check_primary(checker: &mut Checker, expr: &mut Primary) -> Type {
  match expr {
    Primary::Literal { lit, meta_id, .. } => check_literal(checker, lit, meta_id),
    Primary::IdentVal { ident, prim, span } => check_ident(checker, ident, prim, span),
    Primary::Grouping { group, .. } => check_expr(checker, group),
    Primary::Array { exprs, meta_id, .. } => check_array(checker, exprs, meta_id),
  }
}

pub fn check_literal(checker: &mut Checker, lit: &mut Literal, id: &mut usize) -> Type {
  match lit {
    Literal::Bool(_) => Type::Primitive(Primitive::Bool),
    Literal::Char(_) => Type::Primitive(Primitive::Char),
    Literal::Int(_) => {
      *id = checker.substitutions.len();
      let lit_type = checker.new_type_val();
      checker.constraint(lit_type.clone(), Type::Primitive(Primitive::Int), None);
      lit_type
    }
    Literal::Float(_) => {
      *id = checker.substitutions.len();
      let lit_type = checker.new_type_val();
      checker.constraint(lit_type.clone(), Type::Primitive(Primitive::Float), None);
      lit_type
    }
    Literal::Str(_) => Type::Primitive(Primitive::Str),
    Literal::None => Type::Primitive(Primitive::None),
    _ => unimplemented!(),
  }
}

pub fn check_ident(
  checker: &mut Checker,
  ident: &mut str,
  prim: &mut Vec<IdentVal>,
  span: &mut Span,
) -> Type {
  let sym = match checker.scope.get_sym(ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      checker.throw(err, span.clone());
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
) -> Type {
  if prim.is_empty() {
    sym.1.types.clone()
  } else {
    let types = match &mut prim[index] {
      IdentVal::Arguments { args, span } => check_arguments(checker, sym, args, span),
      IdentVal::Selector { ident, span } => check_selector(checker, sym, ident, span),
      _ => unimplemented!(),
    };
    if prim.len() > index + 1 {
      check_ident_val(checker, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn check_array(checker: &mut Checker, exprs: &mut Vec<Expr>, id: &mut usize) -> Type {
  *id = checker.substitutions.len();
  let ret_type = checker.new_type_val();
  let type1;
  if !exprs.is_empty() {
    type1 = check_expr(checker, &mut exprs[0]);
    for (_, expr) in exprs.iter_mut().skip(1).enumerate() {
      let type2 = check_expr(checker, expr);
      checker.constraint(type2, type1.clone(), Some(expr.span()));
    }
  } else {
    type1 = checker.new_type_val();
  }
  checker.constraint(ret_type.clone(), Type::Array(Box::new(type1)), None);
  ret_type
}

pub fn check_arguments(
  checker: &mut Checker,
  sym: &IndexedSymbol,
  args: &mut Vec<Expr>,
  span: &mut Span,
) -> Type {
  if let Type::Function { params, ret_type } = sym.1.types.clone() {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        let expr_type = check_expr(checker, &mut args[i]);
        checker.constraint(expr_type, param.type_ident, Some(args[i].span()));
      } else {
        checker.throw(CompilerErrorKind::MissingParameters, span.clone());
      }
    }
    *ret_type
  } else {
    checker.throw(CompilerErrorKind::MissingCallSignature, span.clone());
    Type::Error
  }
}

pub fn check_selector(
  checker: &mut Checker,
  sym: &IndexedSymbol,
  ident: &mut String,
  span: &mut Span,
) -> Type {
  if let Type::Struct(props) = sym.1.types.clone() {
    for prop in props {
      if prop.ident == *ident {
        return prop.type_ident;
      }
    }

    checker.throw(CompilerErrorKind::MissingProperty, span.clone());
  }
  checker.throw(CompilerErrorKind::NoProperties, span.clone());
  Type::Error
}

pub fn check_cond(
  checker: &mut Checker,
  cond: &mut Expr,
  then_expr: &mut Expr,
  else_expr: &mut Expr,
) -> Type {
  let ret_type = checker.new_type_val();
  let type1 = check_expr(checker, then_expr);
  let type2 = check_expr(checker, else_expr);
  check_bool_expr(checker, cond);

  checker.constraint(type1, type2.clone(), Some(else_expr.span()));
  checker.constraint(type2, ret_type.clone(), None);

  ret_type
}
