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

  Primitive(Primitive),

  For,
  In,
  Match,
  Type,
  Struct,
  Trait,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
  F32,
  F64,
  I32,
  I64,
  U32,
  U64,
  Char,
  Bool,
  Str,
  None,
  Any,
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

      "none" => Some(Keyword::Primitive(Primitive::None)),
      "any" => Some(Keyword::Primitive(Primitive::Any)),
      "bool" => Some(Keyword::Primitive(Primitive::Bool)),
      "char" => Some(Keyword::Primitive(Primitive::Char)),
      "str" => Some(Keyword::Primitive(Primitive::Str)),
      "i32" => Some(Keyword::Primitive(Primitive::I32)),
      "i64" => Some(Keyword::Primitive(Primitive::I64)),
      "u32" => Some(Keyword::Primitive(Primitive::U32)),
      "u64" => Some(Keyword::Primitive(Primitive::U64)),
      "f32" => Some(Keyword::Primitive(Primitive::F32)),
      "f64" => Some(Keyword::Primitive(Primitive::F64)),

      "for" => Some(Keyword::For),
      "in" => Some(Keyword::In),
      "match" => Some(Keyword::Match),
      "type" => Some(Keyword::Type),
      "struct" => Some(Keyword::Struct),
      "trait" => Some(Keyword::Trait),

      _ => None,
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

      Keyword::Primitive(Primitive::None) => "none",
      Keyword::Primitive(Primitive::Any) => "any",
      Keyword::Primitive(Primitive::Bool) => "bool",
      Keyword::Primitive(Primitive::Char) => "char",
      Keyword::Primitive(Primitive::Str) => "str",
      Keyword::Primitive(Primitive::I32) => "i32",
      Keyword::Primitive(Primitive::I64) => "i64",
      Keyword::Primitive(Primitive::U32) => "u32",
      Keyword::Primitive(Primitive::U64) => "u64",
      Keyword::Primitive(Primitive::F32) => "f32",
      Keyword::Primitive(Primitive::F64) => "f64",

      Keyword::For => "for",
      Keyword::In => "in",
      Keyword::Match => "match",
      Keyword::Type => "type",
      Keyword::Struct => "struct",
      Keyword::Trait => "trait",
    }
    .to_string()
  }
}
