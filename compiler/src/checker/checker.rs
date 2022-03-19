use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::ScopeContainer;

use whistle_ast::IdentType;
use whistle_ast::Operator;
use whistle_ast::Primitive;

pub enum Type {
  
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeVal {
  Var(usize),
  Ident(IdentType),
}

pub struct Checker {
  pub scope: ScopeContainer,
  pub substitutions: Vec<TypeVal>,
  pub constraints: Vec<(TypeVal, TypeVal)>,
  pub errors: Vec<CompilerError>,
}

impl Checker {
  pub fn unify(&mut self, type1: TypeVal, type2: TypeVal) {
    match (type1.clone(), type2.clone()) {
      (TypeVal::Ident(type1), TypeVal::Ident(type2)) => {
        if type1 != type2 {
          self.throw(CompilerErrorKind::TypeMismatch, 0)
        }
      }
      (TypeVal::Var(i), TypeVal::Var(j)) if i == j => {}
      (TypeVal::Var(i), _) if self.substitutions[i] != TypeVal::Var(i) => {
        self.unify(self.substitutions[i].clone(), type2)
      }
      (_, TypeVal::Var(j)) if self.substitutions[j] != TypeVal::Var(j) => {
        self.unify(type1, self.substitutions[j].clone())
      }
      (TypeVal::Var(i), _) => self.substitutions[i] = type2.clone(),
      (_, TypeVal::Var(j)) => self.substitutions[j] = type1.clone(),
    }
  }

  pub fn new_type_val(&mut self) -> TypeVal {
    let res = TypeVal::Var(self.substitutions.len());
    self.substitutions.push(res.clone());
    res
  }

  pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
    self.errors.push(CompilerError::new(error, index))
  }
}

pub fn binary_to_type_val(op: &Operator) -> (TypeVal, TypeVal, TypeVal) {
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
      TypeVal::Ident(IdentType::Int),
      TypeVal::Ident(IdentType::Int),
      TypeVal::Ident(IdentType::Int),
    ),

    Operator::LogAnd
    | Operator::LogAndAssign
    | Operator::LogOr
    | Operator::LogOrAssign => (
      TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
      TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
      TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
    ),

    Operator::Eq | Operator::NotEq => (
      TypeVal::Ident(IdentType::Default),
      TypeVal::Ident(IdentType::Default),
      TypeVal::Ident(IdentType::Default),
    ),

    _ => (
      TypeVal::Ident(IdentType::Number),
      TypeVal::Ident(IdentType::Number),
      TypeVal::Ident(IdentType::Number),
    ),
  }
}

pub fn unary_to_type_val(op: &Operator) -> (TypeVal, TypeVal) {
  match op {
    Operator::LogNot => (
      TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
      TypeVal::Ident(IdentType::Primitive(Primitive::Bool)),
    ),

    Operator::BitNot => (
      TypeVal::Ident(IdentType::Int),
      TypeVal::Ident(IdentType::Int),
    ),

    Operator::Sub => (
      TypeVal::Ident(IdentType::Number),
      TypeVal::Ident(IdentType::Number),
    ),

    _ => unreachable!(),
  }
}

