use crate::eat_type;
use crate::parse_expr;
use crate::parse_ident_type;
use crate::parse_params;
use crate::parser::Parser;
use crate::ParserError;

use whistle_ast::IdentExternFn;
use whistle_ast::IdentImport;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::IdentVal;
use whistle_ast::Primary;

use whistle_ast::Primitive;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Span;
use whistle_common::Token;

pub fn parse_ident(parser: &mut Parser) -> Result<String, ParserError> {
  eat_type!(parser, Token::Ident)
}

pub fn parse_ident_typed(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  let start = parser.peek()?.span.start;
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    let type_ident = parse_ident_type(parser)?;
    let end = parser.peek_offset(-1)?.span.end;
    return Ok(IdentTyped {
      ident,
      type_ident,
      span: Some(Span { start, end }),
    });
  };
  let end = parser.peek_offset(-1)?.span.end;
  Ok(IdentTyped {
    ident,
    type_ident: IdentType::Default,
    span: Some(Span { start, end }),
  })
}

pub fn parse_ident_import(parser: &mut Parser) -> Result<IdentImport, ParserError> {
  let start = parser.peek()?.span.start;
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Keyword(Keyword::As)).is_ok() {
    let as_ident = parse_ident(parser)?;
    let end = parser.peek_offset(-1)?.span.end;
    return Ok(IdentImport {
      ident,
      as_ident: Some(as_ident),
      span: Span { start, end },
    });
  }
  let end = parser.peek_offset(-1)?.span.end;
  Ok(IdentImport {
    ident,
    as_ident: None,
    span: Span { start, end },
  })
}

pub fn parse_ident_extern(parser: &mut Parser) -> Result<IdentExternFn, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Fn))?;
  let ident = parse_ident(parser)?;
  let params = parse_params(parser)?;
  let ret_type = if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    parse_ident_type(parser)?
  } else {
    let span = Some(parser.peek()?.span);
    IdentType::Primitive {
      prim: Primitive::None,
      span,
    }
  };
  let end = parser.peek_offset(-1)?.span.end;
  Ok(IdentExternFn {
    ident,
    params,
    ret_type,
    span: Span { start, end },
  })
}

pub fn parse_ident_val(parser: &mut Parser, ident: String) -> Result<Primary, ParserError> {
  let start = parser.peek()?.span.start;
  parse_ident(parser)?;
  let mut prim = Vec::new();
  while parser.within() {
    prim.push(match parser.peek()?.token {
      Token::Punc(Punc::Dot) => parse_selector(parser)?,
      Token::Punc(Punc::LeftParen) => parse_arguments(parser)?,
      _ => break,
    })
  }
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Primary::IdentVal {
    ident,
    prim,
    span: Span { start, end },
  })
}

pub fn parse_selector(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Punc(Punc::Dot))?;
  let ident = parse_ident(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(IdentVal::Selector {
    ident,
    span: Span { start, end },
  })
}

pub fn parse_arguments(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let args = parser.eat_repeat(
    parse_expr,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightParen),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(IdentVal::Arguments {
    args,
    span: Span { start, end },
  })
}
