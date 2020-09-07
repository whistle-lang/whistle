use super::TokenPos;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
  ExpectedCommentInline,
  ExpectedCommentLine,
  ExpectedIdentOrKeyword,
  ExpectedTipIdent,
  ExpectedBoolLit,
  ExpectedNoneLit,
  ExpectedFloatLit,
  ExpectedIntLit,
  ExpectedStringStartDelim,
  ExpectedStringInner,
  ExpectedStringEndDelim,
  ExpectedCharStartDelim,
  ExpectedCharInner,
  ExpectedCharEndDelim,
  ExpectedDec,
  ExpectedBin,
  ExpectedOct,
  ExpectedHex,
  ExpectedExp,
  ExpectedDecOrExp,
  ExpectedOperator,
  ExpectedHash,
  ExpectedLeftParen,
  ExpectedRightParen,
  ExpectedNewline,
  ExpectedPunc,
  UnexpectedEOF,
  NoMatch,
  CouldNotParseFloat,
  EOF,
}

impl ErrorKind {
  pub fn is_terminable(&self) -> bool {
    match self {
      ErrorKind::ExpectedTipIdent => true,
      ErrorKind::ExpectedStringInner => true,
      ErrorKind::ExpectedStringEndDelim => true,
      ErrorKind::ExpectedCharInner => true,
      ErrorKind::ExpectedCharEndDelim => true,
      ErrorKind::ExpectedDec => true,
      ErrorKind::ExpectedBin => true,
      ErrorKind::ExpectedOct => true,
      ErrorKind::ExpectedHex => true,
      ErrorKind::ExpectedExp => true,
      ErrorKind::ExpectedLeftParen => true,
      ErrorKind::ExpectedRightParen => true,
      ErrorKind::ExpectedNewline => true,
      ErrorKind::UnexpectedEOF => true,
      ErrorKind::NoMatch => true,
      ErrorKind::CouldNotParseFloat => true,
      _ => false,
    }
  }
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
