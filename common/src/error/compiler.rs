use crate::DiagnosticHandler;
use crate::Error;
use crate::Span;
use crate::Type;

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

  TypeMismatch { type1: Type, type2: Type },

  Unimplemented,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerError {
  pub kind: CompilerErrorKind,
  pub span: Span,
}

impl CompilerError {
  pub fn new(kind: CompilerErrorKind, span: Span) -> Self {
    Self { kind, span }
  }
}

pub trait CompilerHandler {
  fn throw(&mut self, kind: CompilerErrorKind, span: Span);
}

impl CompilerHandler for DiagnosticHandler {
  fn throw(&mut self, kind: CompilerErrorKind, span: Span) {
    self
      .errors
      .push(Error::CompilerError(CompilerError { kind, span }))
  }
}
