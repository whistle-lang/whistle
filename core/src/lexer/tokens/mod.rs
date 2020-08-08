mod operator;
pub use operator::Operator;
mod keyword;
pub use keyword::Keyword;
mod tip;
pub use tip::Tip;
mod punc;
pub use punc::Punc;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
  CommentLine(String),
  CommentInline(String),
  Ident(String),
  Keyword(Keyword),
  Operator(Operator),
  FloatLit(f64),
  IntLit(usize),
  StringLit(String),
  CharLit(char),
  BoolLit(bool),
  NoneLit,
  Tip(Tip),
  Punc(Punc),
  EOF
}
