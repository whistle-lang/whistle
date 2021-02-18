use whistle_ast::IdentType;
use std::collections::HashMap;
use whistle_ast::Var;

pub struct SymbolTable {
  pub scopes: Vec<Scope>,
  pub types: HashMap<String, IdentType>,
  pub vars: HashMap<String, Var>,
}

pub struct Scope {
  pub scopes: Vec<Scope>,
  pub vars: HashMap<String, Var>,
  pub parent: Box<Option<Scope>>,
}

pub trait Node {
  fn parent(self) -> Scope;
}

impl Scope {
  pub fn new(parent: Option<Scope>) -> Self {
    Self {
      scopes: Vec::new(),
      vars: HashMap::new(),
      parent: Box::new(parent)
    }
  }

  pub fn get_var(self, ident: String) -> Option<Var> {
    for (var, val) in self.vars {
      if var == ident {
        return Some(val)
      }
    }
    if let Some(parent) = *self.parent {
      return parent.get_var(ident)
    }
    None
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


