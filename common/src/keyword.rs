use core::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  Import,
  As,
  From,
  Builtin,
  Export,
  Inline,
  Fn,
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
}

#[derive(PartialEq, Clone, Debug)]
pub enum Primitive {
  F32,
  F64,
  I32,
  I64,
  U32,
  U64,
  Int,
  Float,
  Number,
  Char,
  Bool,
  Str,
  None,
}

impl TryFrom<&str> for Keyword {
  type Error = ();

  fn try_from(keyword: &str) -> Result<Self, Self::Error> {
    match keyword {
      "import" => Ok(Keyword::Import),
      "builtin" => Ok(Keyword::Builtin),
      "as" => Ok(Keyword::As),
      "from" => Ok(Keyword::From),
      "export" => Ok(Keyword::Export),
      "inline" => Ok(Keyword::Inline),
      "fn" => Ok(Keyword::Fn),
      "return" => Ok(Keyword::Return),
      "if" => Ok(Keyword::If),
      "else" => Ok(Keyword::Else),
      "while" => Ok(Keyword::While),
      "break" => Ok(Keyword::Break),
      "continue" => Ok(Keyword::Continue),
      "var" => Ok(Keyword::Var),
      "val" => Ok(Keyword::Val),

      "none" => Ok(Keyword::Primitive(Primitive::None)),
      "bool" => Ok(Keyword::Primitive(Primitive::Bool)),
      "char" => Ok(Keyword::Primitive(Primitive::Char)),
      "str" => Ok(Keyword::Primitive(Primitive::Str)),
      "i32" => Ok(Keyword::Primitive(Primitive::I32)),
      "i64" => Ok(Keyword::Primitive(Primitive::I64)),
      "u32" => Ok(Keyword::Primitive(Primitive::U32)),
      "u64" => Ok(Keyword::Primitive(Primitive::U64)),
      "f32" => Ok(Keyword::Primitive(Primitive::F32)),
      "f64" => Ok(Keyword::Primitive(Primitive::F64)),

      "for" => Ok(Keyword::For),
      "in" => Ok(Keyword::In),
      "match" => Ok(Keyword::Match),
      "type" => Ok(Keyword::Type),
      "struct" => Ok(Keyword::Struct),

      _ => Err(()),
    }
  }
}

impl From<Keyword> for &str {
  fn from(val: Keyword) -> Self {
    match val {
      Keyword::Import => "import",
      Keyword::As => "as",
      Keyword::From => "from",
      Keyword::Export => "export",
      Keyword::Inline => "inline",
      Keyword::Fn => "fn",
      Keyword::Return => "return",
      Keyword::If => "if",
      Keyword::Else => "else",
      Keyword::While => "while",
      Keyword::Break => "break",
      Keyword::Continue => "continue",
      Keyword::Var => "var",
      Keyword::Val => "val",

      Keyword::Primitive(Primitive::None) => "none",
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
      _ => unreachable!(),
    }
  }
}
