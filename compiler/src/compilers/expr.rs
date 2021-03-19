use crate::operator_to_instruction;
use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Function;
use crate::IndexedSymbol;

use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentType;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Unary;

use whistle_common::Primitive;

pub fn compile_expr(compiler: &mut Compiler, fun: &mut Function, expr: Expr) -> IdentType {
  match expr {
    Expr::Binary { op, rhs, lhs } => compile_bin_expr(compiler, fun, op, *rhs, *lhs),
    Expr::Unary(expr) => compile_unary(compiler, fun, expr),
    _ => panic!("exp"),
  }
}

pub fn compile_bin_expr(
  compiler: &mut Compiler,
  fun: &mut Function,
  op: Operator,
  rhs: Expr,
  lhs: Expr,
) -> IdentType {
  if op == Operator::Assign {
    if let Expr::Unary(Unary::Primary(Primary::IdentVal { ident, .. })) = lhs {
      let type1 = compile_expr(compiler, fun, rhs);
      let sym = compiler.scope.get_sym(&ident).unwrap();

      if !sym.1.mutable {
        panic!("Immutable assign!");
      }

      if sym.1.global {
        fun.instruction(Instruction::GlobalSet(sym.0));
      } else {
        fun.instruction(Instruction::LocalSet(sym.0));
      }

      type1
    } else {
      panic!("Could not compile assignment");
    }
  } else {
    let type1 = compile_expr(compiler, fun, lhs);
    let _type2 = compile_expr(compiler, fun, rhs);

    fun.instruction(operator_to_instruction(&op, &type1));

    type1
  }
}

pub fn compile_unary(compiler: &mut Compiler, fun: &mut Function, expr: Unary) -> IdentType {
  match expr {
    Unary::Primary(expr) => compile_primary(compiler, fun, expr),
    Unary::UnaryOp { op: _, expr } => compile_unary(compiler, fun, *expr),
  }
}

pub fn compile_primary(compiler: &mut Compiler, fun: &mut Function, expr: Primary) -> IdentType {
  match expr {
    Primary::Literal(lit) => compile_literal(compiler, fun, lit),
    Primary::IdentVal { ident, prim } => compile_ident(compiler, fun, ident, prim),
    Primary::Grouping(expr) => compile_expr(compiler, fun, *expr),
  }
}

pub fn compile_literal(_compiler: &mut Compiler, fun: &mut Function, lit: Literal) -> IdentType {
  match lit {
    Literal::Bool(val) => {
      fun.instruction(Instruction::I32Const(if val { 1 } else { 0 }));
      IdentType::Primitive(Primitive::Bool)
    }
    Literal::Char(val) => {
      fun.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::Char)
    }
    Literal::Int(val) => {
      fun.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::I32)
    }
    Literal::Float(val) => {
      fun.instruction(Instruction::F64Const(val));
      IdentType::Primitive(Primitive::F32)
    }
    Literal::Str(_) => IdentType::Primitive(Primitive::Str),
    Literal::None => IdentType::Primitive(Primitive::None),
  }
}

#[allow(mutable_borrow_reservation_conflict)]
pub fn compile_ident(
  compiler: &mut Compiler,
  fun: &mut Function,
  ident: String,
  prim: Vec<IdentVal>,
) -> IdentType {
  if let Ok(sym) = compiler.scope.get_sym(&ident) {
    compile_ident_val(compiler, fun, sym.clone(), prim, 0)
  } else {
    compiler.throw(CompilerErrorKind::VarUndefined, 0);
    IdentType::Error
  }
}

pub fn compile_ident_val(
  compiler: &mut Compiler,
  fun: &mut Function,
  sym: IndexedSymbol,
  prim: Vec<IdentVal>,
  index: usize,
) -> IdentType {
  if prim.is_empty() {
    if sym.1.global {
      fun.instruction(Instruction::GlobalGet(sym.0));
    } else {
      fun.instruction(Instruction::LocalGet(sym.0));
    }

    sym.1.types
  } else {
    let types = match &prim[index] {
      IdentVal::Arguments(args) => compile_arguments(compiler, fun, sym.clone(), args.clone()),
      IdentVal::Selector(ident) => compile_selector(compiler, fun, sym.clone(), ident.clone()),
      _ => panic!("future"),
    };
    if prim.len() > index + 1 {
      compile_ident_val(compiler, fun, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn compile_arguments(
  compiler: &mut Compiler,
  fun: &mut Function,
  sym: IndexedSymbol,
  args: Vec<Expr>,
) -> IdentType {
  if let IdentType::Function { params, ret_type } = sym.1.types {
    for (i, param) in params.into_iter().enumerate() {
      if args.len() > i {
        if param.type_ident != compile_expr(compiler, fun, args[i].clone()) {
          compiler.throw(CompilerErrorKind::IncompatibleTypes, 0);
        }
      } else {
        compiler.throw(CompilerErrorKind::MissingParameters, 0);
      }
    }
    fun.instruction(Instruction::Call(sym.0));

    *ret_type
  } else {
    compiler.throw(CompilerErrorKind::NoCallSignatures, 0);
    IdentType::Error
  }
}

pub fn compile_selector(
  compiler: &mut Compiler,
  _fun: &mut Function,
  sym: IndexedSymbol,
  ident: String,
) -> IdentType {
  if let IdentType::Struct(props) = sym.1.types {
    for prop in props {
      if prop.ident == ident {
        return prop.type_ident;
      }
    }
  }
  compiler.throw(CompilerErrorKind::NoProperty, 0);
  IdentType::Error
}
