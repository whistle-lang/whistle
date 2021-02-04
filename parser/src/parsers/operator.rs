use crate::parser::Parser;

use super::super::error::ParserError;
use super::super::error::ParserErrorKind;
use whistle_common::Operator;
use whistle_common::Token;

pub fn parse_binary_op(parser: &mut Parser) -> Result<Operator, ParserError> {
  if let Token::Operator(operator) = parser.eat_type(Token::Operator(Operator::Add))? {
    if Operator::is_binary(operator) {
      return Ok(operator.clone());
    }
  }
  Err(ParserError::new(
    ParserErrorKind::ExpectedUnaryOperator,
    parser.index,
  ))
}

pub fn parse_unary_op(parser: &mut Parser) -> Result<Operator, ParserError> {
  if let Token::Operator(operator) = parser.eat_type(Token::Operator(Operator::Add))? {
    if Operator::is_unary(operator) {
      return Ok(operator.clone());
    }
  }
  Err(ParserError::new(
    ParserErrorKind::ExpectedBinaryOperator,
    parser.index,
  ))
}
