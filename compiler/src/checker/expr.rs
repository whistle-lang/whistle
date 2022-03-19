use crate::binary_to_type_val;
use crate::unary_to_type_val;
use crate::Checker;
use crate::CompilerErrorKind;
use crate::IndexedSymbol;
use crate::Symbol;
use crate::TypeVal;

use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Unary;

use whistle_common::Primitive;

pub fn check_expr(checker: &mut Checker, expr: Expr) -> TypeVal {
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

pub fn check_bool_expr(checker: &mut Checker, expr: Expr) -> TypeVal {
  let ret_type = check_expr(checker, expr);
  checker.constraints.push((ret_type, TypeVal::Ident(IdentType::Primitive(Primitive::Bool))));
  ret_type
}

pub fn check_bin_expr(checker: &mut Checker, op: Operator, rhs: Expr, lhs: Expr) -> TypeVal {
  if op == Operator::Assign {
    if let Expr::Unary(Unary::Primary(Primary::IdentVal { ident, .. })) = lhs {
      let ret_type = checker.new_type_val();
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
      checker.constraints.push((sym.1.type_val, rhs));
      checker.constraints.push((ret_type, ret));

      if !sym.1.mutable {
        checker.throw(CompilerErrorKind::ImmutableAssign, 0);
      }

      ret_type
    } else {
      checker.throw(CompilerErrorKind::Unassignable, 0);
      TypeVal::Ident(IdentType::Error)
    }
  } else {
    let ret_type = checker.new_type_val();
    let type1 = check_expr(checker, lhs);
    let type2 = check_expr(checker, rhs);

    let (rhs, lhs, ret) = binary_to_type_val(&op);
    checker.constraints.push((type1, lhs));
    checker.constraints.push((type2, rhs));
    checker.constraints.push((ret_type, ret));

    ret_type
  }
}

pub fn check_unary(checker: &mut Checker, expr: Unary) -> TypeVal {
  match expr {
    Unary::Primary(expr) => check_primary(checker, expr),
    Unary::UnaryOp { op, expr } => {
      let ret_type = checker.new_type_val();
      let val_type = check_unary(checker, *expr);

      let (ret, val) = unary_to_type_val(&op);
      checker.constraints.push((val_type, val));
      checker.constraints.push((ret_type, ret));

      val_type
    },
  }
}

pub fn check_primary(checker: &mut Checker, expr: Primary) -> TypeVal {
  match expr {
    Primary::Literal(lit) => check_literal(checker, lit),
    Primary::IdentVal { ident, prim } => check_ident(checker, ident, prim),
    Primary::Grouping(expr) => check_expr(checker, *expr),
    // Primary::Array(arr) => check_array(checker, arr),
    _ => unimplemented!()
  }
}

pub fn check_literal(checker: &mut Checker, lit: Literal) -> TypeVal {
  match lit {
    Literal::Bool(val) => TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
    Literal::Char(val) => TypeVal::Ident(IdentType::Primitive(Primitive::Char)),
    Literal::Int(val) => TypeVal::Ident(IdentType::Int),
    Literal::Float(val) => TypeVal::Ident(IdentType::Float),
    Literal::Str(string) => TypeVal::Ident(IdentType::Primitive(Primitive::Str)),
    Literal::None => TypeVal::Ident(IdentType::Primitive(Primitive::None)),
  }
}

pub fn check_ident(checker: &mut Checker, ident: String, prim: Vec<IdentVal>) -> TypeVal {
  match checker.scope.get_sym(&ident) {
    Ok(sym) => check_ident_val(checker, sym.clone(), prim, 0),
    Err(err) => {
      checker.throw(err, 0);
      TypeVal::Ident(IdentType::Error)
    }
  }
}

pub fn check_ident_val(
  checker: &mut Checker,
  sym: IndexedSymbol,
  prim: Vec<IdentVal>,
  index: usize,
) -> TypeVal {
  if prim.is_empty() {
    sym.1.type_val
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

pub fn check_array(checker: &mut Checker, exprs: Vec<Expr>) {
  if let IdentType::Array(inner_type) = checker.scope.expr_type.clone() {
    checker.scope.expr_type = *inner_type
  } else {
    checker.throw(CompilerErrorKind::TypeMismatch, 0);
  }
  let idx = checker.memory.stack;
  let mut ident_type = checker.scope.expr_type.clone();
  for (_, expr) in exprs.into_iter().enumerate() {
    let expr_type = check_expr(checker, expr);
    if ident_type == expr_type {
      ident_type = expr_type.clone();
      let memarg = checker.memory.index_stack();
      let instruction = match expr_type {
        IdentType::Primitive(prim) => match prim {
          Primitive::I32 => Instruction::I32Store(memarg),
          Primitive::F32 => Instruction::F32Store(memarg),
          Primitive::I64 => Instruction::I64Store(memarg),
          Primitive::F64 => Instruction::F64Store(memarg),
          _ => unimplemented!(),
        },
        _ => unimplemented!(),
      };
      fun.instruction(instruction);
    } else {
      checker.throw(CompilerErrorKind::TypeMismatch, 0);
    }
  }
  fun.instruction(Instruction::I64Const(idx as i64));
  IdentType::Array(Box::new(ident_type))
}

pub fn check_arguments(checker: &mut Checker, sym: IndexedSymbol, args: Vec<Expr>) {
  if let IdentType::Function { params, ret_type } = sym.1.types {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        checker.scope.expr_type = param.type_ident.clone();
        if param.type_ident != check_expr(checker, args[i].clone()) {
          checker.throw(CompilerErrorKind::TypeMismatch, 0);
        }
      } else {
        checker.throw(CompilerErrorKind::MissingParameters, 0);
      }
    }
    fun.instruction(Instruction::Call(sym.0));

    *ret_type
  } else {
    checker.throw(CompilerErrorKind::MissingCallSignature, 0);
    IdentType::Error
  }
}

pub fn check_selector(checker: &mut Checker, sym: IndexedSymbol, ident: String) {
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

pub fn check_cond(checker: &mut Checker, cond: Expr, then_expr: Expr, else_expr: Expr) -> TypeVal {
  let ret_type = checker.new_type_val();
  let type1 = check_expr(checker, then_expr);
  let type2 = check_expr(checker, else_expr);
  check_bool_expr(checker, cond);

  checker.constraints.push((type1, type2));
  checker.constraints.push((type2, ret_type));

  ret_type
}
