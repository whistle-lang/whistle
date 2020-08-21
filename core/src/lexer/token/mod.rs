mod operator;
pub use operator::Operator;
mod keyword;
pub use keyword::Keyword;
mod tip;
pub use tip::Tip;
mod punc;
pub use punc::Punc;

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
pub struct TokenPos {
  pub start: usize,
  pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenItem {
  pub token: Token,
  pub pos: TokenPos,
}
