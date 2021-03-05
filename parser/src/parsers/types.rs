use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentType;
use whistle_ast::Primitive;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_ident_type(parser: &mut Parser) -> Result<IdentType, ParserError> {
  match parser.clone().peek()? {
    Token::Keyword(Keyword::Primitive(prim)) => parse_type_prim(parser, prim.clone()),
    Token::Ident(ident) => parse_type_val(parser, ident.clone()),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedType,
      parser.index,
    )),
  }
}

pub fn parse_type_prim(parser: &mut Parser, prim: Primitive) -> Result<IdentType, ParserError> {
  parser.step();
  Ok(IdentType::Primitive(prim))
}

pub fn parse_type_val(parser: &mut Parser, ident: String) -> Result<IdentType, ParserError> {
  parser.step();
  if parser.eat_tok(Token::Operator(Operator::LessThan)).is_ok() {
    // if parser.eat_tok(Token::Punc(Punc::LeftAngleBracket)).is_ok() {
    let prim = parse_type_arguments(parser)?;
    return Ok(IdentType::IdentType { ident, prim });
  }
  Ok(IdentType::Ident(ident))
}

pub fn parse_type_arguments(parser: &mut Parser) -> Result<Vec<IdentType>, ParserError> {
  let args = parser.eat_repeat(
    parse_ident_type,
    Some(Token::Punc(Punc::Comma)),
    Token::Operator(Operator::GreaterThan),
    // Token::Punc(Punc::RightAngleBracket),
  )?;
  parser.eat_tok(Token::Operator(Operator::GreaterThan))?;
  // parser.eat_tok(Token::Punc(Punc::RightAngleBracket))?;
  Ok(args)
}
