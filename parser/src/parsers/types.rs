use crate::parse_ident;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentType;
use whistle_ast::TypeVal;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_ident_type(parser: &mut Parser) -> Result<IdentType, ParserError> {
  let ident_type = match parser.clone().peek()? {
    Token::Keyword(Keyword::Primitive(prim)) => Ok(IdentType::Primitive(prim.clone()))?,
    Token::Ident(ident) => parse_type_val(parser, ident.clone())?,
    _ => {
      return Err(ParserError::new(
        vec![ParserErrorKind::ExpectedPrimaryExpression],
        parser.index,
      ))
    }
  };
  if parser.eat_tok(Token::Operator(Operator::BitOr)).is_ok() {
    return Ok(IdentType::Union {
      lhs: Box::new(ident_type),
      rhs: Box::new(parse_ident_type(parser)?),
    });
  };
  Ok(ident_type)
}

pub fn parse_type_val(parser: &mut Parser, ident: String) -> Result<IdentType, ParserError> {
  parser.step();
  let mut prim = Vec::new();
  while parser.within() {
    prim.push(match parser.peek()? {
      Token::Punc(Punc::Dot) => parse_type_selector(parser)?,
      Token::Punc(Punc::LeftParen) => parse_type_arguments(parser)?,
      _ => break,
    })
  }
  if prim == [] {
    return Ok(IdentType::Ident(ident.clone()));
  }
  Ok(IdentType::IdentType { ident, prim })
}

pub fn parse_type_selector(parser: &mut Parser) -> Result<TypeVal, ParserError> {
  parser.eat_tok(Token::Punc(Punc::Dot))?;
  let ident = parse_ident(parser)?;
  Ok(TypeVal::Selector(ident))
}

pub fn parse_type_arguments(parser: &mut Parser) -> Result<TypeVal, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftAngleBracket))?;
  let mut args = Vec::new();
  if let Some(first) = parser.maybe(parse_ident_type) {
    args.push(first);
    args.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_ident_type(parser)
    }));
  }
  parser.eat_tok(Token::Punc(Punc::RightAngleBracket))?;
  Ok(TypeVal::Arguments(args))
}
