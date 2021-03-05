use whistle_common::Keyword;
use whistle_common::Range;
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
pub struct ParserErrorList {
  pub kind: ParserErrorKind,
  pub index: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
  pub err: Vec<ParserErrorList>,
}

impl ParserError {
  pub fn new(kind: ParserErrorKind, count: usize) -> Self {
    let index = Range {
      start: count,
      end: count,
    };
    let err = vec![ParserErrorList { kind, index }];
    ParserError { err }
  }

  pub fn push(&mut self, kind: ParserErrorKind, count: usize) {
    let index = Range {
      start: count,
      end: count,
    };
    let err = ParserErrorList { kind, index };
    self.err.push(err);
  }

  pub fn extend(&mut self, err: ParserError) {
    self.err.extend(err.err);
  }

  pub fn range(&mut self, count: usize) {
    let len = self.err.len();
    self.err[len - 1].index.end = count;
  }

  pub fn index(self) -> Range {
    let len = self.err.len();
    self.err[len - 1].index
  }
}
