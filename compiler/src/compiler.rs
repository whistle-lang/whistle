use crate::Checker;
use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::IndexedSymbol;
use crate::Memory;
use crate::Module;
use crate::ScopeContainer;

use whistle_ast::Span;
use whistle_ast::Type;

pub struct Compiler {
  pub errors: Vec<CompilerError>,
  pub scope: ScopeContainer,
  pub module: Module,
  pub memory: Memory,
  pub substitutions: Vec<Type>,
}

impl Compiler {
  pub fn new(checker: Checker) -> Self {
    Compiler {
      errors: Vec::new(),
      scope: checker.scope,
      module: Module::new(),
      memory: Memory::new(),
      substitutions: checker.substitutions,
    }
  }

  pub fn query_type(&self, types: Type) -> Type {
    if let Type::Var(i) = types {
      return self.substitutions[i].clone();
    }
    types
  }

  pub fn get_sym(&self, ident: &str) -> Result<IndexedSymbol, CompilerErrorKind> {
    let mut sym = self.scope.get_sym(ident)?.clone();
    sym.1.types = self.query_type(sym.1.types);
    Ok(sym)
  }

  pub fn throw(&mut self, error: CompilerErrorKind, span: Span) {
    self.errors.push(CompilerError::new(error, span))
  }
}
