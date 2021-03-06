use crate::Compiler;
use crate::CompilerErrorKind;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Unary;
use whistle_common::Primitive;

pub fn check_expr(compiler: &mut Compiler, expr: Expr) -> IdentType {
  match expr {
    Expr::Binary { op, rhs, lhs } => check_bin_expr(compiler, op, *rhs, *lhs),
    Expr::Unary(expr) => check_unary(compiler, expr),
    _ => panic!("exp"),
  }
}

pub fn check_bin_expr(compiler: &mut Compiler, _op: Operator, rhs: Expr, lhs: Expr) -> IdentType {
  let type1 = check_expr(compiler, lhs);
  let type2 = check_expr(compiler, rhs);
  if type1 == type2 {
    return IdentType::Error;
  }
  // if !type1.is_number() {
  //   compiler.throw(CompilerErrorKind, 0);
  //   return IdentType::Error;
  // }
  type1
}

pub fn check_unary(compiler: &mut Compiler, expr: Unary) -> IdentType {
  match expr {
    Unary::Primary(expr) => check_primary(compiler, expr),
    Unary::UnaryOp { op: _, expr } => check_unary(compiler, *expr),
  }
}

pub fn check_primary(compiler: &mut Compiler, expr: Primary) -> IdentType {
  match expr {
    Primary::Literal(lit) => check_literal(lit),
    Primary::IdentVal { ident, prim } => check_ident(compiler, ident, prim),
    Primary::Grouping(expr) => check_expr(compiler, *expr),
  }
}

pub fn check_literal(lit: Literal) -> IdentType {
  match lit {
    Literal::Bool(_) => IdentType::Primitive(Primitive::Bool),
    Literal::Char(_) => IdentType::Primitive(Primitive::Char),
    Literal::Int(_) => IdentType::Primitive(Primitive::I32),
    Literal::Float(_) => IdentType::Primitive(Primitive::F32),
    Literal::Str(_) => IdentType::Primitive(Primitive::Str),
    Literal::None => IdentType::Primitive(Primitive::None),
  }
}

pub fn check_ident(compiler: &mut Compiler, ident: String, prim: Vec<IdentVal>) -> IdentType {
  if let Some(ident_type) = compiler.get_var(ident) {
    return check_ident_val(compiler, ident_type.types, prim, 0);
  }
  compiler.throw(CompilerErrorKind::VarUndefined, 0);
  IdentType::Error
}

pub fn check_ident_val(
  compiler: &mut Compiler,
  ident_type: IdentType,
  prim: Vec<IdentVal>,
  index: usize,
) -> IdentType {
  let types = match &prim[index] {
    IdentVal::Arguments(args) => check_arguments(compiler, ident_type, args.clone()),
    IdentVal::Selector(ident) => check_selector(compiler, ident_type, ident.clone()),
    _ => panic!("future"),
  };

  if prim.len() > index + 1 {
    check_ident_val(compiler, types, prim, index + 1)
  } else {
    types
  }
}

pub fn check_arguments(
  compiler: &mut Compiler,
  ident_type: IdentType,
  args: Vec<Expr>,
) -> IdentType {
  if let IdentType::Function { params, ret_type } = ident_type {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        if param.type_ident != check_expr(compiler, args[i].clone()) {
          compiler.throw(CompilerErrorKind::IncompatibleTypes, 0);
        }
      } else {
        compiler.throw(CompilerErrorKind::MissingParameters, 0);
      }
    }
    *ret_type
  } else {
    compiler.throw(CompilerErrorKind::NoCallSignatures, 0);
    IdentType::Error
  }
}

pub fn check_selector(compiler: &mut Compiler, ident_type: IdentType, ident: String) -> IdentType {
  if let IdentType::Struct(props) = ident_type {
    for prop in props {
      if prop.ident == ident {
        return prop.type_ident;
      }
    }
  }
  compiler.throw(CompilerErrorKind::NoProperty, 0);
  IdentType::Error
}
