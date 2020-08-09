use super::TokenPos;

#[derive(Kind, Debug, Clone, PartialEq)]
#[kind(function(terminable = "bool"))]
pub enum ErrorKind {
  ExpectedCommentInline,
  ExpectedCommentLine,
  ExpectedIdentOrKeyword,
  #[kind(terminable)]
  ExpectedTipIdent,
  ExpectedBoolLit,
  ExpectedNoneLit,
  ExpectedFloatLit,
  ExpectedIntLit,
  ExpectedStringStartDelim,
  #[kind(terminable)]
  ExpectedStringInner,
  #[kind(terminable)]
  ExpectedStringEndDelim,
  ExpectedCharStartDelim,
  #[kind(terminable)]
  ExpectedCharInner,
  #[kind(terminable)]
  ExpectedCharEndDelim,
  #[kind(terminable)]
  ExpectedDec,
  #[kind(terminable)]
  ExpectedBin,
  #[kind(terminable)]
  ExpectedOct,
  #[kind(terminable)]
  ExpectedHex,
  #[kind(terminable)]
  ExpectedExp,
  ExpectedDecOrExp,
  ExpectedOperator,
  ExpectedHash,
  #[kind(terminable)]
  ExpectedLeftParen,
  #[kind(terminable)]
  ExpectedRightParen,
  #[kind(terminable)]
  ExpectedNewline,
  ExpectedPunc,
  #[kind(terminable)]
  UnexpectedEOF,
  #[kind(terminable)]
  NoMatch,
  #[kind(terminable)]
  CouldNotParseFloat,
  EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
  pub kind: ErrorKind,
  pub pos: TokenPos,
}

impl LexerError {
  pub fn new(kind: ErrorKind, pos: TokenPos) -> Self {
    Self { kind, pos }
  }
}
