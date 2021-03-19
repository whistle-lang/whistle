use std::collections::HashMap;
use whistle_ast::IdentType;

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeError {
  ScopeUndefined,
  ScopeNotGlobal,
  ScopeNotFunction,
  GlobalParentScope,
  SymbolRedifinition,
  SymbolUndefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
  pub global: bool,
  pub mutable: bool,
  pub types: IdentType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexedSymbol(pub u32, pub Symbol);

pub enum Scope {
  Global {
    fun_idx: u32,
    global_idx: u32,
    symbols: HashMap<String, IndexedSymbol>,
  },
  Function {
    global: usize,
    local_idx: u32,
    symbols: HashMap<String, IndexedSymbol>,
  },
  Block {
    parent: usize,
    symbols: HashMap<String, IndexedSymbol>,
  },
}

pub struct ScopeContainer {
  pub scopes: Vec<Scope>,
  pub curr: usize,
}

impl ScopeContainer {
  pub fn new() -> Self {
    ScopeContainer {
      scopes: Vec::new(),
      curr: 0,
    }
  }

  pub fn get_scope(&self, id: usize) -> Option<&Scope> {
    self.scopes.get(id)
  }

  pub fn get_scope_mut(&mut self, id: usize) -> Option<&mut Scope> {
    self.scopes.get_mut(id)
  }

  pub fn curr_scope(&self) -> Option<&Scope> {
    self.scopes.get(self.curr)
  }

  pub fn curr_scope_mut(&mut self) -> Option<&mut Scope> {
    self.scopes.get_mut(self.curr)
  }

  pub fn enter_scope(&mut self) -> &Scope {
    let scope = match self.curr_scope() {
      None => Scope::Global {
        fun_idx: 0,
        global_idx: 0,
        symbols: HashMap::new(),
      },
      Some(Scope::Global { .. }) => Scope::Function {
        global: self.curr,
        local_idx: 0,
        symbols: HashMap::new(),
      },
      Some(Scope::Function { .. }) | Some(Scope::Block { .. }) => Scope::Block {
        parent: self.curr,
        symbols: HashMap::new(),
      },
    };

    self.scopes.push(scope);
    self.curr = self.scopes.len() - 1;

    &self.scopes[self.curr]
  }

  pub fn enter_scope_mut(&mut self) -> &mut Scope {
    let scope = match self.curr_scope() {
      None => Scope::Global {
        fun_idx: 0,
        global_idx: 0,
        symbols: HashMap::new(),
      },
      Some(Scope::Global { .. }) => Scope::Function {
        global: self.curr,
        local_idx: 0,
        symbols: HashMap::new(),
      },
      Some(Scope::Function { .. }) | Some(Scope::Block { .. }) => Scope::Block {
        parent: self.curr,
        symbols: HashMap::new(),
      },
    };

    self.scopes.push(scope);
    self.curr = self.scopes.len() - 1;

    &mut self.scopes[self.curr]
  }

  pub fn exit_scope(&mut self) -> Option<&Scope> {
    match self.curr_scope() {
      Some(Scope::Function { global, .. }) => {
        self.curr = *global;
        self.curr_scope()
      }
      Some(Scope::Block { parent, .. }) => {
        self.curr = *parent;
        self.curr_scope()
      }
      _ => None,
    }
  }

  pub fn exit_scope_mut(&mut self) -> Option<&mut Scope> {
    match self.curr_scope() {
      Some(Scope::Function { global, .. }) => {
        self.curr = *global;
        self.curr_scope_mut()
      }
      Some(Scope::Block { parent, .. }) => {
        self.curr = *parent;
        self.curr_scope_mut()
      }
      _ => None,
    }
  }

  pub fn fun_scope_of(&self, id: usize) -> Result<&Scope, ScopeError> {
    let scope = self.get_scope(id).ok_or(ScopeError::ScopeUndefined)?;

    match scope {
      Scope::Global { .. } => Err(ScopeError::GlobalParentScope),
      Scope::Block { parent, .. } => self.fun_scope_of(*parent),
      Scope::Function { .. } => Ok(scope),
    }
  }

  pub fn curr_fun_scope(&self) -> Result<&Scope, ScopeError> {
    self.fun_scope_of(self.curr)
  }

  #[allow(mutable_borrow_reservation_conflict)]
  pub fn fun_scope_of_mut(&mut self, id: usize) -> Result<&mut Scope, ScopeError> {
    let scope = self.get_scope(id).ok_or(ScopeError::ScopeUndefined)?;

    match scope {
      Scope::Global { .. } => Err(ScopeError::GlobalParentScope),
      Scope::Block { parent, .. } => self.fun_scope_of_mut(*parent),
      Scope::Function { .. } => Ok(self.get_scope_mut(id).ok_or(ScopeError::ScopeUndefined)?),
    }
  }

  pub fn curr_fun_scope_mut(&mut self) -> Result<&mut Scope, ScopeError> {
    self.fun_scope_of_mut(self.curr)
  }

  pub fn global_scope_of(&self, id: usize) -> Result<&Scope, ScopeError> {
    if let Scope::Function { global, .. } = self.fun_scope_of(id)? {
      Ok(self.get_scope(*global).ok_or(ScopeError::ScopeUndefined)?)
    } else {
      Err(ScopeError::ScopeNotFunction)
    }
  }

  #[allow(mutable_borrow_reservation_conflict)]
  pub fn global_scope_of_mut(&mut self, id: usize) -> Result<&mut Scope, ScopeError> {
    if let Scope::Function { global, .. } = self.fun_scope_of(id)? {
      Ok(
        self
          .get_scope_mut(*global)
          .ok_or(ScopeError::ScopeUndefined)?,
      )
    } else {
      Err(ScopeError::ScopeNotFunction)
    }
  }

  pub fn set_sym_of(
    &mut self,
    id: usize,
    ident: &str,
    sym: IndexedSymbol,
  ) -> Result<(), ScopeError> {
    match self.get_scope_mut(id).ok_or(ScopeError::ScopeUndefined)? {
      Scope::Global { symbols, .. }
      | Scope::Function { symbols, .. }
      | Scope::Block { symbols, .. } => {
        if symbols.contains_key(ident) {
          Err(ScopeError::SymbolRedifinition)
        } else {
          symbols.insert(ident.to_string(), sym);
          Ok(())
        }
      }
    }
  }

  pub fn set_sym(&mut self, ident: &str, sym: IndexedSymbol) -> Result<(), ScopeError> {
    self.set_sym_of(self.curr, ident, sym)
  }

  pub fn get_sym_of(&self, id: usize, ident: &str) -> Result<&IndexedSymbol, ScopeError> {
    match self.get_scope(id).ok_or(ScopeError::ScopeUndefined)? {
      Scope::Global { symbols, .. } => symbols.get(ident).ok_or(ScopeError::SymbolUndefined),
      Scope::Function {
        symbols, global, ..
      } => {
        if let Some(sym) = symbols.get(ident) {
          Ok(sym)
        } else {
          self.get_sym_of(*global, ident)
        }
      }
      Scope::Block {
        symbols, parent, ..
      } => {
        if let Some(sym) = symbols.get(ident) {
          Ok(sym)
        } else {
          self.get_sym_of(*parent, ident)
        }
      }
    }
  }

  pub fn get_sym(&self, ident: &str) -> Result<&IndexedSymbol, ScopeError> {
    self.get_sym_of(self.curr, ident)
  }

  pub fn set_global_sym_of(
    &mut self,
    id: usize,
    ident: &str,
    sym: Symbol,
  ) -> Result<u32, ScopeError> {
    match self.get_scope_mut(id) {
      Some(Scope::Global { global_idx, .. }) => {
        let idx = *global_idx;
        *global_idx += 1;

        self.set_sym_of(id, ident, IndexedSymbol(idx, sym))?;
        Ok(idx)
      }
      _ => Err(ScopeError::ScopeNotGlobal),
    }
  }

  pub fn set_global_sym(&mut self, ident: &str, sym: Symbol) -> Result<u32, ScopeError> {
    self.set_global_sym_of(self.curr, ident, sym)
  }

  pub fn set_fun_sym_of(&mut self, id: usize, ident: &str, sym: Symbol) -> Result<u32, ScopeError> {
    match self.get_scope_mut(id) {
      Some(Scope::Global { fun_idx, .. }) => {
        let idx = *fun_idx;
        *fun_idx += 1;

        self.set_sym_of(id, ident, IndexedSymbol(idx, sym))?;
        Ok(idx)
      }
      _ => Err(ScopeError::ScopeNotGlobal),
    }
  }

  pub fn set_fun_sym(&mut self, ident: &str, sym: Symbol) -> Result<u32, ScopeError> {
    self.set_fun_sym_of(self.curr, ident, sym)
  }

  pub fn set_local_sym_of(
    &mut self,
    id: usize,
    ident: &str,
    sym: Symbol,
  ) -> Result<u32, ScopeError> {
    if let Scope::Function { local_idx, .. } = self.fun_scope_of_mut(id)? {
      let idx = *local_idx;
      *local_idx += 1;

      self.set_sym_of(id, ident, IndexedSymbol(idx, sym))?;
      Ok(idx)
    } else {
      Err(ScopeError::ScopeNotFunction)
    }
  }
  pub fn set_local_sym(&mut self, ident: &str, sym: Symbol) -> Result<u32, ScopeError> {
    self.set_local_sym_of(self.curr, ident, sym)
  }
}

impl Default for ScopeContainer {
  fn default() -> Self {
    Self::new()
  }
}
