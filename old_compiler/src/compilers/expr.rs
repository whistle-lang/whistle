use crate::compiler::Compiler;
use crate::compilers::compile_literal;
use crate::compilers::compile_unary_op;
use crate::encoding::unsigned_leb128;
use crate::opcodes::Opcode;
use crate::types::Type;

use whistle_ast::Expr;
use whistle_ast::Operand;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Unary;

pub fn compile_expr(compiler: &mut Compiler, expr: Expr) -> Type {
  match expr {
    Expr::Binary { op, rhs, lhs } => compile_bin_expr(compiler, op, *rhs, *lhs),
    Expr::Unary(expr) => compile_unary(compiler, expr),
    _ => panic!("exp"),
  }
}

pub fn compile_bin_expr(compiler: &mut Compiler, op: Operator, rhs: Expr, lhs: Expr) -> Type {
  let type1 = compile_expr(compiler, lhs);
  let type2 = compile_expr(compiler, rhs);

  if type1 != type2 {
    panic!("Mismatched types!")
  }
  if !type1.is_number() {
    panic!("Not a number!")
  }

  compiler
    .func
    .code
    .push(Opcode::from_operator(&op, &type1) as u8);
  type1
}

pub fn compile_unary(compiler: &mut Compiler, expr: Unary) -> Type {
  match expr {
    Unary::Primary(expr) => compile_primary(compiler, expr),
    Unary::UnaryOp { op, expr } => compile_unary_op(compiler, op, *expr),
  }
}

pub fn compile_primary(compiler: &mut Compiler, expr: Primary) -> Type {
  match expr {
    Primary::Operand(op) => compile_operand(compiler, op),
    Primary::Arguments { prim, args } => compile_arguments(compiler, *prim, args),
    _ => panic!("prim"),
  }
}

pub fn compile_operand(compiler: &mut Compiler, op: Operand) -> Type {
  match op {
    Operand::Ident(ident) => compile_ident(compiler, &ident),
    Operand::Literal(lit) => compile_literal(compiler, lit),
    Operand::Grouping(grp) => compile_expr(compiler, *grp),
  }
}

pub fn compile_ident(compiler: &mut Compiler, ident: &str) -> Type {
  compiler.func.code.push(Opcode::LocalGet as u8);
  let var = compiler.get_local(ident);
  compiler.func.code.extend(unsigned_leb128(var.index));
  var.local_type
}

pub fn compile_arguments(compiler: &mut Compiler, prim: Primary, args: Vec<Expr>) -> Type {
  if let Primary::Operand(Operand::Ident(name)) = prim {
    let func = compiler.get_func(&name);
    for i in 0..func.param_types.len() {
      compile_expr(compiler, args[i].clone());
      // if !Type::is_compat(&func.param_types[i], &type1) {
      //   panic!("Mismatched types!")
      // }
    }
    compiler.func.code.push(Opcode::Call as u8);
    compiler.func.code.extend(unsigned_leb128(func.index));
    func.result_types[0].clone()
  } else {
    panic!("Can only compile ident calls")
  }
}
