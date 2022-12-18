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
use whistle_common::Range;
use whistle_common::Token;

pub fn parse_ident(parser: &mut Parser) -> Result<String, ParserError> {
  eat_type!(parser, Token::Ident)
}

pub fn parse_ident_typed(parser: &mut Parser) -> Result<IdentTyped, ParserError> {
  let start = parser.peek()?.range.start;
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    let type_ident = parse_ident_type(parser)?;
    let end = parser.peek_offset(-1)?.range.end;
    return Ok(IdentTyped {
      ident,
      type_ident,
      range: Some(Range { start, end }),
    });
  };
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentTyped {
    ident,
    type_ident: IdentType::Default,
    range: Some(Range { start, end }),
  })
}

pub fn parse_ident_import(parser: &mut Parser) -> Result<IdentImport, ParserError> {
  let start = parser.peek()?.range.start;
  let ident = parse_ident(parser)?;
  if parser.eat_tok(Token::Keyword(Keyword::As)).is_ok() {
    let as_ident = parse_ident(parser)?;
    let end = parser.peek_offset(-1)?.range.end;
    return Ok(IdentImport {
      ident,
      as_ident: Some(as_ident),
      range: Range { start, end },
    });
  }
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentImport {
    ident,
    as_ident: None,
    range: Range { start, end },
  })
}

pub fn parse_ident_extern(parser: &mut Parser) -> Result<IdentExternFn, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Keyword(Keyword::Fn))?;
  let ident = parse_ident(parser)?;
  let params = parse_params(parser)?;
  let ret_type = if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    parse_ident_type(parser)?
  } else {
    let range = Some(parser.peek()?.range);
    IdentType::Primitive {
      prim: Primitive::None,
      range,
    }
  };
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentExternFn {
    ident,
    params,
    ret_type,
    range: Range { start, end },
  })
}

pub fn parse_ident_val(parser: &mut Parser, ident: String) -> Result<Primary, ParserError> {
  let start = parser.peek()?.range.start;
  parse_ident(parser)?;
  let mut prim = Vec::new();
  while parser.within() {
    prim.push(match parser.peek()?.token {
      Token::Punc(Punc::Dot) => parse_selector(parser)?,
      Token::Punc(Punc::LeftParen) => parse_arguments(parser)?,
      _ => break,
    })
  }
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Primary::IdentVal {
    ident,
    prim,
    range: Range { start, end },
  })
}

pub fn parse_selector(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Punc(Punc::Dot))?;
  let ident = parse_ident(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentVal::Selector {
    ident,
    range: Range { start, end },
  })
}

pub fn parse_arguments(parser: &mut Parser) -> Result<IdentVal, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let args = parser.eat_repeat(
    parse_expr,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightParen),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(IdentVal::Arguments {
    args,
    range: Range { start, end },
  })
}
