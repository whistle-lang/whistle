use crate::eat_type;
use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::Stmt;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  match parser.peek()? {
    Token::Keyword(Keyword::If) => parse_if_stmt(parser),
    Token::Keyword(Keyword::While) => parse_while_stmt(parser),
    Token::Keyword(Keyword::Continue) => parse_continue_stmt(parser),
    Token::Keyword(Keyword::Break) => parse_break_stmt(parser),
    Token::Keyword(Keyword::Return) => parse_return_stmt(parser),
    Token::Keyword(Keyword::Var) => parse_var_decl(parser),
    Token::Keyword(Keyword::Val) => parse_val_decl(parser),
    Token::Tip(_) => parse_tip(parser),
    Token::Punc(Punc::LeftBrace) => parse_block_stmt(parser),
    _ => parse_expr_stmt(parser),
  }
}

pub fn parse_stmts(parser: &mut Parser) -> Result<Vec<Stmt>, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftBrace))?;
  let stmts = parser.eat_repeat(parse_stmt, None, Token::Punc(Punc::RightBrace))?;
  parser.eat_tok(Token::Punc(Punc::RightBrace))?;
  Ok(stmts)
}

pub fn parse_if_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;
  let then_stmt = parse_stmts(parser)?;
  let mut else_stmt = None;
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_ok() {
    else_stmt = Some(parse_stmts(parser)?);
  }
  Ok(Stmt::If {
    cond,
    then_stmt,
    else_stmt,
  })
}

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::While))?;
  let cond = parse_expr(parser)?;
  let do_stmt = parse_stmts(parser)?;
  Ok(Stmt::While { cond, do_stmt })
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Continue))?;
  Ok(Stmt::Continue)
}

pub fn parse_break_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Break))?;
  Ok(Stmt::Break)
}

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Return))?;
  let ret_type = parser.maybe(parse_expr);
  Ok(Stmt::Return(ret_type))
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  Ok(Stmt::VarDecl { ident_typed, val })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  Ok(Stmt::ValDecl { ident_typed, val })
}

pub fn parse_tip(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let tip = eat_type!(parser, Token::Tip)?;
  Ok(Stmt::Tip(tip))
}

pub fn parse_block_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let stmts = parse_stmts(parser)?;
  Ok(Stmt::Block(stmts))
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let expr = parse_expr(parser)?;
  Ok(Stmt::Expr(expr))
}
