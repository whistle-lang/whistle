use crate::operator_to_ident_type;
use crate::operator_to_instruction;
use crate::Compiler;
use crate::Function;
use crate::IndexedSymbol;
use crate::Symbol;
use whistle_common::CompilerErrorKind;

use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentVal;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Type;
use whistle_ast::Unary;
use whistle_common::CompilerHandler;
use whistle_common::Primitive;
use whistle_common::Span;

pub fn compile_expr(compiler: &mut Compiler, function: &mut Function, expr: Expr) -> Type {
  match expr {
    Expr::Binary { op, rhs, lhs, .. } => compile_bin_expr(compiler, function, op, *rhs, *lhs),
    Expr::Unary { unary, .. } => compile_unary(compiler, function, unary),
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
      ..
    } => compile_cond(compiler, function, *cond, *then_expr, *else_expr),
  }
}

pub fn compile_bin_expr(
  compiler: &mut Compiler,
  function: &mut Function,
  op: Operator,
  rhs: Expr,
  lhs: Expr,
) -> Type {
  if op == Operator::Assign {
    if let Expr::Unary {
      unary: Unary::Primary {
        prim: Primary::IdentVal { ident, .. },
        ..
      },
      span,
    } = lhs
    {
      let type1 = compile_expr(compiler, function, rhs);
      let sym = match compiler.get_sym(&ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          compiler.handler.throw(err, span);
          IndexedSymbol(0, Symbol::default())
        }
      };

      if sym.1.global {
        function.instruction(Instruction::GlobalSet(sym.0));
      } else {
        function.instruction(Instruction::LocalSet(sym.0));
      }

      type1
    } else {
      Type::Error
    }
  } else {
    let type1 = compile_expr(compiler, function, lhs.clone());
    compile_expr(compiler, function, rhs);

    match operator_to_instruction(&op, &type1) {
      Ok(instruction) => {
        function.instruction(instruction);
      }
      Err(_) => return Type::Error,
    }

    match operator_to_ident_type(&op, &type1) {
      Ok(ident_type) => ident_type,
      Err(_) => return Type::Error,
    }
  }
}

pub fn compile_unary(compiler: &mut Compiler, function: &mut Function, expr: Unary) -> Type {
  match expr {
    Unary::Primary { prim, .. } => compile_primary(compiler, function, prim),
    Unary::UnaryOp { op: _, expr, .. } => compile_unary(compiler, function, *expr),
  }
}

pub fn compile_primary(compiler: &mut Compiler, function: &mut Function, expr: Primary) -> Type {
  match expr {
    Primary::Literal { lit, meta_id, .. } => compile_literal(compiler, function, lit, meta_id),
    Primary::IdentVal { ident, prim, span } => compile_ident(compiler, function, ident, prim, span),
    Primary::Grouping { group, .. } => compile_expr(compiler, function, *group),
    Primary::Array { exprs, meta_id, .. } => compile_array(compiler, function, exprs, meta_id),
  }
}

pub fn compile_literal(
  compiler: &mut Compiler,
  function: &mut Function,
  lit: Literal,
  id: usize,
) -> Type {
  match lit {
    Literal::Bool(val) => {
      function.instruction(Instruction::I32Const(if val { 1 } else { 0 }));
      Type::Primitive(Primitive::Bool)
    }
    Literal::Char(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      Type::Primitive(Primitive::Char)
    }
    Literal::Int(val) => {
      match &compiler.substitutions[id] {
        Type::Primitive(Primitive::I32) => function.instruction(Instruction::I32Const(val as i32)),
        Type::Primitive(Primitive::I64) => function.instruction(Instruction::I64Const(val as i64)),
        _ => unreachable!(),
      };
      return compiler.substitutions[id].clone();
    }
    Literal::Float(val) => {
      match &compiler.substitutions[id] {
        Type::Primitive(Primitive::F32) => function.instruction(Instruction::F32Const(val as f32)),
        Type::Primitive(Primitive::F64) => function.instruction(Instruction::F64Const(val as f64)),
        _ => unreachable!(),
      };
      return compiler.substitutions[id].clone();
    }
    Literal::F32(val) => {
      function.instruction(Instruction::F32Const(val as f32));
      Type::Primitive(Primitive::F32)
    }
    Literal::F64(val) => {
      function.instruction(Instruction::F64Const(val as f64));
      Type::Primitive(Primitive::F64)
    }
    Literal::I32(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      Type::Primitive(Primitive::I32)
    }
    Literal::I64(val) => {
      function.instruction(Instruction::I64Const(val as i64));
      Type::Primitive(Primitive::I64)
    }
    Literal::U32(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      Type::Primitive(Primitive::I32)
    }
    Literal::U64(val) => {
      function.instruction(Instruction::I64Const(val as i64));
      Type::Primitive(Primitive::I64)
    }
    Literal::Str(string) => {
      function.instruction(Instruction::I32Const(compiler.memory.stack as i32));
      let bytes = string.as_bytes();
      compiler.memory.buf.extend(bytes);
      compiler.memory.stack += bytes.len() as u64;
      Type::Primitive(Primitive::Str)
    }
    Literal::None => Type::Primitive(Primitive::None),
  }
}

pub fn compile_ident(
  compiler: &mut Compiler,
  function: &mut Function,
  ident: String,
  prim: Vec<IdentVal>,
  span: Span,
) -> Type {
  let sym = match compiler.get_sym(&ident) {
    Ok(sym) => sym,
    Err(err) => {
      compiler.handler.throw(err, span);
      return Type::Error;
    }
  };
  compile_ident_val(compiler, function, sym, prim, 0)
}

pub fn compile_ident_val(
  compiler: &mut Compiler,
  function: &mut Function,
  sym: IndexedSymbol,
  prim: Vec<IdentVal>,
  index: usize,
) -> Type {
  if prim.is_empty() {
    function.instruction(if sym.1.global {
      Instruction::GlobalGet(sym.0)
    } else {
      Instruction::LocalGet(sym.0)
    });

    sym.1.types
  } else {
    let types = match &prim[index] {
      IdentVal::Arguments { args, .. } => {
        compile_arguments(compiler, function, sym.clone(), args.clone())
      }
      IdentVal::Selector { ident, span } => {
        compile_selector(compiler, function, sym.clone(), ident.clone(), span)
      }
      _ => {
        compiler
          .handler
          .throw(CompilerErrorKind::Unimplemented, prim[index].span());
        return Type::Error;
      }
    };
    if prim.len() > index + 1 {
      compile_ident_val(compiler, function, sym, prim, index + 1)
    } else {
      types
    }
  }
}

pub fn compile_array(
  compiler: &mut Compiler,
  function: &mut Function,
  exprs: Vec<Expr>,
  id: usize,
) -> Type {
  let idx = compiler.memory.stack;
  if let Type::Array(expr_type) = compiler.substitutions[id].clone() {
    for (_, expr) in exprs.into_iter().enumerate() {
      let span = expr.span();
      compile_expr(compiler, function, expr);
      let memarg = compiler.memory.index_stack();
      let instruction = match &*expr_type {
        Type::Primitive(prim) => match prim {
          Primitive::I32 => Instruction::I32Store(memarg),
          Primitive::F32 => Instruction::F32Store(memarg),
          Primitive::I64 => Instruction::I64Store(memarg),
          Primitive::F64 => Instruction::F64Store(memarg),
          _ => {
            compiler
              .handler
              .throw(CompilerErrorKind::Unimplemented, span);
            return Type::Error;
          }
        },
        _ => {
          compiler
            .handler
            .throw(CompilerErrorKind::Unimplemented, span);
          return Type::Error;
        }
      };
      function.instruction(instruction);
    }
    function.instruction(Instruction::I64Const(idx as i64));
    return Type::Array(expr_type);
  }
  unreachable!()
}

pub fn compile_arguments(
  compiler: &mut Compiler,
  function: &mut Function,
  sym: IndexedSymbol,
  args: Vec<Expr>,
) -> Type {
  if let Type::Function { params, ret_type } = sym.1.types {
    for arg in args.iter().take(params.len()) {
      compile_expr(compiler, function, arg.clone());
    }
    function.instruction(Instruction::Call(sym.0));

    *ret_type
  } else {
    Type::Error
  }
}

pub fn compile_selector(
  compiler: &mut Compiler,
  _function: &mut Function,
  sym: IndexedSymbol,
  ident: String,
  span: &Span,
) -> Type {
  if let Type::Struct(props) = sym.1.types {
    for prop in props {
      if prop.ident == ident {
        return prop.type_ident;
      }
    }

    compiler
      .handler
      .throw(CompilerErrorKind::MissingProperty, span.clone());
  }
  compiler
    .handler
    .throw(CompilerErrorKind::NoProperties, span.clone());
  Type::Error
}

pub fn compile_cond(
  compiler: &mut Compiler,
  function: &mut Function,
  cond: Expr,
  then_expr: Expr,
  else_expr: Expr,
) -> Type {
  let type1 = compile_expr(compiler, function, then_expr);
  compile_expr(compiler, function, else_expr);
  compile_expr(compiler, function, cond);

  function.instruction(Instruction::Select);

  type1
}
