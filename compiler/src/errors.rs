#[derive(Debug, Clone, PartialEq)]
pub enum CompilerErrorKind {
  VarRedefinition,
	FuncRedefinition,
  TypeRedefinition,

  ExpectedParamType,
  IncompatibleTypes,

  VarUndefined,
  TypeUndefined,
  FuncUndefined,

  NoCallSignatures,
  NoProperty,
  MissingParameters
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