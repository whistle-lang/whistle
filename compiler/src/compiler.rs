use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::Context;
use crate::Scope;
use crate::SymbolTable;
use crate::Var;

use whistle_ast::IdentType;
use whistle_ast::Primitive;

pub struct Compiler {
  pub scopes: Vec<Scope>,
  pub table: SymbolTable,
  pub errors: Vec<CompilerError>,
  pub context: Context,
}

impl Compiler {
  pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
    self.errors.push(CompilerError::new(error, index))
  }

  pub fn scope(&mut self) -> &mut Scope {
    let len = self.scopes.len();
    &mut self.scopes[len]
  }

  pub fn enter_scope(&mut self) -> &mut Scope {
    let scope = Scope::new();
    self.scopes.push(scope);
    self.scope()
  }

  pub fn exit_scope(&mut self) {
    let len = self.scopes.len();
    self.scopes[len] = self.scopes.remove(self.scopes.len());
  }

  pub fn set_var(&mut self, ident: String, var: Var) -> Option<Var> {
    if self.scope().vars.get(&ident).is_some() {
      self.throw(CompilerErrorKind::VarUndefined, 0);
      None
    } else {
      self.scope().vars.insert(ident, var.clone());
      Some(var)
    }
  }

  pub fn get_var(&mut self, ident: String) -> Option<Var> {
    for scope in &self.scopes {
      if let Some(var) = scope.vars.get(&ident) {
        return Some(var.clone());
      }
    }
    self.throw(CompilerErrorKind::VarUndefined, 0);
    None
  }

  pub fn no_implicit_any(&mut self, types: IdentType) -> IdentType {
    if IdentType::Primitive(Primitive::Any) == types {
      self.throw(CompilerErrorKind::NoImplicitAny, 0);
      IdentType::Primitive(Primitive::None)
    } else {
      types
    }
  }
}
