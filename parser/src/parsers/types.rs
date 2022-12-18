use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentType;
use whistle_ast::Primitive;

use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Range;
use whistle_common::Token;

pub fn parse_ident_type(parser: &mut Parser) -> Result<IdentType, ParserError> {
  let start = parser.peek()?.range.start;
  let ident_type = match &parser.peek()?.token.clone() {
    Token::Keyword(Keyword::Primitive(prim)) => parse_type_prim(parser, prim.clone()),
    Token::Ident(ident) => parse_type_val(parser, ident.clone()),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedType,
      parser.peek()?.range,
    )),
  }?;

  if parser.eat_tok(Token::Punc(Punc::LeftBracket)).is_ok()
    && parser.eat_tok(Token::Punc(Punc::RightBracket)).is_ok()
  {
    let end = parser.peek_offset(-1)?.range.end;
    Ok(IdentType::Array {
      ident: Box::new(ident_type),
      range: Some(Range { start, end }),
    })
  } else {
    Ok(ident_type)
  }
}

pub fn parse_type_prim(parser: &mut Parser, prim: Primitive) -> Result<IdentType, ParserError> {
  let range = Some(parser.peek()?.range);
  parser.step();
  Ok(IdentType::Primitive { prim, range })
}

pub fn parse_type_val(parser: &mut Parser, ident: String) -> Result<IdentType, ParserError> {
  let start = parser.peek()?.range.start;
  parser.step();
  if parser.eat_tok(Token::Operator(Operator::LessThan)).is_ok() {
    let prim = parse_type_arguments(parser)?;
    let end = parser.peek_offset(-1)?.range.end;
    return Ok(IdentType::IdentType {
      ident,
      prim,
      range: Some(Range { start, end }),
    });
  }
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentType::Ident {
    ident,
    range: Some(Range { start, end }),
  })
}

pub fn parse_type_arguments(parser: &mut Parser) -> Result<Vec<IdentType>, ParserError> {
  let args = parser.eat_repeat(
    parse_ident_type,
    Some(Token::Punc(Punc::Comma)),
    Token::Operator(Operator::GreaterThan),
  )?;
  parser.eat_tok(Token::Operator(Operator::GreaterThan))?;
  Ok(args)
}
