use crate::DiagnosticHandler;
use crate::Error;
use crate::Keyword;
use crate::Span;
use crate::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
  ExpectedFunIdent,
  ExpectedReturnType,
  ExpectedFunBody,

  ExpectedImportLocation,
  ExpectedAsAlias,
  ExpectedImportIdent,

  ExpectedVarIdent,
  ExpectedValIdent,

  ExpectedAssignment,
  ExpectedOperator,
  ExpectedIfCondition,
  ExpectedIfThenBody,
  ExpectedIfElseBody,
  ExpectedWhileBody,
  ExpectedIdent,
  ExpectedType,
  ExpectedTip,

  ExpectedExpressionStatement,
  ExpectedBlockStmtStart,
  ExpectedBlockStmtEnd,

  ExpectedUnaryOperator,
  ExpectedBinaryOperator,

  ExpectedPrimaryExpression,
  ExpectedProgramStmt,
  ExpectedExpression,
  ExpectedOperand,
  ExpectedKeyword(Keyword),
  ExpectedToken(Token),
  ExpectedTokens(Vec<Token>),
  ExpectedTokenType(String),
  UnexpectedEOF,
  MissingDelimiter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
  pub kind: ParserErrorKind,
  pub span: Span,
}

impl ParserError {
  pub fn new(kind: ParserErrorKind, span: Span) -> Self {
    Self { kind, span }
  }
}

pub trait ParserHandler {
  fn throw(&mut self, kind: ParserErrorKind, span: Span);
}

impl ParserHandler for DiagnosticHandler {
  fn throw(&mut self, kind: ParserErrorKind, span: Span) {
    self
      .errors
      .push(Error::ParserError(ParserError { kind, span }))
  }
}
