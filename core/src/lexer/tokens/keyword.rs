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
    "for" => Some(Keyword::For),
    "in" => Some(Keyword::In),
    "match" => Some(Keyword::Match),
    "type" => Some(Keyword::Type),
    "struct" => Some(Keyword::Struct),
    "trait" => Some(Keyword::Trait),

      _ => None,
    }
  }
}
