#[derive(Debug, Clone, PartialEq)]
pub enum CompilerErrorKind {
  VarRedefinition,
  FuncRedefinition,
  TypeRedefinition,

  NoImplicitAny,
  IncompatibleTypes,

  VarUndefined,
  TypeUndefined,
  FuncUndefined,
  ImmutableAssign,

  NoCallSignatures,
  NoProperty,
  MissingParameters,

  ExpectedBooleanExpr,
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
