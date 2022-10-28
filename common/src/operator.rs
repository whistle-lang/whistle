use core::cmp::Reverse;
use core::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
  LogAndAssign,
  LogOrAssign,
  LogAnd,
  LogOr,
  LogNot,

  BitLeftShiftAssign,
  BitRightShiftAssign,
  BitLeftShift,
  BitRightShift,
  BitAndAssign,
  BitOrAssign,
  BitXorAssign,
  BitAnd,
  BitOr,
  BitXor,
  BitNot,

  Cond,

  AddAssign,
  SubAssign,
  MulAssign,
  DivAssign,
  ModAssign,
  ExpAssign,
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Exp,

  Eq,
  NotEq,
  LessThanOrEq,
  GreaterThanOrEq,
  LessThan,
  GreaterThan,

  Assign,
  Pipe,
}

impl TryFrom<&str> for Operator {
  type Error = ();

  fn try_from(operator: &str) -> Result<Self, Self::Error> {
    match operator {
      "&&=" => Ok(Operator::LogAndAssign),
      "||=" => Ok(Operator::LogOrAssign),
      "&&" => Ok(Operator::LogAnd),
      "||" => Ok(Operator::LogOr),
      "!" => Ok(Operator::LogNot),

      "+=" => Ok(Operator::AddAssign),
      "-=" => Ok(Operator::SubAssign),
      "*=" => Ok(Operator::MulAssign),
      "/=" => Ok(Operator::DivAssign),
      "%=" => Ok(Operator::ModAssign),
      "**=" => Ok(Operator::ExpAssign),
      "+" => Ok(Operator::Add),
      "-" => Ok(Operator::Sub),
      "*" => Ok(Operator::Mul),
      "/" => Ok(Operator::Div),
      "%" => Ok(Operator::Mod),
      "**" => Ok(Operator::Exp),

      "<<=" => Ok(Operator::BitLeftShiftAssign),
      ">>=" => Ok(Operator::BitRightShiftAssign),
      "<<" => Ok(Operator::BitLeftShift),
      ">>" => Ok(Operator::BitRightShift),
      "&=" => Ok(Operator::BitAndAssign),
      "|=" => Ok(Operator::BitOrAssign),
      "^=" => Ok(Operator::BitXorAssign),
      "&" => Ok(Operator::BitAnd),
      "|" => Ok(Operator::BitOr),
      "^" => Ok(Operator::BitXor),
      "~" => Ok(Operator::BitNot),

      "==" => Ok(Operator::Eq),
      "!=" => Ok(Operator::NotEq),
      "<=" => Ok(Operator::LessThanOrEq),
      ">=" => Ok(Operator::GreaterThanOrEq),
      "<" => Ok(Operator::LessThan),
      ">" => Ok(Operator::GreaterThan),

      "=" => Ok(Operator::Assign),
      "|>" => Ok(Operator::Pipe),

      _ => Err(()),
    }
  }
}

impl Operator {
  pub fn operators() -> Vec<String> {
    let mut ops = vec![
      String::from("|>"),
      String::from("&&="),
      String::from("**="),
      String::from("<<="),
      String::from(">>="),
      String::from("||="),
      String::from("!="),
      String::from("%="),
      String::from("&&"),
      String::from("&="),
      String::from("**"),
      String::from("*="),
      String::from("+="),
      String::from("-="),
      String::from("/="),
      String::from("<<"),
      String::from("<="),
      String::from("=="),
      String::from(">="),
      String::from(">>"),
      String::from("^="),
      String::from("|="),
      String::from("||"),
      String::from("!"),
      String::from("%"),
      String::from("&"),
      String::from("*"),
      String::from("+"),
      String::from("-"),
      String::from("/"),
      String::from("<"),
      String::from("="),
      String::from(">"),
      String::from("^"),
      String::from("|"),
      String::from("~"),
    ];
    ops.sort_by_key(|b| Reverse(b.to_owned()));
    ops
  }

  pub fn is_unary(&self) -> bool {
    matches!(self, Operator::BitNot | Operator::LogNot | Operator::Sub)
  }

  pub fn is_binary(&self) -> bool {
    matches!(
      self,
      Operator::LogAndAssign
        | Operator::LogOrAssign
        | Operator::LogAnd
        | Operator::LogOr
        | Operator::BitLeftShiftAssign
        | Operator::BitRightShiftAssign
        | Operator::BitLeftShift
        | Operator::BitRightShift
        | Operator::BitAndAssign
        | Operator::BitOrAssign
        | Operator::BitXorAssign
        | Operator::BitAnd
        | Operator::BitOr
        | Operator::BitXor
        | Operator::AddAssign
        | Operator::SubAssign
        | Operator::MulAssign
        | Operator::DivAssign
        | Operator::ModAssign
        | Operator::ExpAssign
        | Operator::Add
        | Operator::Sub
        | Operator::Mul
        | Operator::Div
        | Operator::Mod
        | Operator::Exp
        | Operator::Eq
        | Operator::NotEq
        | Operator::LessThanOrEq
        | Operator::GreaterThanOrEq
        | Operator::LessThan
        | Operator::GreaterThan
        | Operator::Assign
        | Operator::Pipe
    )
  }

  pub fn is_assign(&self) -> bool {
    *self == Operator::Assign
  }

  pub fn get_prec(&self) -> usize {
    match self {
      Operator::LogNot => 0,
      Operator::BitNot => 0,

      Operator::Exp => 1,

      Operator::Mul => 2,
      Operator::Div => 2,
      Operator::Mod => 2,

      Operator::Add => 3,
      Operator::Sub => 3,

      Operator::BitLeftShift => 4,
      Operator::BitRightShift => 4,

      Operator::LessThanOrEq => 5,
      Operator::GreaterThanOrEq => 5,
      Operator::LessThan => 5,
      Operator::GreaterThan => 5,

      Operator::Eq => 6,
      Operator::NotEq => 6,

      Operator::BitAnd => 7,

      Operator::BitXor => 8,

      Operator::BitOr => 9,

      Operator::LogAnd => 10,
      Operator::LogOr => 11,

      Operator::Cond => 12,

      Operator::ExpAssign => 13,

      Operator::DivAssign => 14,
      Operator::ModAssign => 14,
      Operator::MulAssign => 14,

      Operator::AddAssign => 15,
      Operator::SubAssign => 15,

      Operator::BitLeftShiftAssign => 16,
      Operator::BitRightShiftAssign => 16,

      Operator::BitAndAssign => 17,

      Operator::BitXorAssign => 18,

      Operator::BitOrAssign => 19,

      Operator::LogAndAssign => 20,

      Operator::LogOrAssign => 21,

      Operator::Assign => 22,

      Operator::Pipe => 23,

    }
  }
}
