use crate::Keyword;
use crate::Operator;
use crate::Punc;
use crate::Span;
use crate::Tip;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  CommentLine(String),
  CommentInline(String),
  Ident(String),
  Keyword(Keyword),
  Operator(Operator),
  Literal(Literal),
  Tip(Tip),
  Punc(Punc),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem {
  pub token: Token,
  pub span: Span,
}

/// https://whistle.js.org/docs/specification/grammar#literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Float(f64),
  F32(f64),
  F64(f64),
  Int(usize),
  I32(usize),
  I64(usize),
  U32(usize),
  U64(usize),
  Str(String),
  Char(char),
  Bool(bool),
  None,
}
