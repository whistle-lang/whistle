use whistle_common::Keyword;
use whistle_common::Token;

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

  ExpectedProgramStmt,
  ExpectedExpression,
  ExpectedOperand,
  ExpectedKeyword(Keyword),
  ExpectedToken(Token),
  ExpectedTokenType(Token),
  UnexpectedEOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
  pub kind: ParserErrorKind,
  pub index: usize,
}

pub trait ParserErrorExtend {
  fn extend(self, err: ParserErrorKind) -> Self;
}

impl ParserError {
  pub fn new(kind: ParserErrorKind, index: usize) -> Self {
    Self { kind, index }
  }
}

impl<T> ParserErrorExtend for Result<T, ParserError> where T: Clone {
  fn extend(self, kind: ParserErrorKind) -> Self {
    if let Err(mut err) = self.clone() {
      err.kind = kind
    }
    self
  }
}
