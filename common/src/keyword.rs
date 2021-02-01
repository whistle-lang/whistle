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
  Bool,
  Char,
  Str,
  I32,
  I64,
  U32,
  U64,
  F32,
  F64,

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
      "bool" => Some(Keyword::Bool),
      "char" => Some(Keyword::Char),
      "string" => Some(Keyword::Str),
      "i32" => Some(Keyword::I32),
      "i64" => Some(Keyword::I64),
      "u32" => Some(Keyword::U32),
      "u64" => Some(Keyword::U64),
      "f32" => Some(Keyword::F32),
      "f64" => Some(Keyword::F64),

      "for" => Some(Keyword::For),
      "in" => Some(Keyword::In),
      "match" => Some(Keyword::Match),
      "type" => Some(Keyword::Type),
      "struct" => Some(Keyword::Struct),
      "trait" => Some(Keyword::Trait),

      _ => None,
    }
  }

  pub fn is_type(&self) -> bool {
    match self {
      Keyword::None => true,
      Keyword::Bool => true,
      Keyword::Char => true,
      Keyword::Str => true,
      Keyword::I32 => true,
      Keyword::I64 => true,
      Keyword::U32 => true,
      Keyword::U64 => true,
      Keyword::F32 => true,
      Keyword::F64 => true,
      _ => false,
    }
  }

  pub fn as_string(&self) -> String {
    match self {
      Keyword::Import => "import",
      Keyword::As => "as",
      Keyword::From => "from",
      Keyword::Export => "export",
      Keyword::Fun => "fun",
      Keyword::Return => "return",
      Keyword::If => "if",
      Keyword::Else => "else",
      Keyword::While => "while",
      Keyword::Break => "break",
      Keyword::Continue => "continue",
      Keyword::Var => "var",
      Keyword::Val => "val",

      Keyword::None => "none",
      Keyword::Bool => "bool",
      Keyword::Char => "char",
      Keyword::Str => "str",
      Keyword::I32 => "i32",
      Keyword::I64 => "i64",
      Keyword::U32 => "u32",
      Keyword::U64 => "u64",
      Keyword::F32 => "f32",
      Keyword::F64 => "f64",

      Keyword::For => "for",
      Keyword::In => "in",
      Keyword::Match => "match",
      Keyword::Type => "type",
      Keyword::Struct => "struct",
      Keyword::Trait => "trait",
    }.to_string()
  }
}
