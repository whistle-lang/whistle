use crate::lexer::Keyword;
use crate::lexer::Punc;
use crate::lexer::Token;
use crate::parser::ast::*;
use crate::parser::Parser;

pub fn parse_ident(parser: &mut Parser) -> Option<String> {
  if let Some(Token::Ident(ident)) = parser.eat_type(Token::Ident(String::new())) {
    Some(ident.clone())
  } else {
    None
  }
}

pub fn parse_ident_typed(parser: &mut Parser) -> Option<IdentTyped> {
  parser.maybe(|parser| {
    if let Some(ident) = parse_ident(parser) {
      if parser.eat_tok(Token::Punc(Punc::Colon)).is_some() {
        if let Some(type_ident) = parse_ident_type(parser) {
          return Some(IdentTyped { ident, type_ident });
        }
      }
    }

    None
  })
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

pub fn parse_ident_as(parser: &mut Parser) -> Option<IdentImport> {
  if let Some(ident) = parse_ident(parser) {
    if parser.eat_tok(Token::Keyword(Keyword::As)).is_some() {
      if let Some(as_ident) = parse_ident(parser) {
        return Some(IdentImport {
          ident,
          as_ident: Some(as_ident),
        });
      }
    }
  }

  None
}

pub fn parse_ident_as_import(parser: &mut Parser) -> Option<IdentImport> {
  if let Some(ident) = parse_ident(parser) {
    Some(IdentImport {
      ident,
      as_ident: None,
    })
  } else {
    None
  }
}

pub fn parse_ident_import(parser: &mut Parser) -> Option<IdentImport> {
  parser.or(Vec::from([parse_ident_as, parse_ident_as_import]))
}
