use crate::compiler::Compiler;
use crate::compilers::compile_unary;
use crate::encoding::signed_leb128;
use crate::encoding::unsigned_leb128;
use crate::opcodes::Opcode;
use crate::types::Primitive;
use crate::types::Type;

use whistle_ast::Literal;
use whistle_ast::Operand;
use whistle_ast::Operator;
use whistle_ast::Primary;
use whistle_ast::Unary;

pub fn compile_unary_op(compiler: &mut Compiler, op: Operator, expr: Unary) -> Type {
  match op {
    Operator::Sub => compile_sub(compiler, expr),
    _ => panic!("un"),
  }
}

pub fn compile_sub(compiler: &mut Compiler, expr: Unary) -> Type {
  if let Unary::Primary(Primary::Operand(Operand::Literal(lit))) = expr.clone() {
    if let Literal::Int(num) = lit {
      compiler.func.code.push(Opcode::I64Const as u8);
      compiler.func.code.extend(signed_leb128(-(num as isize)));
      return Type::Primitive(Primitive::Int);
    }
    if let Literal::Float(num) = lit {
      compiler.func.code.push(Opcode::F64Const as u8);
      compiler.func.code.extend(signed_leb128(-(num as isize)));
      return Type::Primitive(Primitive::Float);
    }
  }

  let type1 = compile_unary(compiler, expr);
  if !type1.is_number() {
    panic!("Not a number!")
  }
  compiler.func.code.push(Opcode::as_const(&type1) as u8);
  compiler.func.code.extend(unsigned_leb128(0));
  compiler
    .func
    .code
    .push(Opcode::from_operator(&Operator::Sub, &type1) as u8);
  type1
}

pub fn compile_literal(compiler: &mut Compiler, lit: Literal) -> Type {
  match lit {
    Literal::Int(int) => compile_int_literal(compiler, int),
    Literal::Char(cha) => compile_char_literal(compiler, cha),
    Literal::Str(string) => compile_str_literal(compiler, string),
    _ => panic!("lit"),
  }
}

pub fn compile_int_literal(compiler: &mut Compiler, int: usize) -> Type {
  compiler.func.code.push(Opcode::I64Const as u8);
  compiler.func.code.extend(unsigned_leb128(int));
  Type::Primitive(Primitive::Int)
}

pub fn compile_char_literal(compiler: &mut Compiler, cha: char) -> Type {
  compiler.func.code.push(Opcode::I32Const as u8);
  compiler.func.code.extend(unsigned_leb128(cha as usize));
  Type::Primitive(Primitive::Char)
}

pub fn compile_str_literal(compiler: &mut Compiler, string: String) -> Type {
  compiler.strmem.push(string);
  compiler.func.code.push(Opcode::I32Const as u8);
  compiler.func.code.extend(unsigned_leb128(0));
  Type::Primitive(Primitive::String)
  // compiler.func.code.extend(encode_string(&string[..]));
}
