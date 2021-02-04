use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentImport;
use whistle_ast::IdentTyped;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_ident(parser: &mut Parser) -> Result<String, ParserError> {
  if let Token::Ident(ident) = parser.eat_type(Token::Ident(String::new()))? {
    return Ok(ident.clone());
  }
  Err(ParserError::new(
    ParserErrorKind::ExpectedIdent,
    parser.index,
  ))
}

pub fn parse_ident_option(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    let type_ident = parse_ident_type(parser)?;
    return Ok(IdentTyped { ident, type_ident });
  }
  Ok(IdentTyped {
    ident,
    type_ident: String::from("none"),
  })
}

pub fn parse_ident_typed(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  let ident = parse_ident(parser)?;
  parser.eat_tok(Token::Punc(Punc::Colon))?;
  let type_ident = parse_ident_type(parser)?;
  Ok(IdentTyped { ident, type_ident })
}

pub fn parse_ident_type(parser: &mut Parser) -> Result<String, ParserError> {
  if let Token::Keyword(keyw) = parser.clone().peek()? {
    if keyw.is_type() {
      parser.step();
      return Ok(keyw.as_string());
    }
  }
  parse_ident(parser)
}

pub fn parse_ident_import(parser: &mut Parser) -> Result<IdentImport, ParserError> {
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Keyword(Keyword::As)).is_ok() {
    let as_ident = parse_ident(parser)?;
    return Ok(IdentImport {
      ident,
      as_ident: Some(as_ident),
    });
  }
  Ok(IdentImport {
    ident,
    as_ident: None,
  })
}
