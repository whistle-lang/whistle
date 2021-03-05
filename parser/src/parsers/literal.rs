use crate::parser::Parser;
use crate::ParserError;

use whistle_ast::Literal;
use whistle_ast::Primary;
use whistle_common::Keyword;
use whistle_common::Primitive;
use whistle_common::Token;

pub fn parse_bool_lit(parser: &mut Parser, val: bool) -> Result<Literal, ParserError> {
  parser.eat_type(Token::Literal(Literal::Bool(val)))?;
  Ok(Literal::Bool(val))
}

pub fn parse_int_lit(parser: &mut Parser, val: usize) -> Result<Literal, ParserError> {
  parser.eat_type(Token::Literal(Literal::Int(val)))?;
  Ok(Literal::Int(val))
}

pub fn parse_float_lit(parser: &mut Parser, val: f64) -> Result<Literal, ParserError> {
  parser.eat_type(Token::Literal(Literal::Float(val)))?;
  Ok(Literal::Float(val))
}

pub fn parse_char_lit(parser: &mut Parser, val: char) -> Result<Literal, ParserError> {
  parser.eat_type(Token::Literal(Literal::Char(val)))?;
  Ok(Literal::Char(val))
}

pub fn parse_str_lit(parser: &mut Parser, val: String) -> Result<Literal, ParserError> {
  parser.eat_type(Token::Literal(Literal::Str(val.clone())))?;
  Ok(Literal::Str(val))
}

pub fn parse_none_lit(parser: &mut Parser) -> Result<Literal, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Primitive(Primitive::None)))?;
  Ok(Literal::None)
}

pub fn parse_lit(parser: &mut Parser, literal: Literal) -> Result<Primary, ParserError> {
  Ok(Primary::Literal(match literal {
    Literal::Bool(val) => parse_bool_lit(parser, val)?,
    Literal::Int(val) => parse_int_lit(parser, val)?,
    Literal::Float(val) => parse_float_lit(parser, val)?,
    Literal::Char(val) => parse_char_lit(parser, val)?,
    Literal::Str(val) => parse_str_lit(parser, val)?,
    Literal::None => parse_none_lit(parser)?,
  }))
}
