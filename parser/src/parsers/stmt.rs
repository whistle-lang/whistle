use crate::eat_type;
use crate::parse_cond;
use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;
use whistle_common::ParserError;

use whistle_ast::Expr;
use whistle_ast::Primary;
use whistle_ast::Stmt;

use whistle_ast::Unary;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Span;
use whistle_common::Token;

pub fn parse_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  match parser.peek()?.token {
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
  let index = parser.index;
  let start = parser.peek()?.span.start;

  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;

  let then_stmt = match parser.maybe(parse_stmts) {
    Some(stmts) => stmts,
    None => {
      let start = parser.peek()?.span.start;
      parser.index = index;
      let expr = parse_cond(parser)?;
      let end = parser.peek_offset(-1)?.span.end;
      return Ok(Stmt::Expr {
        expr,
        span: Span { start, end },
      });
    }
  };
  let mut else_stmt = None;
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_ok() {
    else_stmt = Some(parse_stmts(parser)?);
  }
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::If {
    cond,
    then_stmt,
    else_stmt,
    span: Span { start, end },
  })
}

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::While))?;
  let cond = parse_expr(parser)?;
  let do_stmt = parse_stmts(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::While {
    cond,
    do_stmt,
    span: Span { start, end },
  })
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let span = parser.peek()?.span;
  parser.eat_tok(Token::Keyword(Keyword::Continue))?;
  Ok(Stmt::Continue { span })
}

pub fn parse_break_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let span = parser.peek()?.span;
  parser.eat_tok(Token::Keyword(Keyword::Break))?;
  Ok(Stmt::Break { span })
}

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Return))?;
  let ret_type = parser.maybe(parse_expr);
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::Return {
    ret_type,
    span: Span { start, end },
  })
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::VarDecl {
    ident_typed,
    val,
    span: Span { start, end },
  })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::ValDecl {
    ident_typed,
    val,
    span: Span { start, end },
  })
}

pub fn parse_tip(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let span = parser.peek()?.span;
  let tip = eat_type!(parser, Token::Tip)?;
  Ok(Stmt::Tip { tip, span })
}

pub fn parse_block_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  let stmts = parse_stmts(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(Stmt::Block {
    stmts,
    span: Span { start, end },
  })
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.span.start;
  let expr = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  if let Expr::Binary { op, lhs, rhs, span } = expr.clone() {
    if op.clone() == Operator::Assign {
      if let Expr::Unary {
        unary:
          Unary::Primary {
            prim: Primary::IdentVal { ident, .. },
            ..
          },
        ..
      } = (*lhs).clone()
      {
        return Ok(Stmt::Assign {
          ident,
          rhs: *rhs,
          span,
        });
      }
    }
  }
  Ok(Stmt::Expr {
    expr,
    span: Span { start, end },
  })
}
