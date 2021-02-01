use whistle_common::Keyword;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorKind {
  ExpectedProgramStmt,

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

  ExpectedKeyword(Keyword),
  ExpectedIdent,
  ExpectedType,
  ExpectedTip,

  ExpectedExpression,

  ExpectedExpressionStatement,
  ExpectedBlockStmtStart,
  ExpectedBlockStmtEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
  pub kind: ParserErrorKind,
  pub index: usize,
}

impl ParserError {
  pub fn new(kind: ParserErrorKind, index: usize) -> Self {
    Self { kind, index }
  }
}
