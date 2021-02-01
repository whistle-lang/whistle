use whistle_common::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKind {
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

impl LexerErrorKind {
  pub fn is_terminable(&self) -> bool {
    match self {
      LexerErrorKind::ExpectedTipIdent => true,
      LexerErrorKind::ExpectedStringInner => true,
      LexerErrorKind::ExpectedStringEndDelim => true,
      LexerErrorKind::ExpectedCharInner => true,
      LexerErrorKind::ExpectedCharEndDelim => true,
      LexerErrorKind::ExpectedDec => true,
      LexerErrorKind::ExpectedBin => true,
      LexerErrorKind::ExpectedOct => true,
      LexerErrorKind::ExpectedHex => true,
      LexerErrorKind::ExpectedExp => true,
      LexerErrorKind::ExpectedLeftParen => true,
      LexerErrorKind::ExpectedRightParen => true,
      LexerErrorKind::ExpectedNewline => true,
      LexerErrorKind::UnexpectedEOF => true,
      LexerErrorKind::NoMatch => true,
      LexerErrorKind::CouldNotParseFloat => true,
      _ => false,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
  pub kind: LexerErrorKind,
  pub range: Range,
}

impl LexerError {
  pub fn new(kind: LexerErrorKind, range: Range) -> Self {
    Self { kind, range }
  }
}
