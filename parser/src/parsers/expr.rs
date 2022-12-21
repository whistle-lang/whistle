use crate::error::ParserError;
use crate::error::ParserErrorKind;
use crate::parser::Parser;
use crate::parsers::ident::parse_ident_val;
use crate::parsers::literal::parse_lit;

use whistle_ast::Expr;
use whistle_ast::Primary;
use whistle_ast::Unary;

use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Span;
use whistle_common::Token;

pub fn parse_expr(parser: &mut Parser) -> Result<Expr, ParserError> {
  if parser.is_tok(Token::Keyword(Keyword::If)) {
    return parse_cond(parser);
  }
  let start = parser.peek()?.span.start;
  let unary = parse_unary(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  let lhs = Expr::Unary {
    unary,
    span: Span { start, end },
  };
  let expr = parse_expr_prec(parser, start, lhs, usize::MAX)?;
  Ok(expr)
}

pub fn is_greater_precedence(parser: &mut Parser, prec: usize) -> Option<Operator> {
  if let Ok(token) = parser.peek() {
    if let Token::Operator(op) = &token.token {
      if Operator::is_binary(&op) && op.get_prec() <= prec {
        return Some(op.clone());
      }
    }
  }
  None
}

pub fn parse_expr_prec(
  parser: &mut Parser,
  start: usize,
  expr: Expr,
  prec: usize,
) -> Result<Expr, ParserError> {
  let mut lhs = expr;
  while let Some(op) = is_greater_precedence(parser, prec) {
    parser.step();
    let start_rhs = parser.peek()?.span.start;
    let unary = parse_unary(parser)?;
    let end_rhs = parser.peek_offset(-1)?.span.end;
    let mut rhs = Expr::Unary {
      unary,
      span: Span {
        start: start_rhs,
        end: end_rhs,
      },
    };
    while let Some(op) = is_greater_precedence(parser, op.get_prec()) {
      rhs = parse_expr_prec(parser, start_rhs, rhs, op.get_prec())?;
    }
    let end = parser.peek_offset(-1)?.span.end;
    lhs = Expr::Binary {
      lhs: Box::new(lhs),
      op,
      rhs: Box::new(rhs),
      span: Span { start, end },
    }
  }
  Ok(lhs)
}

pub fn parse_unary(parser: &mut Parser) -> Result<Unary, ParserError> {
  let start = parser.peek()?.span.start;
  if let Token::Operator(op) = &parser.peek()?.token.clone() {
    if op.is_unary() {
      parser.step();
      let op = op.clone();
      let expr = Box::new(parse_unary(parser)?);
      let end = parser.peek_offset(-1)?.span.end;
      return Ok(Unary::UnaryOp {
        op,
        expr,
        span: Span { start, end },
      });
    }
  }
  let prim = parse_primary(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Unary::Primary {
    prim,
    span: Span { start, end },
  })
}

pub fn parse_primary(parser: &mut Parser) -> Result<Primary, ParserError> {
  match &parser.peek()?.token.clone() {
    Token::Literal(lit) => parse_lit(parser, lit.to_owned()),
    Token::Punc(Punc::LeftParen) => parse_grouping(parser),
    Token::Punc(Punc::LeftBracket) => parse_array(parser),
    Token::Ident(ident) => parse_ident_val(parser, ident.clone()),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedPrimaryExpression,
      parser.peek()?.span,
    )),
  }
}

pub fn parse_array(parser: &mut Parser) -> Result<Primary, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Punc(Punc::LeftBracket))?;
  let exprs = parser.eat_repeat(
    parse_expr,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightBracket),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightBracket))?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Primary::Array {
    exprs,
    meta_id: 0,
    span: Span { start, end },
  })
}

pub fn parse_grouping(parser: &mut Parser) -> Result<Primary, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let expr = parse_expr(parser)?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Primary::Grouping {
    group: Box::new(expr),
    span: Span { start, end },
  })
}

pub fn parse_cond(parser: &mut Parser) -> Result<Expr, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;
  let then_expr = parse_expr(parser)?;
  parser.eat_tok(Token::Keyword(Keyword::Else))?;
  let else_expr = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Expr::Cond {
    cond: Box::new(cond),
    then_expr: Box::new(then_expr),
    else_expr: Box::new(else_expr),
    span: Span { start, end },
  })
}
