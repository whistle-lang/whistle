use super::ident::*;
use super::literal::*;
use super::operator::*;
use crate::lexer::*;
use crate::parser::ast::*;
use crate::parser::parser::*;

pub fn parse_expr(parser: &mut Parser) -> Option<Expr> {
  let expr = parse_expr_prec(parser, usize::MAX);

  if let Some(expr) = expr {
    if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
      parse_cond(parser, expr)
    } else {
      Some(expr)
    }
  } else {
    None
  }
}

pub fn parse_expr_prec(parser: &mut Parser, prec: usize) -> Option<Expr> {
  if let Some(mut lhs) = parser.maybe(parse_unary) {
    while let Some(op) = parser.check(parse_binary_op) {
      if op.get_prec() <= prec {
        if let Some(bin) = parser.maybe(|parser| parse_binary(parser, lhs.to_owned())) {
          lhs = bin;
        } else {
          return None;
        }
      } else {
        break;
      }
    }

    // println!("{:?}", lhs);
    return Some(lhs);
  }

  None
}

pub fn parse_unary(parser: &mut Parser) -> Option<Expr> {
  if let Some(prim) = parser.maybe(parse_primary) {
    Some(Expr::Unary(Unary::Primary(prim)))
  } else if let Some(expr) = parser.maybe(parse_unary_operation) {
    Some(Expr::Unary(expr))
  } else {
    None
  }
}

pub fn parse_unary_operation(parser: &mut Parser) -> Option<Unary> {
  if let Some(op) = parse_unary_op(parser) {
    if let Some(Expr::Unary(expr)) = parse_unary(parser) {
      let expr = Box::new(expr);

      return Some(Unary::UnaryOp { op, expr });
    }
  }

  None
}

pub fn parse_primary(parser: &mut Parser) -> Option<Primary> {
  if let Some(prim) = parser.maybe(parse_operand) {
    let prim = Primary::Operand(prim);

    parse_primary_prim(parser, prim)
  } else {
    None
  }
}

pub fn parse_primary_prim(parser: &mut Parser, prim: Primary) -> Option<Primary> {
  if let Some(selector) = parser.maybe(|parser| parse_selector(parser, prim.to_owned())) {
    parse_primary_prim(parser, selector)
  } else if let Some(arguments) = parser.maybe(|parser| parse_arguments(parser, prim.to_owned())) {
    parse_primary_prim(parser, arguments)
  } else {
    Some(prim)
  }
}

pub fn parse_operand(parser: &mut Parser) -> Option<Operand> {
  if let Some(lit) = parse_lit(parser) {
    Some(Operand::Literal(lit))
  } else if let Some(ident) = parse_ident(parser) {
    Some(Operand::Ident(ident))
  } else if let Some(grouping) = parser.maybe(parse_grouping) {
    Some(Operand::Grouping(grouping))
  } else {
    None
  }
}

pub fn parse_grouping(parser: &mut Parser) -> Option<Box<Expr>> {
  if parser.eat_tok(Token::Punc(Punc::LeftParen)).is_some() {
    if let Some(expr) = parse_expr(parser) {
      if parser.eat_tok(Token::Punc(Punc::RightParen)).is_some() {
        return Some(Box::new(expr));
      }
    }
  }

  None
}

pub fn parse_selector(parser: &mut Parser, prim: Primary) -> Option<Primary> {
  if parser.eat_tok(Token::Punc(Punc::Dot)).is_some() {
    if let Some(ident) = parse_ident(parser) {
      let prim = Box::new(prim);

      return Some(Primary::Selector { prim, ident });
    }
  }

  None
}

pub fn parse_arguments(parser: &mut Parser, prim: Primary) -> Option<Primary> {
  if parser.eat_tok(Token::Punc(Punc::LeftParen)).is_some() {
    let mut args = Vec::new();

    if let Some(first) = parse_expr(parser) {
      args.push(first);
      args.append(&mut parser.repeating(|parser| {
        if parser.eat_tok(Token::Punc(Punc::Comma)).is_some() {
          parse_expr(parser)
        } else {
          None
        }
      }));
    }
    if parser.eat_tok(Token::Punc(Punc::RightParen)).is_some() {
      let prim = Box::new(prim);

      return Some(Primary::Arguments { prim, args });
    }
  }

  None
}

pub fn parse_binary(parser: &mut Parser, lhs: Expr) -> Option<Expr> {
  if let Some(op) = parse_binary_op(parser) {
    if let Some(rhs) = parse_expr_prec(parser, op.get_prec()) {
      let lhs = Box::new(lhs);
      let rhs = Box::new(rhs);
      return Some(Expr::Binary { lhs, op, rhs });
    }
  }

  None
}

pub fn parse_cond(parser: &mut Parser, then_expr: Expr) -> Option<Expr> {
  if let Some(cond) = parse_expr(parser) {
    if parser.eat_tok(Token::Keyword(Keyword::Else)).is_some() {
      if let Some(else_expr) = parse_expr(parser) {
        let then_expr = Box::new(then_expr);
        let cond = Box::new(cond);
        let else_expr = Box::new(else_expr);
        return Some(Expr::Cond {
          then_expr,
          cond,
          else_expr,
        });
      }
    }
  }

  None
}
