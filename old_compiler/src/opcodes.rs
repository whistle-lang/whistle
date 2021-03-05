// https://webassembly.github.io/spec/core/binary/modules.html#sections
use crate::types::Primitive;
use crate::types::Type;
use whistle_ast::Operator;

pub enum Section {
  Custom = 0,
  Type = 1,
  Import = 2,
  Func = 3,
  Table = 4,
  Memory = 5,
  Global = 6,
  Export = 7,
  Start = 8,
  Element = 9,
  Code = 10,
  Data = 11,
}

// https://webassembly.github.io/spec/core/binary/instructions.html
pub enum Opcode {
  Unreachable = 0,
  Nop = 1,
  Block = 2,
  Loop = 3,
  If = 4,
  Else = 5,
  End = 11,
  Br = 12,
  BrIf = 13,
  BrTable = 14,
  Return = 15,
  Call = 16,
  CallIndirect = 17,
  Drop = 26,
  Select = 27,

  LocalGet = 32,
  LocalSet = 33,
  LocalTee = 34,

  GlobalGet = 35,
  GlobalSet = 36,

  I32Load = 40,
  I64Load = 41,
  F32Load = 42,
  F64Load = 43,

  I32Load8Signed = 44,
  I32Load8Unsigned = 45,
  I32Load16Signed = 46,
  I32Load16Unsigned = 47,
  I64Load8Signed = 48,
  I64Load8Unsigned = 49,
  I64Load16Signed = 50,
  I64Load16Unsigned = 51,
  I64Load32Signed = 52,
  I64Load32Unsigned = 53,

  I32Store = 54,
  I64Store = 55,
  F32Store = 56,
  F64Store = 57,

  I32Store8 = 58,
  I32Store16 = 59,

  I64Store8 = 60,
  I64Store16 = 61,
  I64Store32 = 62,

  MemorySize = 63,
  MemoryGrow = 64,

  I32Const = 65,
  I64Const = 66,
  F32Const = 67,
  F64Const = 68,

  I32Eqz = 69,
  I32Eq = 70,
  I32NotEq = 71,
  I32LessThanSigned = 72,
  I32LessThanUnsigned = 73,
  I32GreaterThanSigned = 74,
  I32GreaterThanUnsigned = 75,
  I32LessThanOrEqSigned = 76,
  I32LessThanOrEqUnsigned = 77,
  I32GreaterThanOrEqSigned = 78,
  I32GreaterThanOrEqUnsigned = 79,

  I64Eqz = 80,
  I64Eq = 81,
  I64NotEq = 82,
  I64LessThanSigned = 83,
  I64LessThanUnsigned = 84,
  I64GreaterThanSigned = 85,
  I64GreaterThanUnsigned = 86,
  I64LessThanOrEqSigned = 87,
  I64LessThanOrEqUnsigned = 88,
  I64GreaterThanOrEqSigned = 89,
  I64GreaterThanOrEqUnsigned = 90,

  F32Eq = 91,
  F32NotEq = 92,
  F32LesserThan = 93,
  F32GreaterThan = 94,
  F32LesserThanOrEq = 95,
  F32GreaterThanOrEq = 96,

  F64Eq = 97,
  F64NotEq = 98,
  F64LesserThan = 99,
  F64GreaterThan = 100,
  F64LesserThanOrEq = 101,
  F64GreaterThanOrEq = 102,

  I32Clz = 103,
  I32Ctz = 104,
  I32Popcnt = 105,
  I32Add = 106,
  I32Sub = 107,
  I32Mul = 108,
  I32DivSigned = 109,
  I32DivUnsigned = 110,
  I32RemSigned = 111,
  I32RemUnsigned = 112,
  I32BitAnd = 113,
  I32BitOr = 114,
  I32BitXor = 115,
  I32BitLeftShift = 116,
  I32BitRightShiftSigned = 117,
  I32BitRightShiftUnsigned = 118,
  I32Rotl = 119,
  I32Rotr = 120,

  I64Clz = 121,
  I64Ctz = 122,
  I64Popcnt = 123,
  I64Add = 124,
  I64Sub = 125,
  I64Mul = 126,
  I64DivSigned = 127,
  I64DivUnsigned = 128,
  I64RemSigned = 129,
  I64RemUnsigned = 130,
  I64BitAnd = 131,
  I64BitOr = 132,
  I64BitXor = 133,
  I64BitLeftShift = 134,
  I64BitRightShiftSigned = 135,
  I64BitRightShiftUnsigned = 136,
  I64Rotl = 137,
  I64Rotr = 138,

  F32Abs = 139,
  F32Neg = 140,
  F32Ceil = 141,
  F32Floor = 142,
  F32Trunc = 143,
  F32Nearest = 144,
  F32Sqrt = 145,
  F32Add = 146,
  F32Sub = 147,
  F32Mul = 148,
  F32Div = 149,
  F32Min = 150,
  F32Max = 151,
  F32Copysign = 152,

  F64Abs = 153,
  F64Neg = 154,
  F64Ceil = 155,
  F64Floor = 156,
  F64Trunc = 157,
  F64Nearest = 158,
  F64Sqrt = 159,
  F64Add = 160,
  F64Sub = 161,
  F64Mul = 162,
  F64Div = 163,
  F64Min = 164,
  F64Max = 165,
  F64Copysign = 166,
  I32WrapI64 = 167,

  I32TruncSignedF32 = 168,
  I32TruncUnsignedF32 = 169,

  I32TruncSignedF64 = 170,
  I32TruncUnsignedF64 = 171,

  I64ExtendSignedI32 = 172,
  I64ExtendUnsignedI32 = 173,

  I64TruncSignedF32 = 174,
  I64TruncUnsignedF32 = 175,

  I64TruncSignedF64 = 176,
  I64TruncUnsignedF64 = 177,

  F32ConvertSignedI32 = 178,
  F32ConvertUnsignedI32 = 179,

  F32ConvertSignedI64 = 180,
  F32ConvertUnsignedI64 = 181,

  F32DemoteF64 = 182,

  F64ConvertSignedI32 = 183,
  F64ConvertUnsignedI32 = 184,

  F64ConvertSignedI64 = 185,
  F64ConvertUnsignedI64 = 186,

  F64PromoteF32 = 187,

  I32ReinterpretF32 = 188,
  I64ReinterpretF64 = 189,
  F32ReinterpretI32 = 190,
  F64ReinterpretI64 = 191,
}

impl Opcode {
  pub fn as_const(val_type: &Type) -> Opcode {
    match val_type.to_valtype() {
      ValType::I32 => Opcode::I32Const,
      ValType::I64 => Opcode::I64Const,
      ValType::F32 => Opcode::F32Const,
      ValType::F64 => Opcode::F64Const,
      _ => panic!("Unknown const value type"),
    }
  }

  pub fn from_operator(op: &Operator, val_type: &Type) -> Opcode {
    if let Type::Primitive(primitive) = val_type {
      match op {
        Operator::Add => match primitive {
          Primitive::F32 => Opcode::F32Add,
          Primitive::F64 => Opcode::F64Add,

          Primitive::I32 => Opcode::I32Add,
          Primitive::U32 => Opcode::I32Add,
          Primitive::I64 => Opcode::I64Add,
          Primitive::U64 => Opcode::I64Add,

          _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
        },
        Operator::Sub => match primitive {
          Primitive::F32 => Opcode::F32Sub,
          Primitive::F64 => Opcode::F64Sub,

          Primitive::I32 => Opcode::I32Sub,
          Primitive::U32 => Opcode::I32Sub,
          Primitive::I64 => Opcode::I64Sub,
          Primitive::U64 => Opcode::I64Sub,

          _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
        },
        Operator::Mul => match primitive {
          Primitive::F32 => Opcode::F32Mul,
          Primitive::F64 => Opcode::F64Mul,

          Primitive::I32 => Opcode::I32Mul,
          Primitive::U32 => Opcode::I32Mul,
          Primitive::I64 => Opcode::I64Mul,
          Primitive::U64 => Opcode::I64Mul,

          _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
        },
        Operator::Div => match primitive {
          Primitive::F32 => Opcode::F32Div,
          Primitive::F64 => Opcode::F64Div,

          Primitive::I32 => Opcode::I32DivSigned,
          Primitive::U32 => Opcode::I32DivUnsigned,
          Primitive::I64 => Opcode::I64DivSigned,
          Primitive::U64 => Opcode::I64DivUnsigned,

          _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
        },
        Operator::Mod => match primitive {
          Primitive::I32 => Opcode::I32RemSigned,
          Primitive::U32 => Opcode::I32RemUnsigned,
          Primitive::I64 => Opcode::I64RemSigned,
          Primitive::U64 => Opcode::I64RemUnsigned,

          _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
        },
        _ => panic!("Operator {:?} does not exist for type {:?}", op, primitive),
      }
    } else {
      panic!("Only primitive types can currently be operated on")
    }
  }
}

// http://webassembly.github.io/spec/core/binary/modules.html#export-section
pub enum ExportType {
  Func = 0x00,
  Table = 0x01,
  Mem = 0x02,
  Global = 0x03,
}

// https://webassembly.github.io/spec/core/binary/types.html#value-types
pub enum ValType {
  I32 = 0x7f,
  I64 = 0x7e,
  F32 = 0x7d,
  F64 = 0x7c,
  Empty = 0x40,
}

// https://webassembly.github.io/spec/core/binary/types.html#binary-blocktype
pub enum Blocktype {
  Empty = 0x40,
}

pub enum Names {
  Mut = 0x01,
  FunctionType = 0x60,
  EmptyArray = 0x0,
}

pub const MAGIC_MODULE_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
pub const MODULE_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
