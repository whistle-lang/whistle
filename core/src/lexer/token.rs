#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  // Ignored by default:
  CommentLine,
  CommentInline,
  // Rest:
  Ident,
  Keyword,
  Operator,
  FloatLit,
  IntLit,
  StringLit,
  CharLit,
  BoolLit,
  NoneLit,
  Tip,
  Punc
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<T> {
  pub kind: TokenKind,
  pub value: T,
  pub index: usize,
}

impl<T> Token<T> {
  pub fn new(kind: TokenKind, value: T, index: usize) -> Self {
    Self { kind, value, index }
  }
}
