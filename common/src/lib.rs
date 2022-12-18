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
pub struct Range {
  pub start: usize,
  pub end: usize,
}

pub const DUMMY: Range = Range { start: 0, end: 0 };

impl From<usize> for Range {
  fn from(index: usize) -> Self {
    Range {
      start: index,
      end: index,
    }
  }
}
