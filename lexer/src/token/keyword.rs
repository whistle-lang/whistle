#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  Import,
  As,
  From,
  Export,
  Fun,
  Return,
  If,
  Else,
  While,
  Break,
  Continue,
  Var,
  Val,
  None,

  For,
  In,
  Match,
  Type,
  Struct,
  Trait,
}

impl Keyword {
  pub fn from(op: &str) -> Option<Keyword> {
    match op {
      "import" => Some(Keyword::Import),
      "as" => Some(Keyword::As),
      "from" => Some(Keyword::From),
      "export" => Some(Keyword::Export),
      "fun" => Some(Keyword::Fun),
      "return" => Some(Keyword::Return),
      "if" => Some(Keyword::If),
      "else" => Some(Keyword::Else),
      "while" => Some(Keyword::While),
      "break" => Some(Keyword::Break),
      "continue" => Some(Keyword::Continue),
      "var" => Some(Keyword::Var),
      "val" => Some(Keyword::Val),
      "none" => Some(Keyword::None),

      "for" => Some(Keyword::For),
      "in" => Some(Keyword::In),
      "match" => Some(Keyword::Match),
      "type" => Some(Keyword::Type),
      "struct" => Some(Keyword::Struct),
      "trait" => Some(Keyword::Trait),

      _ => None,
    }
  }

  #[allow(clippy::match_like_matches_macro)]
  pub fn is_type(&self) -> bool {
    match self {
      Keyword::None => true,
      _ => false,
    }
  }

  pub fn as_string(&self) -> String {
    match self {
      Keyword::Import => "import".to_string(),
      Keyword::As => "as".to_string(),
      Keyword::From => "from".to_string(),
      Keyword::Export => "export".to_string(),
      Keyword::Fun => "fun".to_string(),
      Keyword::Return => "return".to_string(),
      Keyword::If => "if".to_string(),
      Keyword::Else => "else".to_string(),
      Keyword::While => "while".to_string(),
      Keyword::Break => "break".to_string(),
      Keyword::Continue => "continue".to_string(),
      Keyword::Var => "var".to_string(),
      Keyword::Val => "val".to_string(),
      Keyword::None => "none".to_string(),
      Keyword::For => "for".to_string(),
      Keyword::In => "in".to_string(),
      Keyword::Match => "match".to_string(),
      Keyword::Type => "type".to_string(),
      Keyword::Struct => "struct".to_string(),
      Keyword::Trait => "trait".to_string(),
    }
  }
}
