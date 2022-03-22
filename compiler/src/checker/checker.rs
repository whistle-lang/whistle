use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::ScopeContainer;

use whistle_ast::IdentType;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primitive;

pub struct Checker {
  pub scope: ScopeContainer,
  pub substitutions: Vec<IdentType>,
  pub constraints: Vec<(IdentType, IdentType)>,
  pub errors: Vec<CompilerError>,

  // this is probably a terrible idea but screw it
  pub literals: Vec<(usize, *mut Literal)>,
  pub idents: Vec<(usize, *mut IdentType)>,
}

impl Checker {
  pub fn new() -> Self {
    Self {
      scope: ScopeContainer::new(),
      substitutions: Vec::new(),
      constraints: Vec::new(),
      errors: Vec::new(),
      literals: Vec::new(),
      idents: Vec::new(),
    }
  }

  pub fn unify(&mut self, type1: IdentType, type2: IdentType) {
    let base1 = self.base_type(type1.clone());
    let base2 = self.base_type(type2.clone());
    if let IdentType::Var(i) = base1 {
      match (self.substitutions[i].clone(), base2.clone()) {
        (IdentType::Array(arr1), IdentType::Array(arr2)) => {
          if let IdentType::Var(j) = *arr1 {
            return self.unify_base(j, *arr2);
          }
        }
        _ => self.unify_base(i, base2),
      }
    } else {
      if let Err(err) = Checker::is_subtype(base2.clone(), base1.clone()) {
        println!("{:?}: Cannot assign {:?} to {:?}", err, base1, base2);
        self.throw(err, 0)
      }
    }
  }

  pub fn unify_base(&mut self, i: usize, base2: IdentType) {
    if let IdentType::Var(j) = base2 {
      match Checker::is_subtype(self.substitutions[j].clone(), self.substitutions[i].clone()) {
        Ok(is_subtype) => {
          if is_subtype {
            self.substitutions[i] = base2.clone()
          } else {
            self.substitutions[j] = self.substitutions[i].clone()
          }
        }
        Err(err) => {
          println!(
            "{:?}: Cannot assign {:?} to {:?}",
            err, self.substitutions[i], self.substitutions[j]
          );
          self.throw(err, 0)
        }
      }
    } else {
      match Checker::is_subtype(base2.clone(), self.substitutions[i].clone()) {
        Ok(is_subtype) => {
          if is_subtype {
            self.substitutions[i] = base2
          }
        }
        Err(err) => {
          println!(
            "{:?}: Cannot assign {:?} to {:?}",
            err, self.substitutions[i], base2
          );
          self.throw(err, 0)
        }
      }
    }
  }

  pub fn coerce(types: IdentType) -> IdentType {
    match types {
      IdentType::Primitive(Primitive::Int) => IdentType::Primitive(Primitive::I32),
      IdentType::Primitive(Primitive::Float) => IdentType::Primitive(Primitive::F64),
      IdentType::Primitive(Primitive::Number) => IdentType::Primitive(Primitive::I32),
      IdentType::Var(_) => IdentType::Error,
      IdentType::Array(arr) => IdentType::Array(Box::new(Checker::coerce(*arr))),
      _ => types,
    }
  }

  pub fn new_type_val(&mut self) -> IdentType {
    let res = IdentType::Var(self.substitutions.len());
    self.substitutions.push(res.clone());
    res
  }

  pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
    self.errors.push(CompilerError::new(error, index))
  }

  pub fn base_type(&self, types: IdentType) -> IdentType {
    if let IdentType::Var(i) = types {
      if let IdentType::Var(j) = self.substitutions[i] {
        if i != j {
          return self.base_type(self.substitutions[i].clone());
        }
      }
    }
    types
  }

  pub fn substitute(&self, types: IdentType) -> IdentType {
    if let IdentType::Var(i) = types {
      if IdentType::Var(i) == self.substitutions[i] {
        return types;
      }
      return self.substitute(self.substitutions[i].clone());
    } else if let IdentType::Array(arr) = types {
      return IdentType::Array(Box::new(self.substitute(*arr)));
    }
    types
  }

  // pub fn substitute(&self, types: IdentType) -> IdentType {
  //   if let IdentType::Var(i) = types {
  //     if let IdentType::Var(j) = self.substitutions[i] {
  //       return self.substitutions[j].clone();
  //     }
  //     return self.substitutions[i].clone();
  //   }
  //   types
  // }
  
  pub fn is_subtype(subtype: IdentType, maintype: IdentType) -> Result<bool, CompilerErrorKind> {
    if let IdentType::Var(_) = subtype {
      return Ok(true);
    }
    if subtype == maintype {
      return Ok(true);
    }
    match maintype {
      IdentType::Primitive(prim) => match prim {
        Primitive::Int => match subtype {
          IdentType::Primitive(Primitive::I32)
          | IdentType::Primitive(Primitive::I64)
          | IdentType::Primitive(Primitive::U32)
          | IdentType::Primitive(Primitive::U64) => Ok(true),
          IdentType::Primitive(Primitive::Number)
          | IdentType::Primitive(Primitive::Int)
          | IdentType::Default => Ok(false),
          _ => Err(CompilerErrorKind::TypeMismatch),
        },
        Primitive::Float => match subtype {
          IdentType::Primitive(Primitive::F32) | IdentType::Primitive(Primitive::F64) => Ok(true),
          IdentType::Primitive(Primitive::Number)
          | IdentType::Primitive(Primitive::Float)
          | IdentType::Default => Ok(false),
          _ => Err(CompilerErrorKind::TypeMismatch),
        },
        Primitive::I32 | Primitive::I64 | Primitive::U32 | Primitive::U64
          if subtype == IdentType::Primitive(Primitive::Int) =>
        {
          Ok(false)
        }
        Primitive::F32 | Primitive::F64 if subtype == IdentType::Primitive(Primitive::Float) => {
          Ok(false)
        }
        _ => Err(CompilerErrorKind::TypeMismatch),
      },
      IdentType::Var(_) => Ok(true),
      _ => Err(CompilerErrorKind::TypeMismatch),
    }
  }
}

pub fn binary_to_type_val(op: &Operator) -> IdentType {
  match op {
    Operator::Mod
    | Operator::ModAssign
    | Operator::BitAnd
    | Operator::BitAndAssign
    | Operator::BitOr
    | Operator::BitOrAssign
    | Operator::BitXor
    | Operator::BitXorAssign
    | Operator::BitLeftShift
    | Operator::BitLeftShiftAssign
    | Operator::BitRightShift
    | Operator::BitRightShiftAssign => IdentType::Primitive(Primitive::Int),

    Operator::LogAnd | Operator::LogAndAssign | Operator::LogOr | Operator::LogOrAssign => {
      IdentType::Primitive(Primitive::Bool)
    }

    Operator::Eq | Operator::NotEq => (IdentType::Default),

    _ => IdentType::Primitive(Primitive::Number),
  }
}

pub fn unary_to_type_val(op: &Operator) -> IdentType {
  match op {
    Operator::LogNot => IdentType::Primitive(Primitive::Bool),

    Operator::BitNot => IdentType::Primitive(Primitive::Int),

    Operator::Sub => IdentType::Primitive(Primitive::Number),

    _ => unreachable!(),
  }
}
