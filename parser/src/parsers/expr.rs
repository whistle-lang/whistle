use crate::parser::Parser;
use crate::parsers::ident::parse_ident;
use crate::parsers::literal::parse_lit;
use crate::parsers::operator::parse_binary_op;
use crate::parsers::operator::parse_unary_op;
use crate::error::ParserError;
use crate::error::ParserErrorKind;

use whistle_ast::Expr;
use whistle_ast::Operand;
use whistle_ast::Primary;
use whistle_ast::Unary;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;
use whistle_common::Operator;

pub fn parse_expr(parser: &mut Parser) -> Result<Expr, ParserError> {
  let expr = parse_expr_prec(parser, usize::MAX)?;
  if parser.is_tok(Token::Keyword(Keyword::If)) {
    return parse_cond(parser, expr)
  }
  Ok(expr)
}

pub fn is_greater_precedence(tok: &Token, prec: usize) -> bool {
  if let Token::Operator(op) = tok {
    return Operator::is_binary(op) && op.get_prec() <= prec
  }
  false
}

pub fn parse_expr_prec(parser: &mut Parser, prec: usize) -> Result<Expr, ParserError> {
  let mut lhs = Expr::Unary(parse_unary(parser)?);
  while is_greater_precedence(parser.peek()?, prec) {
    lhs = parse_binary(parser, lhs.to_owned())?;
  }
  Ok(lhs)
}

pub fn parse_unary(parser: &mut Parser) -> Result<Unary, ParserError> {
  if let Token::Operator(op) = parser.peek()? {
    if op.is_unary() {
      return Ok(parse_unary_operation(parser)?)
    }
    //Expected unary operator
  }
  Ok(Unary::Primary(parse_primary(parser)?))
}

pub fn parse_unary_operation(parser: &mut Parser) -> Result<Unary, ParserError> {
  let op = parse_unary_op(parser)?;
  let expr = parse_unary(parser)?;
  Ok(Unary::UnaryOp { op, expr: Box::new(expr) })
}

pub fn parse_primary(parser: &mut Parser) -> Result<Primary, ParserError> {
  let prim = parse_operand(parser)?;
  parse_primary_prim(parser, prim)
}

pub fn parse_primary_prim(parser: &mut Parser, prim: Primary) -> Result<Primary, ParserError> {
  let prim = match parser.peek()? {
    Token::Punc(Punc::Dot) => parse_selector(parser, prim.to_owned())?,
    Token::Punc(Punc::LeftParen) => parse_arguments(parser, prim.to_owned())?,
    _ => return Ok(prim)
  };
  parse_primary_prim(parser, prim)
}

pub fn parse_operand(parser: &mut Parser) -> Result<Primary, ParserError> {
  let op = match parser.clone().peek()? {
    Token::Literal(lit) => parse_lit(parser, lit.to_owned())?,
    Token::Punc(Punc::LeftParen) => parse_grouping(parser)?,
    Token::Ident(_) => Operand::Ident(parse_ident(parser)?),
    _ => return Err(ParserError::new(ParserErrorKind::ExpectedOperand, parser.index))
  };
  Ok(Primary::Operand(op))
}

pub fn parse_grouping(parser: &mut Parser) -> Result<Operand, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let expr = parse_expr(parser)?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(Operand::Grouping(Box::new(expr)))
}

pub fn parse_selector(parser: &mut Parser, prim: Primary) -> Result<Primary, ParserError> {
  parser.eat_tok(Token::Punc(Punc::Dot))?;
  let ident = parse_ident(parser)?;
  Ok(Primary::Selector { prim: Box::new(prim), ident })
}

pub fn parse_arguments(parser: &mut Parser, prim: Primary) -> Result<Primary, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let mut args = Vec::new();
  if let Some(first) = parser.maybe(parse_expr) {
    args.push(first);
    args.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_expr(parser)
    }));
  }
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(Primary::Arguments { prim: Box::new(prim), args })
}

pub fn parse_binary(parser: &mut Parser, lhs: Expr) -> Result<Expr, ParserError> {
  let op = parse_binary_op(parser)?;
  let rhs = parse_expr_prec(parser, op.get_prec())?;
  Ok(Expr::Binary { 
    lhs: Box::new(lhs), 
    op, 
    rhs: Box::new(rhs) 
  })
}

pub fn parse_cond(parser: &mut Parser, then_expr: Expr) -> Result<Expr, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;
  parser.eat_tok(Token::Keyword(Keyword::Else))?;
  let else_expr = parse_expr(parser)?;
  Ok(Expr::Cond {
    then_expr: Box::new(then_expr),
    cond: Box::new(cond),
    else_expr: Box::new(else_expr),
  })
}
