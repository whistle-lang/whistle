use crate::parser::Parser;

use whistle_common::Operator;
use whistle_common::Token;

pub fn parse_binary_op(parser: &mut Parser) -> Option<Operator> {
  if let Some(Token::Operator(operator)) = parser.eat_type(Token::Operator(Operator::Add)) {
    if Operator::is_binary(operator) {
      return Some(operator.clone());
    }
  }

  None
}

pub fn parse_unary_op(parser: &mut Parser) -> Option<Operator> {
  if let Some(Token::Operator(operator)) = parser.eat_type(Token::Operator(Operator::Add)) {
    if Operator::is_unary(operator) {
      return Some(operator.clone());
    }
  }

  None
}
