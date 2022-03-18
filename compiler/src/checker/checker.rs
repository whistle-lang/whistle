use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::ScopeContainer;

use whistle_ast::IdentType;

#[derive(PartialEq, Clone)]
pub enum TypeVal {
  Var(usize),
  Ident(IdentType),
}

pub struct Checker {
  pub scopes: ScopeContainer,
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
