use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentType;
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
  if &Token::Punc(Punc::LeftAngleBracket) == parser.peek()? {
    let prim = parse_type_arguments(parser)?;
    return Ok(IdentType::IdentType { ident, prim })
  }
  Ok(IdentType::Ident(ident.clone()))
}

pub fn parse_type_arguments(parser: &mut Parser) -> Result<Vec<IdentType>, ParserError> {
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
  Ok(args)
}
