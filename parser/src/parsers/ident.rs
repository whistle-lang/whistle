use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentImport;
use whistle_ast::IdentTyped;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_ident(parser: &mut Parser) -> Option<String> {
  if let Some(Token::Ident(ident)) = parser.eat_type(Token::Ident(String::new())) {
    Some(ident.clone())
  } else {
    None
  }
}

pub fn parse_ident_typed(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  if let Some(ident) = parse_ident(parser) {
    if parser.eat_tok(Token::Punc(Punc::Colon)).is_some() {
      if let Some(type_ident) = parse_ident_type(parser) {
        Ok(IdentTyped { ident, type_ident })
      } else {
        Err(ParserError::new(ParserErrorKind::ExpectedType, parser.index))
      }
    } else {
      Err(ParserError::new(ParserErrorKind::ExpectedType, parser.index))
    }
  } else {
    Err(ParserError::new(ParserErrorKind::ExpectedIdent, parser.index))
  }
}

pub fn parse_ident_type(parser: &mut Parser) -> Option<String> {
  if let Some(ident) = parse_ident(parser) {
    return Some(ident);
  } else if let Some(Token::Keyword(keyw)) = parser.clone().peek() {
    if keyw.is_type() {
      parser.step();
      return Some(keyw.as_string());
    }
  }

  None
}

pub fn parse_ident_as(parser: &mut Parser) -> Option<Result<IdentImport, ParserError>> {
  if let Some(ident) = parse_ident(parser) {
    if parser.eat_tok(Token::Keyword(Keyword::As)).is_some() {
      return if let Some(as_ident) = parse_ident(parser) {
        Some(Ok(IdentImport {
          ident,
          as_ident: Some(as_ident),
        }))
      } else {
        Some(Err(ParserError::new(
          ParserErrorKind::ExpectedAsAlias,
          parser.index,
        )))
      };
    }
  }

  None
}

pub fn parse_ident_as_import(parser: &mut Parser) -> Option<Result<IdentImport, ParserError>> {
  if let Some(ident) = parse_ident(parser) {
    Some(Ok(IdentImport {
      ident,
      as_ident: None,
    }))
  } else {
    None
  }
}

pub fn parse_ident_import(parser: &mut Parser) -> Result<IdentImport, ParserError> {
  if let Some(result) = parser.or(Vec::from([parse_ident_as, parse_ident_as_import])) {
    result
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedImportIdent,
      parser.index,
    ))
  }
}
