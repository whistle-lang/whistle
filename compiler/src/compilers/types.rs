use crate::CompilerErrorKind;

use wasm_encoder::Instruction;
use wasm_encoder::ValType;

use whistle_ast::IdentType;
use whistle_ast::Operator;
use whistle_ast::Primitive;

pub fn ident_type_to_val_type(ident_type: IdentType) -> ValType {
  match ident_type {
    IdentType::Primitive(prim) => prim_to_val_type(prim),
    IdentType::Array(_) => ValType::I64,
    _ => panic!(),
  }
}

pub fn prim_to_val_type(prim: Primitive) -> ValType {
  match prim {
    Primitive::F32 => ValType::F32,
    Primitive::F64 => ValType::F64,
    Primitive::I32 => ValType::I32,
    Primitive::I64 => ValType::I64,
    Primitive::U32 => ValType::I32,
    Primitive::U64 => ValType::I64,
    Primitive::Char => ValType::I32,
    Primitive::Bool => ValType::I32,
    Primitive::Str => ValType::I32,
    Primitive::None => ValType::I32,
  }
}

pub fn operator_to_instruction<'a>(
  op: &Operator,
  ident_type: &IdentType,
) -> Result<Instruction<'a>, CompilerErrorKind> {
  if let IdentType::Primitive(prim) = ident_type {
    match op {
      Operator::Add => match prim {
        Primitive::I32 => Ok(Instruction::I32Add),
        Primitive::U32 => Ok(Instruction::I32Add),
        Primitive::I64 => Ok(Instruction::I64Add),
        Primitive::U64 => Ok(Instruction::I64Add),
        Primitive::F32 => Ok(Instruction::F32Add),
        Primitive::F64 => Ok(Instruction::F64Add),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Sub => match prim {
        Primitive::I32 => Ok(Instruction::I32Sub),
        Primitive::U32 => Ok(Instruction::I32Sub),
        Primitive::I64 => Ok(Instruction::I64Sub),
        Primitive::U64 => Ok(Instruction::I64Sub),
        Primitive::F32 => Ok(Instruction::F32Sub),
        Primitive::F64 => Ok(Instruction::F64Sub),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Mul => match prim {
        Primitive::I32 => Ok(Instruction::I32Mul),
        Primitive::U32 => Ok(Instruction::I32Mul),
        Primitive::I64 => Ok(Instruction::I64Mul),
        Primitive::U64 => Ok(Instruction::I64Mul),
        Primitive::F32 => Ok(Instruction::F32Mul),
        Primitive::F64 => Ok(Instruction::F64Mul),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Div => match prim {
        Primitive::I32 => Ok(Instruction::I32DivS),
        Primitive::U32 => Ok(Instruction::I32DivU),
        Primitive::I64 => Ok(Instruction::I64DivS),
        Primitive::U64 => Ok(Instruction::I64DivU),
        Primitive::F32 => Ok(Instruction::F32Div),
        Primitive::F64 => Ok(Instruction::F64Div),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Mod => match prim {
        Primitive::I32 => Ok(Instruction::I32RemS),
        Primitive::U32 => Ok(Instruction::I32RemU),
        Primitive::I64 => Ok(Instruction::I64RemS),
        Primitive::U64 => Ok(Instruction::I64RemU),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Eq => match prim {
        Primitive::I32 => Ok(Instruction::I32Eq),
        Primitive::U32 => Ok(Instruction::I32Eq),
        Primitive::I64 => Ok(Instruction::I64Eq),
        Primitive::U64 => Ok(Instruction::I64Eq),
        Primitive::F32 => Ok(Instruction::F32Eq),
        Primitive::F64 => Ok(Instruction::F64Eq),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::NotEq => match prim {
        Primitive::I32 => Ok(Instruction::I32Ne),
        Primitive::U32 => Ok(Instruction::I32Ne),
        Primitive::I64 => Ok(Instruction::I64Ne),
        Primitive::U64 => Ok(Instruction::I64Ne),
        Primitive::F32 => Ok(Instruction::F32Ne),
        Primitive::F64 => Ok(Instruction::F64Ne),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::LessThan => match prim {
        Primitive::I32 => Ok(Instruction::I32LeS),
        Primitive::U32 => Ok(Instruction::I32LeU),
        Primitive::I64 => Ok(Instruction::I64LeS),
        Primitive::U64 => Ok(Instruction::I64LeU),
        Primitive::F32 => Ok(Instruction::F32Le),
        Primitive::F64 => Ok(Instruction::F64Le),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::GreaterThan => match prim {
        Primitive::I32 => Ok(Instruction::I32GeS),
        Primitive::U32 => Ok(Instruction::I32GeU),
        Primitive::I64 => Ok(Instruction::I64GeS),
        Primitive::U64 => Ok(Instruction::I64GeU),
        Primitive::F32 => Ok(Instruction::F32Ge),
        Primitive::F64 => Ok(Instruction::F64Ge),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      _ => Err(CompilerErrorKind::UnknownOperator),
    }
  } else {
    Err(CompilerErrorKind::Unimplemented)
  }
}

pub fn operator_to_ident_type(
  op: &Operator,
  lhs: &IdentType,
) -> Result<IdentType, CompilerErrorKind> {
  if let IdentType::Primitive(prim) = lhs {
    match op {
      Operator::Add => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::I32)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::U32)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::I64)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::U64)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::F32)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::F64)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Sub => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::I32)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::U32)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::I64)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::U64)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::F32)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::F64)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Mul => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::I32)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::U32)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::I64)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::U64)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::F32)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::F64)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Div => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::I32)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::U32)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::I64)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::U64)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::F32)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::F64)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Mod => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::I32)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::U32)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::I64)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::U64)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::Eq => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::Bool)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::NotEq => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::Bool)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::LessThan => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::Bool)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      Operator::GreaterThan => match prim {
        Primitive::I32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::I64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::U64 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F32 => Ok(IdentType::Primitive(Primitive::Bool)),
        Primitive::F64 => Ok(IdentType::Primitive(Primitive::Bool)),

        _ => Err(CompilerErrorKind::UnknownOperator),
      },
      _ => Err(CompilerErrorKind::UnknownOperator),
    }
  } else {
    Err(CompilerErrorKind::Unimplemented)
  }
}

pub struct Function<'a> {
  pub ident: String,
  pub instructions: Vec<Instruction<'a>>,
  locals: Vec<(u32, ValType)>,
}

impl<'a> Function<'a> {
  pub fn new(ident: String) -> Self {
    Function {
      ident,
      instructions: Vec::new(),
      locals: Vec::new(),
    }
  }

  pub fn instruction(&mut self, instruction: Instruction<'a>) -> &mut Self {
    self.instructions.push(instruction.clone());
    // println!("{:?}", instruction);
    self
  }

  pub fn local(&mut self, idx: u32, val_type: ValType) -> &mut Self {
    self.locals.push((idx, val_type));
    // println!("{:?}", (idx, val_type));
    self
  }
}

impl From<Function<'_>> for wasm_encoder::Function {
  fn from(fun: Function) -> wasm_encoder::Function {
    let locals: Vec<_> = fun.locals.iter().map(|(_, l)| *l).collect();
    let mut res = wasm_encoder::Function::new_with_locals_types(locals);
    for instruction in fun.instructions {
      res.instruction(&instruction);
    }
    res
  }
}
