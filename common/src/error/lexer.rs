use crate::DiagnosticHandler;
use crate::Error;
use crate::Span;

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
  pub span: Span,
}

impl LexerError {
  pub fn new(kind: LexerErrorKind, span: Span) -> Self {
    Self { kind, span }
  }
}

pub trait LexerHandler {
  fn throw(&mut self, kind: LexerErrorKind, span: Span);
}

impl LexerHandler for DiagnosticHandler {
  fn throw(&mut self, kind: LexerErrorKind, span: Span) {
    self
      .errors
      .push(Error::LexerError(LexerError { kind, span }))
  }
}
