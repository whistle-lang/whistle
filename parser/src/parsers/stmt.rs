use crate::parse_expr;
use crate::parse_ident_typed;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::Expr;
use whistle_ast::Stmt;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Tip;
use whistle_common::Token;

pub fn parse_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  match parser.peek() {
    Some(Token::Keyword(Keyword::If)) => parse_if_stmt(parser),
    Some(Token::Keyword(Keyword::While)) => parse_while_stmt(parser),
    Some(Token::Keyword(Keyword::Continue)) => parse_continue_stmt(parser),
    Some(Token::Keyword(Keyword::Break)) => parse_break_stmt(parser),
    Some(Token::Keyword(Keyword::Return)) => parse_return_stmt(parser),
    Some(Token::Keyword(Keyword::Var)) => parse_var_decl(parser),
    Some(Token::Keyword(Keyword::Val)) => parse_val_decl(parser),
    Some(Token::Tip(_)) => parse_tip(parser),
    Some(Token::Punc(Punc::LeftBrace)) => parse_block_stmt(parser),
    _ => parse_expr_stmt(parser),
  }
}

pub fn parse_if_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
    if let Some(cond) = parse_expr(parser) {
      let then_stmt = parse_stmt(parser)?;
      let cond = Box::new(cond);
      let then_stmt = Box::new(then_stmt);
      let mut else_stmt = None;

      if parser.eat_tok(Token::Keyword(Keyword::Else)).is_some() {
        else_stmt = Some(Box::new(parse_stmt(parser)?));
      }

      Ok(Stmt::If {
        cond,
        then_stmt,
        else_stmt,
      })
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedIfCondition,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::If),
      parser.index,
    ))
  }
}

pub fn parse_while_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::While)).is_some() {
    let cond = if let Some(cond) = parser.maybe(parse_expr) {
      Some(Box::new(cond))
    } else {
      None
    };

    if let Ok(do_stmt) = parse_stmt(parser) {
      let do_stmt = Box::new(do_stmt);

      Ok(Stmt::While { cond, do_stmt })
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedWhileBody,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::While),
      parser.index,
    ))
  }
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Continue)).is_some() {
    Ok(Stmt::Continue)
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Continue),
      parser.index,
    ))
  }
}

pub fn parse_break_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Break)).is_some() {
    Ok(Stmt::Break)
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Break),
      parser.index,
    ))
  }
}

pub fn parse_return_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Return)).is_some() {
    if let Some(expr) = parser.maybe(parse_expr) {
      return Ok(Stmt::Return(Some(Box::new(expr))));
    } else {
      return Ok(Stmt::Return(None));
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Return),
      parser.index,
    ))
  }
}

pub fn parse_assign(parser: &mut Parser) -> Result<Expr, ParserError> {
  if let Some(Token::Operator(op)) = parser.peek() {
    if op.is_assign() {
      parser.step();
      if let Some(expr) = parse_expr(parser) {
        Ok(expr)
      } else {
        Err(ParserError::new(
          ParserErrorKind::ExpectedExpression,
          parser.index,
        ))
      }
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedAssignment,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedOperator,
      parser.index,
    ))
  }
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
    let ident_typed = parse_ident_typed(parser)?;
    let assign = parse_assign(parser)?;
    let val = Box::new(assign);

    Ok(Stmt::VarDecl { ident_typed, val })
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Var),
      parser.index,
    ))
  }
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
    let ident_typed = parse_ident_typed(parser)?;
    let assign = parse_assign(parser)?;
    let val = Box::new(assign);

    Ok(Stmt::ValDecl { ident_typed, val })
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Val),
      parser.index,
    ))
  }
}

pub fn parse_tip(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if let Some(Token::Tip(tip)) = parser.eat_type(Token::Tip(Tip {
    ident: String::new(),
    value: String::new(),
  })) {
    Ok(Stmt::Tip(tip.to_owned()))
  } else {
    Err(ParserError::new(ParserErrorKind::ExpectedTip, parser.index))
  }
}

pub fn parse_block_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if parser.eat_tok(Token::Punc(Punc::LeftBrace)).is_some() {
    let mut stmts = Vec::new();

    loop {
      match parse_stmt(parser) {
        Ok(stmt) => stmts.push(stmt),
        Err(err) => {
          if err.kind == ParserErrorKind::ExpectedExpressionStatement {
            break;
          } else {
            return Err(err);
          }
        }
      }
    }

    if parser.eat_tok(Token::Punc(Punc::RightBrace)).is_some() {
      Ok(Stmt::Block(stmts))
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedBlockStmtEnd,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedBlockStmtStart,
      parser.index,
    ))
  }
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Result<Stmt, ParserError> {
  if let Some(expr) = parse_expr(parser) {
    Ok(Stmt::Expr(expr))
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedExpressionStatement,
      parser.index,
    ))
  }
}
