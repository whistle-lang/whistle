use crate::operator_to_ident_type;
use crate::operator_to_instruction;
use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Function;
use crate::IndexedSymbol;
use crate::Symbol;

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
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
    } => compile_cond(compiler, fun, *cond, *then_expr, *else_expr),
  }
}

pub fn compile_bool_expr(compiler: &mut Compiler, fun: &mut Function, expr: Expr) -> IdentType {
  let expr_type = compile_expr(compiler, fun, expr);
  if IdentType::Primitive(Primitive::Bool) != expr_type {
    compiler.throw(CompilerErrorKind::ExpectedBooleanExpr, 0)
  }
  expr_type
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
      let sym = match compiler.scope.get_sym(&ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          compiler.throw(err, 0);
          IndexedSymbol(0, Symbol::default())
        }
      };

      if !sym.1.mutable {
        compiler.throw(CompilerErrorKind::ImmutableAssign, 0);
      }

      if sym.1.global {
        fun.instruction(Instruction::GlobalSet(sym.0));
      } else {
        fun.instruction(Instruction::LocalSet(sym.0));
      }

      if sym.1.types != type1 {
        compiler.throw(CompilerErrorKind::TypeMismatch, 0);
      }

      type1
    } else {
      compiler.throw(CompilerErrorKind::Unassignable, 0);
      IdentType::Error
    }
  } else {
    let type1 = compile_expr(compiler, fun, lhs);
    let type2 = compile_expr(compiler, fun, rhs);

    if type1 != type2 {
      compiler.throw(CompilerErrorKind::TypeMismatch, 0);
    }

    match operator_to_instruction(&op, &type1) {
      Ok(instruction) => {
        fun.instruction(instruction);
      }
      Err(err) => compiler.throw(err, 0),
    }

    match operator_to_ident_type(&op, &type1) {
      Ok(ident_type) => ident_type,
      Err(err) => {
        compiler.throw(err, 0);
        IdentType::Error
      }
    }
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
    Primary::Array(arr) => compile_array(compiler, fun, arr),
  }
}

pub fn compile_literal(compiler: &mut Compiler, fun: &mut Function, lit: Literal) -> IdentType {
  match lit {
    Literal::Bool(val) => {
      fun.instruction(Instruction::I32Const(if val { 1 } else { 0 }));

      if compiler.scope.expr_type.clone() != IdentType::Primitive(Primitive::Bool) {
        compiler.throw(CompilerErrorKind::TypeMismatch, 0);
      }
      compiler.scope.expr_type.clone()
    }
    Literal::Char(val) => {
      fun.instruction(Instruction::I32Const(val as i32));

      if compiler.scope.expr_type.clone() != IdentType::Primitive(Primitive::Char) {
        compiler.throw(CompilerErrorKind::TypeMismatch, 0);
      }
      compiler.scope.expr_type.clone()
    }
    Literal::Int(val) => {
      if let IdentType::Default = compiler.scope.expr_type {
        fun.instruction(Instruction::I32Const(val as i32));
        IdentType::Primitive(Primitive::I32)
      } else if let IdentType::Primitive(prim) = compiler.scope.expr_type.clone() {
        match prim {
          Primitive::F32 => {
            fun.instruction(Instruction::F32Const(val as f32));
            IdentType::Primitive(Primitive::F32)
          }
          Primitive::F64 => {
            fun.instruction(Instruction::F64Const(val as f64));
            IdentType::Primitive(Primitive::F64)
          }
          Primitive::I32 => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::I32)
          }
          Primitive::I64 => {
            fun.instruction(Instruction::I64Const(val as i64));
            IdentType::Primitive(Primitive::I64)
          }
          Primitive::U32 => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::U32)
          }
          Primitive::U64 => {
            fun.instruction(Instruction::I64Const(val as i64));
            IdentType::Primitive(Primitive::U64)
          }
          Primitive::Char => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::Char)
          }
          Primitive::Bool => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::Bool)
          }
          _ => IdentType::Error,
        }
      } else {
        IdentType::Error
      }
    }
    Literal::Float(val) => {
      if let IdentType::Default = compiler.scope.expr_type {
        fun.instruction(Instruction::F64Const(val as f64));
        IdentType::Primitive(Primitive::F64)
      } else if let IdentType::Primitive(prim) = compiler.scope.expr_type.clone() {
        match prim {
          Primitive::F32 => {
            fun.instruction(Instruction::F32Const(val as f32));
            IdentType::Primitive(Primitive::F32)
          }
          Primitive::F64 => {
            fun.instruction(Instruction::F64Const(val as f64));
            IdentType::Primitive(Primitive::F64)
          }
          Primitive::I32 => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::I32)
          }
          Primitive::I64 => {
            fun.instruction(Instruction::I64Const(val as i64));
            IdentType::Primitive(Primitive::I64)
          }
          Primitive::U32 => {
            fun.instruction(Instruction::I32Const(val as i32));
            IdentType::Primitive(Primitive::I32)
          }
          Primitive::U64 => {
            fun.instruction(Instruction::I64Const(val as i64));
            IdentType::Primitive(Primitive::I64)
          }
          _ => IdentType::Error,
        }
      } else {
        IdentType::Error
      }
    },
    Literal::Str(string) => {
      fun.instruction(Instruction::I32Const(compiler.memory.stack as i32));
      let bytes = string.as_bytes();
      compiler.memory.buf.extend(bytes);
      compiler.memory.stack += bytes.len() as u64;
      IdentType::Primitive(Primitive::Str)
    },
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
  match compiler.scope.get_sym(&ident) {
    Ok(sym) => {
      let ident_type = compile_ident_val(compiler, fun, sym.clone(), prim, 0);
      if compiler.scope.expr_type.clone() == IdentType::Default {
        compiler.scope.expr_type = ident_type.clone();
      }
      ident_type
    }
    Err(err) => {
      compiler.throw(err, 0);
      IdentType::Error
    }
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
    fun.instruction(if sym.1.global {
      Instruction::GlobalGet(sym.0)
    } else {
      Instruction::LocalGet(sym.0)
    });

    sym.1.types
  } else {
    let types = match &prim[index] {
      IdentVal::Arguments(args) => compile_arguments(compiler, fun, sym.clone(), args.clone()),
      IdentVal::Selector(ident) => compile_selector(compiler, fun, sym.clone(), ident.clone()),
      _ => {
        compiler.throw(CompilerErrorKind::Unimplemented, 0);
        IdentType::Error
      }
    };
    if prim.len() > index + 1 {
      compile_ident_val(compiler, fun, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn compile_array(compiler: &mut Compiler, fun: &mut Function, exprs: Vec<Expr>) -> IdentType {
  if let IdentType::Array(inner_type) = compiler.scope.expr_type.clone() {
    compiler.scope.expr_type = *inner_type
  } else {
    compiler.throw(CompilerErrorKind::TypeMismatch, 0);
  }
  let idx = compiler.memory.stack;
  let mut ident_type = compiler.scope.expr_type.clone();
  for (_, expr) in exprs.into_iter().enumerate() {
    let expr_type = compile_expr(compiler, fun, expr);
    if ident_type == expr_type {
      ident_type = expr_type.clone();
      let memarg = compiler.memory.index_stack();
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
      compiler.throw(CompilerErrorKind::TypeMismatch, 0);
    }
  }
  fun.instruction(Instruction::I64Const(idx as i64));
  IdentType::Array(Box::new(ident_type))
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
        compiler.scope.expr_type = param.type_ident.clone();
        if param.type_ident != compile_expr(compiler, fun, args[i].clone()) {
          compiler.throw(CompilerErrorKind::TypeMismatch, 0);
        }
      } else {
        compiler.throw(CompilerErrorKind::MissingParameters, 0);
      }
    }
    fun.instruction(Instruction::Call(sym.0));

    *ret_type
  } else {
    compiler.throw(CompilerErrorKind::MissingCallSignature, 0);
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

    compiler.throw(CompilerErrorKind::MissingProperty, 0);
  }
  compiler.throw(CompilerErrorKind::NoProperties, 0);
  IdentType::Error
}

pub fn compile_cond(
  compiler: &mut Compiler,
  fun: &mut Function,
  cond: Expr,
  then_expr: Expr,
  else_expr: Expr,
) -> IdentType {
  let type1 = compile_expr(compiler, fun, then_expr);
  let type2 = compile_expr(compiler, fun, else_expr);
  compile_bool_expr(compiler, fun, cond);

  fun.instruction(Instruction::Select);

  if type1 != type2 {
    IdentType::Error
  } else {
    type1
  }
}
