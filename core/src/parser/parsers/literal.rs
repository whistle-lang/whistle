pub use crate::parser::ast::*;
pub use crate::parser::parser::*;

pub fn parse_boolean_literal(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(Token::BoolLit(boolean)) = parser.eat_type(Token::BoolLit(true)) {
    return Some(PrimaryExpr::Operand(Operand::Literal(Literal::Bool(
      *boolean,
    ))));
  }
  None
}

pub fn parse_integer_literal(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(Token::IntLit(integer)) = parser.eat_type(Token::IntLit(0)) {
    return Some(PrimaryExpr::Operand(Operand::Literal(Literal::Int(
      *integer,
    ))));
  }
  None
}

pub fn parse_float_literal(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(Token::FloatLit(float)) = parser.eat_type(Token::FloatLit(0.0)) {
    return Some(PrimaryExpr::Operand(Operand::Literal(Literal::Float(
      *float,
    ))));
  }
  None
}

pub fn parse_char_literal(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(chars) = "n".chars().next() {
    if let Some(Token::CharLit(character)) = parser.eat_type(Token::CharLit(chars)) {
      return Some(PrimaryExpr::Operand(Operand::Literal(Literal::Char(
        *character,
      ))));
    }
  }
  None
}

pub fn parse_str_literal(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(Token::StrLit(string)) = parser.eat_type(Token::StrLit(String::new())) {
    return Some(PrimaryExpr::Operand(Operand::Literal(Literal::Str(
      string.to_string(),
    ))));
  }
  None
}
