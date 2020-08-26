use crate::lexer::Keyword;
use crate::lexer::Token;
use crate::parser::ast::*;
use crate::parser::parser::*;

pub fn parse_bool_lit(parser: &mut Parser) -> Option<Literal> {
  if let Some(Token::BoolLit(boolean)) = parser.eat_type(Token::BoolLit(true)) {
    Some(Literal::Bool(*boolean))
  } else {
    None
  }
}

pub fn parse_int_lit(parser: &mut Parser) -> Option<Literal> {
  if let Some(Token::IntLit(integer)) = parser.eat_type(Token::IntLit(0)) {
    Some(Literal::Int(*integer))
  } else {
    None
  }
}

pub fn parse_float_lit(parser: &mut Parser) -> Option<Literal> {
  if let Some(Token::FloatLit(float)) = parser.eat_type(Token::FloatLit(0.0)) {
    Some(Literal::Float(*float))
  } else {
    None
  }
}

pub fn parse_char_lit(parser: &mut Parser) -> Option<Literal> {
  if let Some(Token::CharLit(character)) = parser.eat_type(Token::CharLit('a')) {
    Some(Literal::Char(*character))
  } else {
    None
  }
}

pub fn parse_str_lit(parser: &mut Parser) -> Option<Literal> {
  if let Some(Token::StrLit(string)) = parser.eat_type(Token::StrLit(String::new())) {
    Some(Literal::Str(string.to_string()))
  } else {
    None
  }
}

pub fn parse_none_lit(parser: &mut Parser) -> Option<Literal> {
  if parser.eat_tok(Token::Keyword(Keyword::None)).is_some() {
    Some(Literal::None)
  } else {
    None
  }
}

pub fn parse_lit(parser: &mut Parser) -> Option<Literal> {
  match parser.peek() {
    Some(Token::BoolLit(_)) => parse_bool_lit(parser),
    Some(Token::IntLit(_)) => parse_int_lit(parser),
    Some(Token::FloatLit(_)) => parse_float_lit(parser),
    Some(Token::CharLit(_)) => parse_char_lit(parser),
    Some(Token::StrLit(_)) => parse_str_lit(parser),
    Some(Token::Keyword(Keyword::None)) => parse_none_lit(parser),
    _ => None,
  }
}
