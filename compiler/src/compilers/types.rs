use wasm_encoder::Instruction;
use wasm_encoder::ValType;

use whistle_ast::IdentType;
use whistle_ast::Operator;
use whistle_ast::Primitive;

pub fn ident_type_to_val_type(ident_type: IdentType) -> ValType {
  match ident_type {
    IdentType::Primitive(prim) => prim_to_val_type(prim),
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
    Primitive::Any => panic!(),
  }
}

pub fn ident_type_to_const(prim: Primitive) -> ValType {
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
    Primitive::Any => panic!(),
  }
}

pub fn operator_to_instruction<'a>(op: &Operator, ident_type: &IdentType) -> Instruction<'a> {
  if let IdentType::Primitive(prim) = ident_type {
    match op {
      Operator::Add => match prim {
        Primitive::F32 => Instruction::F32Add,
        Primitive::F64 => Instruction::F64Add,

        Primitive::I32 => Instruction::I32Add,
        Primitive::U32 => Instruction::I32Add,
        Primitive::I64 => Instruction::I64Add,
        Primitive::U64 => Instruction::I64Add,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::Sub => match prim {
        Primitive::F32 => Instruction::F32Sub,
        Primitive::F64 => Instruction::F64Sub,

        Primitive::I32 => Instruction::I32Sub,
        Primitive::U32 => Instruction::I32Sub,
        Primitive::I64 => Instruction::I64Sub,
        Primitive::U64 => Instruction::I64Sub,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::Mul => match prim {
        Primitive::F32 => Instruction::F32Mul,
        Primitive::F64 => Instruction::F64Mul,

        Primitive::I32 => Instruction::I32Mul,
        Primitive::U32 => Instruction::I32Mul,
        Primitive::I64 => Instruction::I64Mul,
        Primitive::U64 => Instruction::I64Mul,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::Div => match prim {
        Primitive::F32 => Instruction::F32Div,
        Primitive::F64 => Instruction::F64Div,

        Primitive::I32 => Instruction::I32DivS,
        Primitive::U32 => Instruction::I32DivU,
        Primitive::I64 => Instruction::I64DivS,
        Primitive::U64 => Instruction::I64DivU,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::Mod => match prim {
        Primitive::I32 => Instruction::I32RemS,
        Primitive::U32 => Instruction::I32RemU,
        Primitive::I64 => Instruction::I64RemS,
        Primitive::U64 => Instruction::I64RemU,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::Eq => match prim {
        Primitive::I32 => Instruction::I32Eq,
        Primitive::U32 => Instruction::I32Eq,
        Primitive::I64 => Instruction::I64Eq,
        Primitive::U64 => Instruction::I64Eq,
        Primitive::F32 => Instruction::F32Eq,
        Primitive::F64 => Instruction::F64Eq,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::NotEq => match prim {
        Primitive::I32 => Instruction::I32Neq,
        Primitive::U32 => Instruction::I32Neq,
        Primitive::I64 => Instruction::I64Neq,
        Primitive::U64 => Instruction::I64Neq,
        Primitive::F32 => Instruction::F32Neq,
        Primitive::F64 => Instruction::F64Neq,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::LessThan => match prim {
        Primitive::I32 => Instruction::I32LeS,
        Primitive::U32 => Instruction::I32LeU,
        Primitive::I64 => Instruction::I64LeS,
        Primitive::U64 => Instruction::I64LeU,
        Primitive::F32 => Instruction::F32Le,
        Primitive::F64 => Instruction::F64Le,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      Operator::GreaterThan => match prim {
        Primitive::I32 => Instruction::I32GeS,
        Primitive::U32 => Instruction::I32GeU,
        Primitive::I64 => Instruction::I64GeS,
        Primitive::U64 => Instruction::I64GeU,
        Primitive::F32 => Instruction::F32Ge,
        Primitive::F64 => Instruction::F64Ge,

        _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
      },
      _ => panic!("Operator {:?} does not exist for type {:?}", op, prim),
    }
  } else {
    panic!("Only primitive types can currently be operated on");
  }
}

pub struct Function<'a> {
  locals: Vec<(u32, ValType)>,
  instructions: Vec<Instruction<'a>>,
}

impl<'a> Function<'a> {
  pub fn new() -> Self {
    Function {
      locals: Vec::new(),
      instructions: Vec::new(),
    }
  }

  pub fn local(&mut self, idx: u32, val_type: ValType) -> &mut Self {
    self.locals.push((idx, val_type));
    self
  }

  pub fn instruction(&mut self, instruction: Instruction<'a>) -> &mut Self {
    self.instructions.push(instruction);
    self
  }
}

impl Default for Function<'_> {
  fn default() -> Self {
    Function::new()
  }
}

impl From<Function<'_>> for wasm_encoder::Function {
  fn from(fun: Function) -> wasm_encoder::Function {
    let mut res = wasm_encoder::Function::new(fun.locals);
    for instruction in fun.instructions {
      res.instruction(instruction);
    }
    res
  }
}
