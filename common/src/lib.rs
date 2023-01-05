mod keyword;
pub use keyword::Keyword;
pub use keyword::Primitive;
mod operator;
pub use operator::Operator;
mod punc;
pub use punc::Punc;
mod tip;
pub use tip::Tip;
mod error;
mod token;
pub use error::*;
mod types;
pub use token::Literal;
pub use token::Token;
pub use token::TokenItem;
pub use types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

pub const DUMMY: Span = Span { start: 0, end: 0 };

impl From<usize> for Span {
  fn from(index: usize) -> Self {
    Span {
      start: index,
      end: index,
    }
  }
}
