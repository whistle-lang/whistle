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
  UnexpectedEof,
  NoMatch,
  CouldNotParseFloat,
  Eof,
}

impl LexerErrorKind {
  pub fn is_terminable(&self) -> bool {
    matches!(
      self,
      LexerErrorKind::ExpectedTipIdent
        | LexerErrorKind::ExpectedStringInner
        | LexerErrorKind::ExpectedStringEndDelim
        | LexerErrorKind::ExpectedCharInner
        | LexerErrorKind::ExpectedCharEndDelim
        | LexerErrorKind::ExpectedDec
        | LexerErrorKind::ExpectedBin
        | LexerErrorKind::ExpectedOct
        | LexerErrorKind::ExpectedHex
        | LexerErrorKind::ExpectedExp
        | LexerErrorKind::ExpectedLeftParen
        | LexerErrorKind::ExpectedRightParen
        | LexerErrorKind::ExpectedNewline
        | LexerErrorKind::UnexpectedEof
        | LexerErrorKind::NoMatch
        | LexerErrorKind::CouldNotParseFloat
    )
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
