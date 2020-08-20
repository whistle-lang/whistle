use super::expression::*;
use super::ident::*;
use crate::parser::ast::*;
use crate::parser::Parser;

pub fn parse_stmt(parser: &mut Parser) -> Option<Stmt> {
  //TODO: parse_ident
  match parser.peek() {
    Some(Token::Tip(_tip)) => parse_tip(parser),
    Some(Token::Keyword(keyword)) => match keyword {
      Keyword::If => parse_if_stmt(parser),
      Keyword::Return => parse_return_stmt(parser),
      Keyword::Var => parse_var_decl(parser),
      Keyword::Val => parse_val_decl(parser),
      Keyword::While => parse_while_stmt(parser),
      Keyword::Continue => parse_continue_stmt(parser),
      Keyword::Break => parse_break_stmt(parser),
      Keyword::Fun => parse_fun_decl(parser),
      _ => {
        println!("Could not parse statement {:?}", keyword);
        None
      }
    },
    Some(Token::Punc(Punc::LeftBrace)) => parse_block_stmt(parser),
    _ => parse_expr_stmt(parser),
  }
}

//TODO: parse_tip
pub fn parse_tip(_parser: &mut Parser) -> Option<Stmt> {
  Some(Stmt::Continue)
}

pub fn parse_if_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
    if let Some(cond) = parse_expr(parser) {
      if let Some(then_stmt) = parse_stmt(parser) {
        return Some(Stmt::If {
          cond: Box::new(cond),
          then_stmt: Box::new(then_stmt),
          else_stmt: parse_else_stmt(parser),
        });
      }
    }
  }
  None
}

pub fn parse_else_stmt(parser: &mut Parser) -> Option<Box<Stmt>> {
  if parser.eat_tok(Token::Keyword(Keyword::Else)).is_some() {
    if let Some(else_stmt) = parse_stmt(parser) {
      return Some(Box::new(else_stmt));
    }
  }
  None
}

pub fn parse_return_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Return)).is_some() {
    if let Some(expr) = parse_expr(parser) {
      return Some(Stmt::Return(Some(Box::new(expr))));
    }
  }
  None
}

pub fn parse_var_decl(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
    if let Some(ident) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        return Some(Stmt::VarDecl {
          ident_typed: ident,
          val: Box::new(value),
        });
      }
    }
  }
  None
}

pub fn parse_assign(parser: &mut Parser) -> Option<Expr> {
  if parser.eat_tok(Token::Operator(Operator::Assign)).is_some() {
    return parse_expr(parser);
  }
  None
}

pub fn parse_val_decl(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
    if let Some(ident) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        return Some(Stmt::ValDecl {
          ident_typed: ident,
          val: Box::new(value),
        });
      }
    }
  }
  None
}

pub fn parse_while_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
    if let Some(cond) = parse_expr(parser) {
      if let Some(do_stmt) = parse_stmt(parser) {
        return Some(Stmt::While {
          cond: Some(Box::new(cond)),
          do_stmt: Box::new(do_stmt),
        });
      }
    }
  }
  None
}

pub fn parse_continue_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Continue)).is_some() {
    return Some(Stmt::Continue);
  }
  None
}

pub fn parse_break_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Break)).is_some() {
    return Some(Stmt::Break);
  }
  None
}

pub fn parse_block_stmt(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Punc(Punc::LeftBrace)).is_some() {
    let mut stmts: Vec<Stmt> = Vec::new();

    while !parser.is_tok_eq(Token::Punc(Punc::RightBrace), 0) {
      if let Some(stmt) = parse_stmt(parser) {
        stmts.push(stmt)
      }
    }
    if parser.eat_tok(Token::Punc(Punc::RightBrace)).is_some() {
      return Some(Stmt::Block(stmts));
    }
  }
  None
}

pub fn parse_expr_stmt(parser: &mut Parser) -> Option<Stmt> {
  if let Some(expr) = parse_expr(parser) {
    return Some(Stmt::Expr(expr));
  }
  None
}

pub fn parse_fun_decl(parser: &mut Parser) -> Option<Stmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Fun)).is_some() {
    if let Some(identifier) = parse_ident(parser) {
      if let Some(parameters) = parse_params(parser) {
        if let Some(r#type) = parse_ident(parser) {
          if let Some(stmt) = parse_stmt(parser) {
            return Some(Stmt::FunDecl {
              ident: identifier,
              params: parameters,
              ret_type: r#type,
              stmt: Box::new(stmt),
            });
          }
        }
      }
    }
  }
  None
}

pub fn parse_params(parser: &mut Parser) -> Option<Vec<IdentTyped>> {
  if parser.eat_type(Token::Punc(Punc::LeftParen)).is_some() {
    let exprs = parser.until_is(
      |parser| {
        let expr = parse_ident_typed(parser);
        parser.eat_type(Token::Punc(Punc::Comma));
        expr
      },
      Token::Punc(Punc::RightBrace),
    );

    if parser.eat_type(Token::Punc(Punc::RightParen)).is_some() {
      return Some(exprs);
    }
  }
  None
}
