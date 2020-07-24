use super::position::span::SpanData;
use crate::ast::operators::BinaryOp;
use crate::ast::operators::AssignOp as AssignOpToken;

use enum_kind::Kind;
use num_bigint::BigInt as BigIntValue;
use whistle_atoms::{whi_word, WhiWord};

use fmt::Display;
use std::fmt::{self, Debug, Formatter};

// TODO(@qu4k): add tokens to enum
// eg: Import, /// `import

#[derive(Kind, Debug, Clone, PartialEq)]
#[kind(functions(starts_expr = "bool", before_expr = "bool"))]
pub enum Token {
  /// Identifier, "null", "true", "false
  /// Contains `null` and ``
  #[kind(delegate)]
  Word(Word),
  /// '#'
  Hash,
  /// '@'
  At,
  /// '.'
  Dot,
  /// '...'
  #[kind(before_expr)]
  DotDotDot,
  /// '!'
  #[kind(before_expr, starts_expr)]
  Bang,

  /// '('
  #[kind(before_expr, starts_expr)]
  LParen,
  /// ')'
  RParen,
  /// `[`
  #[kind(before_expr, starts_expr)]
  LBracket,
  /// ']'
  RBracket,
  /// '{'
  #[kind(before_expr, starts_expr)]
  LBrace,
  /// '}'
  RBrace,

  /// ';'
  #[kind(before_expr)]
  Semi,
  /// ','
  #[kind(before_expr)]
  Comma,

  /// '`'
  #[kind(starts_expr)]
  BackQuote,

  /// ':'
  #[kind(before_expr)]
  Colon,
  /// '::'
  #[kind(before_expr)]
  ColonColon,
  ///
  #[kind(delegate)]
  BinOp(BinOpToken),
  ///
  #[kind(before_expr)]
  AssignOp(AssignOpToken),

  /// '${'
  #[kind(before_expr, starts_expr)]
  DollarLBrace,

  /// '?'
  #[kind(before_expr)]
  QuestionMark,

  /// `++`
  #[kind(before_expr, starts_expr)]
  PlusPlus,
  /// `--`
  #[kind(before_expr, starts_expr)]
  MinusMinus,

  /// `~`
  #[kind(before_expr, starts_expr)]
  Tilde,

  /// String literal.
  #[kind(starts_expr)]
  Str {
    value: WhiWord,
    /// This field exsits because 'use\x20strict' is **not** an use strict
    /// directive.
    has_escape: bool,
  },

  /// Regexp literal.
  #[kind(starts_expr)]
  Regex(WhiWord, WhiWord),

  /// TODO: Make Num as enum and separate decimal, binary, ..etc
  #[kind(starts_expr)]
  Num(f64),

  #[kind(starts_expr)]
  BigInt(BigIntValue),

  Shebang(WhiWord),
  Error, // TODO(@qu4k): add error type
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenAndSpan {
  pub token: Token,
  /// Had a line break before this token?
  pub had_line_break: bool,
  pub span: SpanData,
}
#[derive(Kind, Clone, PartialEq, Eq, Hash)]
#[kind(functions(starts_expr = "bool", before_expr = "bool"))]
pub enum Word {
  #[kind(delegate)]
  Keyword(Keyword),

  #[kind(starts_expr)]
  Null,

  #[kind(starts_expr)]
  True,
  #[kind(starts_expr)]
  False,

  #[kind(starts_expr)]
  Ident(WhiWord),
}

// TODO(@qu4k): add before_expr && after_expr
// eg: #[kind(before_expr)]
#[derive(Kind, Clone, Copy, PartialEq, Eq, Hash)]
#[kind(function(before_expr = "bool", starts_expr = "bool"))]
pub enum Keyword {
  As,
  Break,
  Continue,
  Else,
  Export,
  For,
  From,
  Fun,
  If,
  Import,
  In,
  Loop,
  Match,
  Return,
  Struct,
  Trait,
  Type,
  Val,
  Var,
  While,
}

#[derive(Kind, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[kind(functions(starts_expr = "bool"))]
pub enum BinOpToken {
  EqEq,
  NotEq,
  Lt,
  LtEq,
  Gt,
  GtEq,
  LShift,
  RShift,
  ZeroFillRShift,

  #[kind(starts_expr)]
  Add,
  #[kind(starts_expr)]
  Sub,
  Mul,
  Div,
  Mod,

  BitOr,
  BitXor,
  BitAnd,
  Exp,

  LogicalOr,
  LogicalAnd,

  NullishCoalescing,
}

impl From<WhiWord> for Word {
  fn from(i: WhiWord) -> Self {
    use self::Keyword::*;
    match i {
      whi_word!("as") => As.into(),
      whi_word!("break") => Break.into(),
      whi_word!("continue") => Continue.into(),
      whi_word!("else") => Else.into(),
      whi_word!("export") => Export.into(),
      whi_word!("false") => Word::False,
      whi_word!("for") => For.into(),
      whi_word!("from") => From.into(),
      whi_word!("fun") => Fun.into(),
      whi_word!("if") => If.into(),
      whi_word!("import") => Import.into(),
      whi_word!("in") => In.into(),
      whi_word!("loop") => Loop.into(),
      whi_word!("match") => Match.into(),
      whi_word!("null") => Word::Null,
      whi_word!("return") => Return.into(),
      whi_word!("struct") => Struct.into(),
      whi_word!("trait") => Trait.into(),
      whi_word!("true") => Word::True,
      whi_word!("type") => Type.into(),
      whi_word!("val") => Val.into(),
      whi_word!("var") => Var.into(),
      whi_word!("while") => While.into(),
      _ => Word::Ident(i),
    }
  }
}

impl From<Keyword> for Word {
  fn from(kwd: Keyword) -> Self {
    Word::Keyword(kwd)
  }
}

impl From<Word> for WhiWord {
  fn from(w: Word) -> Self {
    use self::Keyword::*;
    match w {
      Word::Keyword(k) => match k {
        As => whi_word!("as"),
        Break => whi_word!("break"),
        Continue => whi_word!("continue"),
        Else => whi_word!("else"),
        Export => whi_word!("export"),
        For => whi_word!("for"),
        From => whi_word!("from"),
        Fun => whi_word!("fun"),
        If => whi_word!("if"),
        Import => whi_word!("import"),
        In => whi_word!("in"),
        Loop => whi_word!("loop"),
        Match => whi_word!("match"),
        Return => whi_word!("return"),
        Struct => whi_word!("struct"),
        Trait => whi_word!("trait"),
        Type => whi_word!("type"),
        Val => whi_word!("val"),
        Var => whi_word!("var"),
        While => whi_word!("while"),
      },

      Word::Null => whi_word!("null"),
      Word::True => whi_word!("true"),
      Word::False => whi_word!("false"),

      Word::Ident(w) => w,
    }
  }
}

impl Debug for Word {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      Word::Ident(ref s) => Display::fmt(s, f),
      _ => {
        let s: WhiWord = self.clone().into();
        Display::fmt(&s, f)
      }
    }
  }
}

impl Keyword {
  fn into_whi_word(self) -> WhiWord {
    use self::Keyword::*;
    match self {
      As => whi_word!("as"),
      Break => whi_word!("break"),
      Continue => whi_word!("continue"),
      Else => whi_word!("else"),
      Export => whi_word!("export"),
      For => whi_word!("for"),
      From => whi_word!("from"),
      Fun => whi_word!("fun"),
      If => whi_word!("if"),
      Import => whi_word!("import"),
      In => whi_word!("in"),
      Loop => whi_word!("loop"),
      Match => whi_word!("match"),
      Return => whi_word!("return"),
      Struct => whi_word!("struct"),
      Trait => whi_word!("trait"),
      Type => whi_word!("type"),
      Val => whi_word!("val"),
      Var => whi_word!("var"),
      While => whi_word!("while"),
    }
  }
}

impl Debug for Keyword {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "keyword '{}'", self.into_whi_word())
  }
}

impl From<BinOpToken> for BinaryOp {
  fn from(t: BinOpToken) -> Self {
    use self::BinaryOp::*;
    match t {
      BinOpToken::EqEq => EqEq,
      BinOpToken::NotEq => NotEq,
      BinOpToken::Lt => Lt,
      BinOpToken::LtEq => LtEq,
      BinOpToken::Gt => Gt,
      BinOpToken::GtEq => GtEq,
      BinOpToken::LShift => LShift,
      BinOpToken::RShift => RShift,
      BinOpToken::ZeroFillRShift => ZeroFillRShift,
      BinOpToken::Add => Add,
      BinOpToken::Sub => Sub,
      BinOpToken::Mul => Mul,
      BinOpToken::Div => Div,
      BinOpToken::Mod => Mod,
      BinOpToken::BitOr => BitOr,
      BinOpToken::BitXor => BitXor,
      BinOpToken::BitAnd => BitAnd,
      BinOpToken::LogicalOr => LogicalOr,
      BinOpToken::LogicalAnd => LogicalAnd,
      BinOpToken::Exp => Exp,
      BinOpToken::NullishCoalescing => NullishCoalescing,
    }
  }
}

impl BinOpToken {
  pub const fn before_expr(self) -> bool {
      true
  }
}
