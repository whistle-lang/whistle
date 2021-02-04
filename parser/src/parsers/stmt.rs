use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorExtend;
use crate::ParserErrorKind;

use whistle_ast::Expr;
use whistle_ast::Stmt;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Tip;
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

pub fn parse_if_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser).extend(ParserErrorKind::ExpectedIfCondition)?;
  let then_stmt = parse_stmt(parser)?;
  let then_stmt = Box::new(then_stmt);
  let mut else_stmt = None;
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_ok() {
    else_stmt = Some(Box::new(parse_stmt(parser)?));
  }
  Ok(Stmt::If {
    cond: Box::new(cond),
    then_stmt,
    else_stmt,
  })
}

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::While))?;
  let cond = match parse_expr(parser) {
    Ok(cond) => Some(Box::new(cond)),
    Err(_) => None,
  };
  let do_stmt = parse_stmt(parser)?;
  Ok(Stmt::While {
    cond,
    do_stmt: Box::new(do_stmt),
  })
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
  Ok(Stmt::Return(match parse_expr(parser) {
    Ok(expr) => Some(Box::new(expr)),
    Err(_) => None,
  }))
}

pub fn parse_assign(parser: &mut Parser) -> Result<Expr, ParserError> {
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  parse_expr(parser)
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  let assign = parse_assign(parser)?;
  Ok(Stmt::VarDecl {
    ident_typed,
    val: Box::new(assign),
  })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  let assign = parse_assign(parser)?;
  Ok(Stmt::ValDecl {
    ident_typed,
    val: Box::new(assign),
  })
}

pub fn parse_tip(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if let Token::Tip(tip) = parser.eat_type(Token::Tip(Tip {
    ident: String::new(),
    value: String::new(),
  }))? {
    return Ok(Stmt::Tip(tip.to_owned()));
  }
  Err(ParserError::new(ParserErrorKind::ExpectedTip, parser.index))
}

pub fn parse_block_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftBrace))?;
  let mut stmts = Vec::new();
  if let Some(first) = parser.maybe(parse_stmt) {
    stmts.push(first);
    stmts.append(&mut parser.eat_repeat(parse_stmt));
  };
  parser.eat_tok(Token::Punc(Punc::RightBrace))?;
  Ok(Stmt::Block(stmts))
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let expr = parse_expr(parser)?;
  Ok(Stmt::Expr(expr))
}
