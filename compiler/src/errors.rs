use whistle_ast::Type;
use whistle_common::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerErrorKind {
  ScopeUndefined,
  ScopeNotGlobal,
  ScopeNotFunction,
  ScopeNotInFunction,
  SymbolRedifinition,
  SymbolUndefined,

  ExpectedBooleanExpr,
  ImmutableAssign,
  MissingParameters,
  MissingCallSignature,
  MissingProperty,
  NoImplicitAny,
  NoProperties,
  Unassignable,
  UnknownOperator,

  TypeMismatch {
    type1: Type,
    type2: Type,
  },

  Unimplemented,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerError {
  pub kind: CompilerErrorKind,
  pub range: Range,
}

impl CompilerError {
  pub fn new(kind: CompilerErrorKind, range: Range) -> Self {
    Self { kind, range }
  }
}
