use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::ScopeContainer;

use whistle_ast::IdentType;
use whistle_ast::Literal;
use whistle_ast::Operator;
use whistle_ast::Primitive;
use whistle_ast::Type;

pub struct Checker {
  pub scope: ScopeContainer,
  pub substitutions: Vec<Type>,
  pub constraints: Vec<(Type, Type)>,
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

  pub fn unify(&mut self, type1: Type, type2: Type) {
    println!("Constraint {:?}, {:?}", type1, type2);
    let base1 = self.base_type(type1);
    let base2 = self.base_type(type2);
    println!("Base {:?}, {:?}", base1, base2);
    if let Type::Var(i) = base1 {
      match (self.substitutions[i].clone(), base2.clone()) {
        (Type::Array(arr1), Type::Array(arr2)) => {
          if let Type::Var(j) = *arr1 {
            self.unify_base(j, *arr2)
          }
        }
        _ => self.unify_base(i, base2),
      }
    } else if let Err(err) = Checker::is_subtype(base2.clone(), base1.clone()) {
      println!(
        "{:?}: Cannot assign {:?} to {:?}",
        err,
        self.substitute(base1),
        self.substitute(base2)
      );
      self.throw(err, 0)
    }
    println!("{:?}\n", self.substitutions);
  }

  pub fn unify_base(&mut self, i: usize, base2: Type) {
    if let Type::Var(j) = base2 {
      match Checker::is_subtype(self.substitutions[j].clone(), self.substitutions[i].clone()) {
        Ok(is_subtype) => {
          if is_subtype {
            self.substitutions[i] = base2
          } else {
            self.substitutions[j] = self.substitutions[i].clone()
          }
        }
        Err(err) => {
          println!(
            "{:?}: Cannot assign {:?} to {:?}",
            err,
            self.substitute(self.substitutions[j].clone()),
            self.substitute(self.substitutions[i].clone())
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
            err,
            self.substitute(self.substitutions[i].clone()),
            self.substitute(base2.clone())
          );
          self.throw(err, 0)
        }
      }
    }
  }

  pub fn coerce(types: Type) -> Type {
    match types {
      Type::Primitive(Primitive::Int) => Type::Primitive(Primitive::I32),
      Type::Primitive(Primitive::Float) => Type::Primitive(Primitive::F64),
      Type::Primitive(Primitive::Number) => Type::Primitive(Primitive::I32),
      Type::Var(_) => Type::Error,
      Type::Array(arr) => Type::Array(Box::new(Checker::coerce(*arr))),
      _ => types,
    }
  }

  pub fn new_type_val(&mut self) -> Type {
    let res = Type::Var(self.substitutions.len());
    self.substitutions.push(res.clone());
    res
  }

  pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
    self.errors.push(CompilerError::new(error, index))
  }

  pub fn base_type(&self, types: Type) -> Type {
    if let Type::Var(i) = types {
      if let Type::Var(j) = self.substitutions[i] {
        if i != j {
          return self.base_type(self.substitutions[i].clone());
        }
      }
    }
    types
  }

  pub fn substitute(&self, types: Type) -> Type {
    if let Type::Var(i) = types {
      if Type::Var(i) == self.substitutions[i] {
        return types;
      }
      return self.substitute(self.substitutions[i].clone());
    } else if let Type::Array(arr) = types {
      return Type::Array(Box::new(self.substitute(*arr)));
    }
    types
  }

  // pub fn substitute(&self, types: Type) -> Type {
  //   if let Type::Var(i) = types {
  //     if let Type::Var(j) = self.substitutions[i] {
  //       return self.substitutions[j].clone();
  //     }
  //     return self.substitutions[i].clone();
  //   }
  //   types
  // }

  pub fn is_subtype(type1: Type, type2: Type) -> Result<bool, CompilerErrorKind> {
    if let Type::Var(_) = type1 {
      return Ok(true);
    }

    if type1 == type2 {
      return Ok(true);
    }

    match type2 {
      Type::Primitive(prim) => match prim {
        Primitive::Number => match type1 {
          Type::Primitive(Primitive::I32)
          | Type::Primitive(Primitive::I64)
          | Type::Primitive(Primitive::U32)
          | Type::Primitive(Primitive::U64)
          | Type::Primitive(Primitive::F32)
          | Type::Primitive(Primitive::F64)
          | Type::Primitive(Primitive::Int)
          | Type::Primitive(Primitive::Float) => Ok(true),
          Type::Default => Ok(false),
          _ => Err(CompilerErrorKind::TypeMismatch),
        },
        Primitive::Int => match type1 {
          Type::Primitive(Primitive::I32)
          | Type::Primitive(Primitive::I64)
          | Type::Primitive(Primitive::U32)
          | Type::Primitive(Primitive::U64) => Ok(true),
          Type::Primitive(Primitive::Number) | Type::Primitive(Primitive::Int) | Type::Default => {
            Ok(false)
          }
          _ => Err(CompilerErrorKind::TypeMismatch),
        },
        Primitive::Float => match type1 {
          Type::Primitive(Primitive::F32) | Type::Primitive(Primitive::F64) => Ok(true),
          Type::Primitive(Primitive::Number)
          | Type::Primitive(Primitive::Float)
          | Type::Default => Ok(false),
          _ => Err(CompilerErrorKind::TypeMismatch),
        },
        Primitive::I32
        | Primitive::I64
        | Primitive::U32
        | Primitive::U64
        | Primitive::F32
        | Primitive::F64
          if type1 == Type::Primitive(Primitive::Number) =>
        {
          Ok(false)
        }
        Primitive::I32 | Primitive::I64 | Primitive::U32 | Primitive::U64
          if type1 == Type::Primitive(Primitive::Int) =>
        {
          Ok(false)
        }
        Primitive::F32 | Primitive::F64 if type1 == Type::Primitive(Primitive::Float) => Ok(false),
        _ => Err(CompilerErrorKind::TypeMismatch),
      },
      Type::Var(_) => Ok(true),
      _ => Err(CompilerErrorKind::TypeMismatch),
    }
  }
}

impl Default for Checker {
  fn default() -> Self {
    Self::new()
  }
}

pub fn binary_to_type_val(op: &Operator) -> Type {
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
    | Operator::BitRightShiftAssign => Type::Primitive(Primitive::Int),

    Operator::LogAnd | Operator::LogAndAssign | Operator::LogOr | Operator::LogOrAssign => {
      Type::Primitive(Primitive::Bool)
    }

    Operator::Eq | Operator::NotEq => Type::Default,

    _ => Type::Primitive(Primitive::Number),
  }
}

pub fn unary_to_type_val(op: &Operator) -> Type {
  match op {
    Operator::LogNot => Type::Primitive(Primitive::Bool),

    Operator::BitNot => Type::Primitive(Primitive::Int),

    Operator::Sub => Type::Primitive(Primitive::Number),

    _ => unreachable!(),
  }
}
