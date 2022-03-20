use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::ScopeContainer;

use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
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
  pub idents: Vec<(usize, *mut IdentTyped)>,
}

impl Checker {
  pub fn unify(&mut self, type1: IdentType, type2: IdentType) {
    match (type1.clone(), type2.clone()) {
      (IdentType::Var(i), IdentType::Var(j)) if i == j => {}
      (IdentType::Var(i), _) if self.substitutions[i] != IdentType::Var(i) => {
        self.unify(self.substitutions[i].clone(), type2)
      }
      (_, IdentType::Var(j)) if self.substitutions[j] != IdentType::Var(j) => {
        self.unify(type1, self.substitutions[j].clone())
      }
      (IdentType::Var(i), _) => self.substitutions[i] = type2.clone(),
      (_, IdentType::Var(j)) => self.substitutions[j] = type1.clone(),
      _ => {
        if type1 != type2 {
          self.throw(CompilerErrorKind::TypeMismatch, 0)
        }
      }
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

  pub fn substitute(&mut self, types: IdentType) -> IdentType {
    match types {
      IdentType::Var(i) if self.substitutions[i] != IdentType::Var(i) => self.substitute(types),
      _ => types,
    }
  }
}

pub fn binary_to_type_val(op: &Operator) -> (IdentType, IdentType, IdentType) {
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
    | Operator::BitRightShiftAssign => (
      IdentType::Primitive(Primitive::Int),
      IdentType::Primitive(Primitive::Int),
      IdentType::Primitive(Primitive::Int),
    ),

    Operator::LogAnd | Operator::LogAndAssign | Operator::LogOr | Operator::LogOrAssign => (
      IdentType::Primitive(Primitive::Bool),
      IdentType::Primitive(Primitive::Bool),
      IdentType::Primitive(Primitive::Bool),
    ),

    Operator::Eq | Operator::NotEq => (IdentType::Default, IdentType::Default, IdentType::Default),

    _ => (IdentType::Number, IdentType::Number, IdentType::Number),
  }
}

pub fn unary_to_type_val(op: &Operator) -> (IdentType, IdentType) {
  match op {
    Operator::LogNot => (
      IdentType::Primitive(Primitive::Bool),
      IdentType::Primitive(Primitive::Bool),
    ),

    Operator::BitNot => (
      IdentType::Primitive(Primitive::Int),
      IdentType::Primitive(Primitive::Int),
    ),

    Operator::Sub => (IdentType::Number, IdentType::Number),

    _ => unreachable!(),
  }
}
