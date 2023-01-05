use crate::Checker;
use crate::IndexedSymbol;
use crate::Memory;
use crate::Module;
use crate::ScopeContainer;
use whistle_common::CompilerErrorKind;

use whistle_ast::Type;
use whistle_common::DiagnosticHandler;

pub struct Compiler {
  pub handler: DiagnosticHandler,
  pub scope: ScopeContainer,
  pub module: Module,
  pub memory: Memory,
  pub substitutions: Vec<Type>,
}

impl Compiler {
  pub fn new(checker: Checker) -> Self {
    Compiler {
      handler: checker.handler,
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
}
