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
use whistle_common::Token;

pub fn parse_expr(parser: &mut Parser) -> Result<Expr, ParserError> {
  if parser.is_tok(Token::Keyword(Keyword::If)) {
    return parse_cond(parser);
  }

  let lhs = Expr::Unary(parse_unary(parser)?);
  let expr = parse_expr_prec(parser, lhs, usize::MAX)?;
  Ok(expr)
}

pub fn is_greater_precedence(parser: &mut Parser, prec: usize) -> Option<Operator> {
  if let Ok(Token::Operator(op)) = parser.peek() {
    if Operator::is_binary(op) && op.get_prec() <= prec {
      return Some(op.clone());
    }
  }
  None
}

pub fn parse_expr_prec(parser: &mut Parser, expr: Expr, prec: usize) -> Result<Expr, ParserError> {
  let mut lhs = expr;
  while let Some(op) = is_greater_precedence(parser, prec) {
    parser.step();
    let mut rhs = Expr::Unary(parse_unary(parser)?);
    while let Some(op) = is_greater_precedence(parser, op.get_prec()) {
      rhs = parse_expr_prec(parser, rhs, op.get_prec())?;
    }
    lhs = Expr::Binary {
      lhs: Box::new(lhs),
      op,
      rhs: Box::new(rhs),
    }
  }
  Ok(lhs)
}

pub fn parse_unary(parser: &mut Parser) -> Result<Unary, ParserError> {
  if let Token::Operator(op) = parser.peek()? {
    if op.is_unary() {
      let op = op.clone();
      let expr = Box::new(parse_unary(parser)?);
      return Ok(Unary::UnaryOp { op, expr });
    }
  }
  Ok(Unary::Primary(parse_primary(parser)?))
}

pub fn parse_primary(parser: &mut Parser) -> Result<Primary, ParserError> {
  match parser.clone().peek()? {
    Token::Literal(lit) => parse_lit(parser, lit.to_owned()),
    Token::Punc(Punc::LeftParen) => parse_grouping(parser),
    Token::Ident(ident) => parse_ident_val(parser, ident.clone()),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedPrimaryExpression,
      parser.index,
    )),
  }
}

pub fn parse_grouping(parser: &mut Parser) -> Result<Primary, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let expr = parse_expr(parser)?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(Primary::Grouping(Box::new(expr)))
}

pub fn parse_cond(parser: &mut Parser) -> Result<Expr, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;
  let then_expr = parse_expr(parser)?;
  parser.eat_tok(Token::Keyword(Keyword::Else))?;
  let else_expr = parse_expr(parser)?;
  Ok(Expr::Cond {
    then_expr: Box::new(then_expr),
    cond: Box::new(cond),
    else_expr: Box::new(else_expr),
  })
}
