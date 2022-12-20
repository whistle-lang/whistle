mod keyword;
pub use keyword::Keyword;
pub use keyword::Primitive;
mod operator;
pub use operator::Operator;
mod punc;
pub use punc::Punc;
mod tip;
pub use tip::Tip;
mod token;
pub use token::Literal;
pub use token::Token;
pub use token::TokenItem;

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
