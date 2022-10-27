use core::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Punc {
  // couldn't think of a name for the @ symbol so here it is for now
  Snabel,
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

impl TryFrom<char> for Punc {
  type Error = ();

  fn try_from(punc: char) -> Result<Self, Self::Error> {
    match punc {
      ',' => Ok(Punc::Comma),
      ':' => Ok(Punc::Colon),
      '.' => Ok(Punc::Dot),
      '[' => Ok(Punc::LeftBracket),
      ']' => Ok(Punc::RightBracket),
      '{' => Ok(Punc::LeftBrace),
      '}' => Ok(Punc::RightBrace),
      '(' => Ok(Punc::LeftParen),
      ')' => Ok(Punc::RightParen),
      '<' => Ok(Punc::LeftAngleBracket),
      '>' => Ok(Punc::RightAngleBracket),
      '@' => Ok(Punc::Snabel),

      _ => Err(()),
    }
  }
}
