use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::Module;
use crate::ScopeContainer;

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
}

impl Default for Compiler {
  fn default() -> Self {
    Self::new()
  }
}
