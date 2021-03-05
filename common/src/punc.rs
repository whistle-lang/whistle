#[derive(Debug, Clone, PartialEq)]
pub enum Punc {
  Comma,
  Colon,
  Dot,
  LeftBracket,
  RightBracket,
  LeftBrace,
  RightBrace,
  LeftParen,
  RightParen,
  LeftAngleBracket,
  RightAngleBracket,
}

impl Punc {
  pub fn from(op: char) -> Option<Punc> {
    match op {
      ',' => Some(Punc::Comma),
      ':' => Some(Punc::Colon),
      '.' => Some(Punc::Dot),
      '[' => Some(Punc::LeftBracket),
      ']' => Some(Punc::RightBracket),
      '{' => Some(Punc::LeftBrace),
      '}' => Some(Punc::RightBrace),
      '(' => Some(Punc::LeftParen),
      ')' => Some(Punc::RightParen),
      '<' => Some(Punc::LeftAngleBracket),
      '>' => Some(Punc::RightAngleBracket),

      _ => None,
    }
  }
}
