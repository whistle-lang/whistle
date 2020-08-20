use super::ident::*;
use super::literal::*;
pub use crate::lexer::*;
use crate::parser::ast::*;
use crate::parser::parser::*;

pub fn parse_expr(parser: &mut Parser) -> Option<Expr> {
  //TODO: parse_conditionals
  if let Some(expr) = parse_unary_expr(parser) {
    let previous = Expr::Unary(expr);
    if let Some(Token::Operator(operator)) = parser.peek() {
      if Operator::is_binary(operator) {
        return parse_binary_expr(parser, previous);
      }
    }
    return Some(previous);
  }
  None
}

pub fn parse_unary_expr(parser: &mut Parser) -> Option<UnaryExpr> {
  if let Some(operator) = parse_unary_operator(parser) {
    if let Some(expr) = parse_unary_expr(parser) {
      return Some(UnaryExpr::UnaryOp {
        op: operator,
        expr: Box::new(expr),
      });
    }
  }
  parse_primary_expr(parser)
}

pub fn parse_binary_expr(parser: &mut Parser, previous: Expr) -> Option<Expr> {
  if let Some(operator) = parse_binary_operator(parser) {
    if let Some(expr) = parse_expr(parser) {
      return Some(Expr::Binary {
        lhs: Box::new(previous),
        op: operator,
        //TODO: precedence
        rhs: Box::new(expr),
      });
    }
  }
  None
}

pub fn parse_primary_expr(parser: &mut Parser) -> Option<UnaryExpr> {
  if let Some(current) = parser.peek() {
    //TODO: parse_selector, parse_index, parse_parse_slice
    let primary = match current {
      Token::BoolLit(_current) => parse_boolean_literal(parser),
      Token::IntLit(_current) => parse_integer_literal(parser),
      Token::FloatLit(_current) => parse_float_literal(parser),
      Token::CharLit(_current) => parse_char_literal(parser),
      Token::StrLit(_current) => parse_str_literal(parser),
      Token::Ident(_current) => {
        if parser.is_tok_eq(Token::Punc(Punc::LeftParen), 1) {
          parse_function_call(parser)
        } else {
          parse_variable_access(parser)
        }
      }
      Token::Punc(Punc::LeftParen) => parse_grouping(parser),
      _ => {
        println!("Could not parse expression {:?}", current);
        None
      }
    };
    if let Some(primary) = primary {
      return Some(UnaryExpr::Primary(primary));
    }
  }
  None
}

pub fn parse_function_call(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(name) = parse_ident(parser) {
    if parser.eat_type(Token::Punc(Punc::LeftParen)).is_some() {
      let exprs = parser.until_is(
        |parser| {
          let expr = parse_expr(parser);
          parser.eat_type(Token::Punc(Punc::Comma));
          expr
        },
        Token::Punc(Punc::RightBrace),
      );

      if parser.eat_type(Token::Punc(Punc::RightParen)).is_some() {
        return Some(PrimaryExpr::Arguments {
          prim: Box::new(PrimaryExpr::Operand(Operand::Ident(name))),
          args: exprs,
        });
      }
    }
  }
  None
}

pub fn parse_variable_access(parser: &mut Parser) -> Option<PrimaryExpr> {
  if let Some(name) = parse_ident(parser) {
    return Some(PrimaryExpr::Operand(Operand::Ident(name)));
  }
  None
}

pub fn parse_grouping(parser: &mut Parser) -> Option<PrimaryExpr> {
  if parser.eat_type(Token::Punc(Punc::LeftParen)).is_some() {
    if let Some(expr) = parse_expr(parser) {
      if parser.eat_type(Token::Punc(Punc::RightParen)).is_some() {
        return Some(PrimaryExpr::Operand(Operand::Grouping(Box::new(expr))));
      }
    }
  }
  None
}

pub fn parse_binary_operator(parser: &mut Parser) -> Option<Operator> {
  if let Some(Token::Operator(operator)) = parser.eat_type(Token::Operator(Operator::Add)) {
    if Operator::is_binary(operator) {
      return Some(operator.clone());
    }
  }
  None
}

pub fn parse_unary_operator(parser: &mut Parser) -> Option<Operator> {
  if let Some(Token::Operator(operator)) = parser.eat_type(Token::Operator(Operator::Add)) {
    if Operator::is_unary(operator) {
      return Some(operator.clone());
    }
  }
  None
}
