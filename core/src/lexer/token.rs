use super::tokens::TokenValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub value: TokenValue,
  pub index: usize,
}

impl Token {
  pub fn new(value: TokenValue, index: usize) -> Self {
    Self { value, index }
  }
}
