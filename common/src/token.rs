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
  FloatLit(f64),
  IntLit(usize),
  StrLit(String),
  CharLit(char),
  BoolLit(bool),
  Tip(Tip),
  Punc(Punc),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem {
  pub token: Token,
  pub range: Range,
}
