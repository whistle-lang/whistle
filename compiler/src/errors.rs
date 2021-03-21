#[derive(Debug, Clone, PartialEq)]
pub enum CompilerErrorKind {
  ScopeUndefined,
  ScopeNotGlobal,
  ScopeNotFunction,
  ScopeNotInFunction,
  SymbolRedifinition,
  SymbolUndefined,

  ExpectedBooleanExpr,
  TypeMismatch,
  ImmutableAssign,
  MissingParameters,
  MissingCallSignature,
  MissingProperty,
  NoImplicitAny,
  NoProperties,
  Unassignable,
  UnknownOperator,

  Unimplemented,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerError {
  pub kind: CompilerErrorKind,
  pub index: usize,
}

impl CompilerError {
  pub fn new(kind: CompilerErrorKind, index: usize) -> Self {
    Self { kind, index }
  }
}
