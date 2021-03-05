use std::collections::HashMap;
use whistle_ast::IdentType;

#[derive(Debug, Clone)]
pub struct SymbolTable {
  pub scopes: Vec<Scope>,
  pub types: HashMap<String, IdentType>,
  pub vars: HashMap<String, Var>,
}

#[derive(Debug, Clone)]
pub struct Scope {
  pub scopes: Vec<Scope>,
  pub vars: HashMap<String, Var>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
  pub mutable: bool,
  pub types: IdentType,
}

impl Scope {
  pub fn new() -> Self {
    Self {
      scopes: Vec::new(),
      vars: HashMap::new(),
    }
  }
}

impl Default for Scope {
  fn default() -> Self {
    Scope::new()
  }
}

impl SymbolTable {
  pub fn new() -> Self {
    Self {
      scopes: Vec::new(),
      types: HashMap::new(),
      vars: HashMap::new(),
    }
  }
}

impl Default for SymbolTable {
  fn default() -> Self {
    SymbolTable::new()
  }
}
