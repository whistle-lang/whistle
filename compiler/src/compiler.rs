use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::Memory;
use crate::Module;
use crate::ScopeContainer;

use whistle_common::Range;

pub struct Compiler {
  pub errors: Vec<CompilerError>,
  pub scope: ScopeContainer,
  pub module: Module,
  pub memory: Memory,
}

impl Compiler {
  pub fn new() -> Self {
    Compiler {
      errors: Vec::new(),
      scope: ScopeContainer::new(),
      module: Module::new(),
      memory: Memory::new(),
    }
  }

  pub fn throw(&mut self, error: CompilerErrorKind, range: Range) {
    self.errors.push(CompilerError::new(error, range))
  }
}

impl Default for Compiler {
  fn default() -> Self {
    Self::new()
  }
}
