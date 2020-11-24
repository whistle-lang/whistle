use std::cmp::Reverse;

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
}

impl Operator {
  pub fn from(op: &str) -> Option<Operator> {
    match op {
      "&&=" => Some(Operator::LogAndAssign),
      "||=" => Some(Operator::LogOrAssign),
      "&&" => Some(Operator::LogAnd),
      "||" => Some(Operator::LogOr),
      "!" => Some(Operator::LogNot),

      "+=" => Some(Operator::AddAssign),
      "-=" => Some(Operator::SubAssign),
      "*=" => Some(Operator::MulAssign),
      "/=" => Some(Operator::DivAssign),
      "%=" => Some(Operator::ModAssign),
      "**=" => Some(Operator::ExpAssign),
      "+" => Some(Operator::Add),
      "-" => Some(Operator::Sub),
      "*" => Some(Operator::Mul),
      "/" => Some(Operator::Div),
      "%" => Some(Operator::Mod),
      "**" => Some(Operator::Exp),

      "<<=" => Some(Operator::BitLeftShiftAssign),
      ">>=" => Some(Operator::BitRightShiftAssign),
      "<<" => Some(Operator::BitLeftShift),
      ">>" => Some(Operator::BitRightShift),
      "&=" => Some(Operator::BitAndAssign),
      "|=" => Some(Operator::BitOrAssign),
      "^=" => Some(Operator::BitXorAssign),
      "&" => Some(Operator::BitAnd),
      "|" => Some(Operator::BitOr),
      "^" => Some(Operator::BitXor),
      "~" => Some(Operator::BitNot),

      "==" => Some(Operator::Eq),
      "!=" => Some(Operator::NotEq),
      "<=" => Some(Operator::LessThanOrEq),
      ">=" => Some(Operator::GreaterThanOrEq),
      "<" => Some(Operator::LessThan),
      ">" => Some(Operator::GreaterThan),

      "=" => Some(Operator::Assign),

      _ => None,
    }
  }

  pub fn operators() -> Vec<String> {
    let mut ops = vec![
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
    vec![Operator::BitNot, Operator::LogNot, Operator::Sub].contains(self)
  }

  pub fn is_binary(&self) -> bool {
    vec![
      Operator::LogAndAssign,
      Operator::LogOrAssign,
      Operator::LogAnd,
      Operator::LogOr,
      Operator::BitLeftShiftAssign,
      Operator::BitRightShiftAssign,
      Operator::BitLeftShift,
      Operator::BitRightShift,
      Operator::BitAndAssign,
      Operator::BitOrAssign,
      Operator::BitXorAssign,
      Operator::BitAnd,
      Operator::BitOr,
      Operator::BitXor,
      Operator::AddAssign,
      Operator::SubAssign,
      Operator::MulAssign,
      Operator::DivAssign,
      Operator::ModAssign,
      Operator::ExpAssign,
      Operator::Add,
      Operator::Sub,
      Operator::Mul,
      Operator::Div,
      Operator::Mod,
      Operator::Exp,
      Operator::Eq,
      Operator::NotEq,
      Operator::LessThanOrEq,
      Operator::GreaterThanOrEq,
      Operator::LessThan,
      Operator::GreaterThan,
      Operator::Assign,
    ]
    .contains(self)
  }

  pub fn is_assign(&self) -> bool {
    vec![Operator::Assign].contains(self)
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
    }
  }
}
