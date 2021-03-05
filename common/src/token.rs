use crate::Keyword;
use crate::Operator;
use crate::Punc;
use crate::Range;
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
  pub range: Range,
}

/// https://whistle.js.org/docs/specification/grammar#literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Float(f64),
  Int(usize),
  Str(String),
  Char(char),
  Bool(bool),
  None,
}
