use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::Module;
use crate::ScopeContainer;

use whistle_ast::IdentType;
use whistle_ast::Primitive;

pub struct Compiler {
  pub errors: Vec<CompilerError>,
  pub scope: ScopeContainer,
  pub module: Module,
}

impl Compiler {
  pub fn new() -> Self {
    Compiler {
      errors: Vec::new(),
      scope: ScopeContainer::new(),
      module: Module::new(),
    }
  }

  pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
    self.errors.push(CompilerError::new(error, index))
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

impl Default for Compiler {
  fn default() -> Self {
    Self::new()
  }
}
