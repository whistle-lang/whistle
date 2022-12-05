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

pub fn compile_expr(compiler: &mut Compiler, function: &mut Function, expr: Expr) -> IdentType {
  match expr {
    Expr::Binary { op, rhs, lhs } => compile_bin_expr(compiler, function, op, *rhs, *lhs),
    Expr::Unary(expr) => compile_unary(compiler, function, expr),
    Expr::Cond {
      cond,
      then_expr,
      else_expr,
    } => compile_cond(compiler, function, *cond, *then_expr, *else_expr),
  }
}

pub fn compile_bin_expr(
  compiler: &mut Compiler,
  function: &mut Function,
  op: Operator,
  rhs: Expr,
  lhs: Expr,
) -> IdentType {
  if op == Operator::Assign {
    if let Expr::Unary(Unary::Primary(Primary::IdentVal { ident, .. })) = lhs {
      let type1 = compile_expr(compiler, function, rhs);
      let sym = match compiler.scope.get_sym(&ident) {
        Ok(sym) => sym.clone(),
        Err(err) => {
          compiler.throw(err, 0);
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
      IdentType::Error
    }
  } else {
    let type1 = compile_expr(compiler, function, lhs);
    compile_expr(compiler, function, rhs);

    match operator_to_instruction(&op, &type1) {
      Ok(instruction) => {
        function.instruction(instruction);
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

pub fn compile_unary(compiler: &mut Compiler, function: &mut Function, expr: Unary) -> IdentType {
  match expr {
    Unary::Primary(expr) => compile_primary(compiler, function, expr),
    Unary::UnaryOp { op: _, expr } => compile_unary(compiler, function, *expr),
  }
}

pub fn compile_primary(
  compiler: &mut Compiler,
  function: &mut Function,
  expr: Primary,
) -> IdentType {
  match expr {
    Primary::Literal(lit) => compile_literal(compiler, function, lit),
    Primary::IdentVal { ident, prim } => compile_ident(compiler, function, ident, prim),
    Primary::Grouping(expr) => compile_expr(compiler, function, *expr),
    Primary::Array{ exprs, type_ident} => compile_array(compiler, function, exprs, type_ident),
  }
}

pub fn compile_literal(
  compiler: &mut Compiler,
  function: &mut Function,
  lit: Literal,
) -> IdentType {
  match lit {
    Literal::Bool(val) => {
      function.instruction(Instruction::I32Const(if val { 1 } else { 0 }));
      IdentType::Primitive(Primitive::Bool)
    }
    Literal::Char(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::Char)
    }
    Literal::Int(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::I32)
    }
    Literal::Float(val) => {
      function.instruction(Instruction::F64Const(val as f64));
      IdentType::Primitive(Primitive::F64)
    }
    Literal::F32(val) => {
      function.instruction(Instruction::F32Const(val as f32));
      IdentType::Primitive(Primitive::F32)
    }
    Literal::F64(val) => {
      function.instruction(Instruction::F64Const(val as f64));
      IdentType::Primitive(Primitive::F64)
    }
    Literal::I32(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::I32)
    }
    Literal::I64(val) => {
      function.instruction(Instruction::I64Const(val as i64));
      IdentType::Primitive(Primitive::I64)
    }
    Literal::U32(val) => {
      function.instruction(Instruction::I32Const(val as i32));
      IdentType::Primitive(Primitive::I32)
    }
    Literal::U64(val) => {
      function.instruction(Instruction::I64Const(val as i64));
      IdentType::Primitive(Primitive::I64)
    }
    Literal::Str(string) => {
      function.instruction(Instruction::I32Const(compiler.memory.stack as i32));
      let bytes = string.as_bytes();
      compiler.memory.buf.extend(bytes);
      compiler.memory.stack += bytes.len() as u64;
      IdentType::Primitive(Primitive::Str)
    }
    Literal::None => IdentType::Primitive(Primitive::None),
  }
}


pub fn compile_ident(
  compiler: &mut Compiler,
  function: &mut Function,
  ident: String,
  prim: Vec<IdentVal>,
) -> IdentType {
  match compiler.scope.get_sym(&ident) {
    Ok(sym) => compile_ident_val(compiler, function, sym.clone(), prim, 0),
    Err(err) => {
      compiler.throw(err, 0);
      IdentType::Error
    }
  }
}

pub fn compile_ident_val(
  compiler: &mut Compiler,
  function: &mut Function,
  sym: IndexedSymbol,
  prim: Vec<IdentVal>,
  index: usize,
) -> IdentType {
  if prim.is_empty() {
    function.instruction(if sym.1.global {
      Instruction::GlobalGet(sym.0)
    } else {
      Instruction::LocalGet(sym.0)
    });

    sym.1.types
  } else {
    let types = match &prim[index] {
      IdentVal::Arguments(args) => compile_arguments(compiler, function, sym.clone(), args.clone()),
      IdentVal::Selector(ident) => compile_selector(compiler, function, sym.clone(), ident.clone()),
      _ => {
        compiler.throw(CompilerErrorKind::Unimplemented, 0);
        IdentType::Error
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
  ident_type: IdentType,
) -> IdentType {
  let idx = compiler.memory.stack;
  for (_, expr) in exprs.into_iter().enumerate() {
    let expr_type = compile_expr(compiler, function, expr);
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
    function.instruction(instruction);
  }
  function.instruction(Instruction::I64Const(idx as i64));
  IdentType::Array(Box::new(ident_type))
}

pub fn compile_arguments(
  compiler: &mut Compiler,
  function: &mut Function,
  sym: IndexedSymbol,
  args: Vec<Expr>,
) -> IdentType {
  if let IdentType::Function { params, ret_type } = sym.1.types {
    for arg in args.iter().take(params.len()) {
      compile_expr(compiler, function, arg.clone());
    }
    function.instruction(Instruction::Call(sym.0));

    *ret_type
  } else {
    IdentType::Error
  }
}

pub fn compile_selector(
  compiler: &mut Compiler,
  _function: &mut Function,
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
  function: &mut Function,
  cond: Expr,
  then_expr: Expr,
  else_expr: Expr,
) -> IdentType {
  let type1 = compile_expr(compiler, function, then_expr);
  compile_expr(compiler, function, else_expr);
  compile_expr(compiler, function, cond);

  function.instruction(Instruction::Select);

  type1
}
