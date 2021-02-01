mod keyword;
pub use keyword::Keyword;
mod operator;
pub use operator::Operator;
mod punc;
pub use punc::Punc;
mod tip;
pub use tip::Tip;
mod token;
pub use token::Token;
pub use token::TokenItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl From<usize> for Range {
  fn from(index: usize) -> Self {
    Range {
      start: index,
      end: index,
    }
  }
}
