use crate::eat_type;
use crate::parse_cond;
use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;
use crate::ParserError;

use whistle_ast::Expr;
use whistle_ast::Primary;
use whistle_ast::Stmt;

use whistle_ast::Unary;
use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Range;
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
  let start = parser.peek()?.range.start;

  parser.eat_tok(Token::Keyword(Keyword::If))?;
  let cond = parse_expr(parser)?;

  let then_stmt = match parser.maybe(parse_stmts) {
    Some(stmts) => stmts,
    None => {
      let start = parser.peek()?.range.start;
      parser.index = index;
      let expr = parse_cond(parser)?;
      let end = parser.peek_offset(-1)?.range.end;
      return Ok(Stmt::Expr {
        expr,
        range: Range { start, end },
      });
    }
  };
  let mut else_stmt = None;
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_ok() {
    else_stmt = Some(parse_stmts(parser)?);
  }
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::If {
    cond,
    then_stmt,
    else_stmt,
    range: Range { start, end },
  })
}

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Keyword(Keyword::While))?;
  let cond = parse_expr(parser)?;
  let do_stmt = parse_stmts(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::While {
    cond,
    do_stmt,
    range: Range { start, end },
  })
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let range = parser.peek()?.range;
  parser.eat_tok(Token::Keyword(Keyword::Continue))?;
  Ok(Stmt::Continue { range })
}

pub fn parse_break_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let range = parser.peek()?.range;
  parser.eat_tok(Token::Keyword(Keyword::Break))?;
  Ok(Stmt::Break { range })
}

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Keyword(Keyword::Return))?;
  let ret_type = parser.maybe(parse_expr);
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::Return {
    ret_type,
    range: Range { start, end },
  })
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::VarDecl {
    ident_typed,
    val,
    range: Range { start, end },
  })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::ValDecl {
    ident_typed,
    val,
    range: Range { start, end },
  })
}

pub fn parse_tip(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let range = parser.peek()?.range;
  let tip = eat_type!(parser, Token::Tip)?;
  Ok(Stmt::Tip { tip, range })
}

pub fn parse_block_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  let stmts = parse_stmts(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  Ok(Stmt::Block {
    stmts,
    range: Range { start, end },
  })
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  let start = parser.peek()?.range.start;
  let expr = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.range.end;
  if let Expr::Binary {
    op,
    lhs,
    rhs,
    range,
  } = expr.clone()
  {
    if op.clone() == Operator::Assign {
      if let Expr::Unary { unary, .. } = (*lhs).clone() {
        if let Unary::Primary { prim, .. } = unary {
          if let Primary::IdentVal { ident, .. } = prim {
            return Ok(Stmt::Assign {
              ident,
              rhs: *rhs,
              range,
            });
          }
        }
      }
    }
  }
  Ok(Stmt::Expr {
    expr,
    range: Range { start, end },
  })
}
