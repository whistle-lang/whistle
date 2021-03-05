use crate::opcodes::Opcode;
use crate::opcodes::ValType;

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
  Ident(String),
  Primitive(Primitive),
  Union(Union),
  Struct,
  IdentType {
    ident: String,
    prim: Vec<Type>,
  },
  None,
}

impl Type {
  pub fn from(name: &str) -> Type {
    match name {
      "f32" => Type::Primitive(Primitive::F32),
      "f64" => Type::Primitive(Primitive::F64),
      "i32" => Type::Primitive(Primitive::I32),
      "i64" => Type::Primitive(Primitive::I64),
      "u32" => Type::Primitive(Primitive::U32),
      "u64" => Type::Primitive(Primitive::U64),
      "char" => Type::Primitive(Primitive::Char),
      "bool" => Type::Primitive(Primitive::Bool),
      "string" => Type::Primitive(Primitive::String),

      _ => panic!("Unknown type name {}", name),
    }
  }

  pub fn is_number(&self) -> bool {
    vec![
      Type::Primitive(Primitive::F32),
      Type::Primitive(Primitive::F64),
      Type::Primitive(Primitive::I32),
      Type::Primitive(Primitive::I64),
      Type::Primitive(Primitive::U32),
      Type::Primitive(Primitive::U64),
      Type::Primitive(Primitive::Char),
      Type::Primitive(Primitive::Bool),
    ]
    .contains(self)
  }

  pub fn to_valtype(&self) -> ValType {
    match self {
      Type::Primitive(Primitive::F32) => ValType::F32,
      Type::Primitive(Primitive::F64) => ValType::F64,
      Type::Primitive(Primitive::I32) => ValType::I32,
      Type::Primitive(Primitive::I64) => ValType::I64,
      Type::Primitive(Primitive::U32) => ValType::I32,
      Type::Primitive(Primitive::U64) => ValType::I64,

      Type::Primitive(Primitive::Int) => ValType::I64,
      Type::Primitive(Primitive::Float) => ValType::F64,

      Type::Primitive(Primitive::Char) => ValType::I32,
      Type::Primitive(Primitive::Bool) => ValType::I32,

      Type::Primitive(Primitive::String) => ValType::I32,

      Type::Union(_) => ValType::I32,
      Type::Struct => ValType::I32,
      Type::None => ValType::Empty,
    }
  }

  pub fn is_subtype_of(&self, type2: &Type) -> bool {
    match self {
      Type::Union(un) => un.is_subtype_of(type2),
      Type::Primitive(prim) => prim.is_subtype_of(type2),
      Type::Struct => self == type2,
      _ => panic!("subtype"),
    }
  }

  pub fn is_compat(type1: &Type, type2: &Type) -> bool {
    type1.is_subtype_of(type2) || type2.is_subtype_of(type1) || type1 == type2
  }

  pub fn convert(&self, type2: &Type) -> Result<Option<Opcode>, ()> {
    if self != type2 {
      if let Type::Primitive(prim1) = self {
        if let Type::Primitive(prim2) = type2 {
          if let Some(new_type) = prim1.to_default().convert(prim2.to_default()) {
            return Ok(Some(new_type));
          } else {
            return Ok(None);
          }
        }
      }
    }
    Err(())
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
  F32,
  F64,
  I32,
  I64,
  U32,
  U64,
  Char,
  Bool,
  String,

  Int,
  Float,
}

impl Primitive {
  pub fn get_subtypes(&self) -> &[Primitive] {
    match self {
      Primitive::Int => &[
        Primitive::I32,
        Primitive::I64,
        Primitive::U32,
        Primitive::U64,
      ],
      Primitive::Float => &[Primitive::F32, Primitive::F64],
      _ => &[],
    }
  }

  pub fn is_subtype_of(&self, type2: &Type) -> bool {
    if let Type::Primitive(prim) = type2 {
      for types in self.get_subtypes() {
        if types == prim {
          return true;
        }
      }
    }
    false
  }

  pub fn to_default(&self) -> &Primitive {
    match self {
      Primitive::Int => &Primitive::I64,
      Primitive::Float => &Primitive::F64,
      _ => self,
    }
  }

  pub fn convert(&self, to: &Primitive) -> Option<Opcode> {
    match self {
      Primitive::I32 => match to {
        Primitive::I64 => Some(Opcode::I32WrapI64),

        Primitive::U32 => None,
        Primitive::U64 => Some(Opcode::I32WrapI64),

        Primitive::F32 => Some(Opcode::I32TruncSignedF32),
        Primitive::F64 => Some(Opcode::I32TruncSignedF64),
        _ => panic!("Type conversion not covered! ({:?} to {:?})", self, to),
      },

      Primitive::I64 => match to {
        Primitive::I32 => Some(Opcode::I64ExtendSignedI32),

        Primitive::U32 => Some(Opcode::I64ExtendUnsignedI32),
        Primitive::U64 => None,

        Primitive::F32 => Some(Opcode::I64TruncSignedF32),
        Primitive::F64 => Some(Opcode::I64TruncSignedF64),
        _ => panic!("Type conversion not covered! ({:?} to {:?})", self, to),
      },

      Primitive::F32 => match to {
        Primitive::I32 => Some(Opcode::F32ConvertSignedI32),
        Primitive::I64 => Some(Opcode::F32ConvertSignedI64),

        Primitive::U32 => Some(Opcode::F32ConvertUnsignedI32),
        Primitive::U64 => Some(Opcode::F32ConvertUnsignedI64),

        Primitive::F64 => Some(Opcode::F32DemoteF64),
        _ => panic!("Type conversion not covered! ({:?} to {:?})", self, to),
      },

      Primitive::F64 => match to {
        Primitive::I32 => Some(Opcode::F64ConvertSignedI32),
        Primitive::I64 => Some(Opcode::F64ConvertSignedI64),

        Primitive::U32 => Some(Opcode::F64ConvertUnsignedI32),
        Primitive::U64 => Some(Opcode::F64ConvertUnsignedI64),

        Primitive::F32 => Some(Opcode::F64PromoteF32),
        _ => panic!("Type conversion not covered! ({:?} to {:?})", self, to),
      },

      _ => panic!("Type conversion not covered! ({:?} to {:?})", self, to),
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Union {
  subtypes: Vec<Type>,
}

impl Union {
  pub fn from(subtypes: Vec<Type>) -> Union {
    Union { subtypes }
  }

  pub fn is_subtype_of(&self, type2: &Type) -> bool {
    for types in self.subtypes.clone() {
      if types.is_subtype_of(type2) || &types == type2 {
        return true;
      }
    }
    false
  }
}
