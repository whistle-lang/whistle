use crate::parse_assign;
use crate::parse_ident;
use crate::parse_ident_import;
use crate::parse_ident_type;
use crate::parse_ident_typed;
use crate::parse_stmt;
use crate::parser::Parser;

use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_lexer::Keyword;
use whistle_lexer::Punc;
use whistle_lexer::Token;

pub fn parse_program(parser: &mut Parser) -> Option<ProgramStmt> {
  match parser.peek() {
    Some(Token::Keyword(Keyword::Fun)) => parse_fun_decl(parser),
    Some(Token::Keyword(Keyword::Import)) => parse_import(parser),
    Some(Token::Keyword(Keyword::Val)) => parse_val_decl(parser),
    Some(Token::Keyword(Keyword::Var)) => parse_var_decl(parser),
    _ => None,
  }
}

pub fn parse_params(parser: &mut Parser) -> Option<Vec<IdentTyped>> {
  if parser.eat_tok(Token::Punc(Punc::LeftParen)).is_some() {
    let mut idents = Vec::new();

    if let Some(first) = parse_ident_typed(parser) {
      idents.push(first);

      idents.append(&mut parser.repeating(|parser| {
        if parser.eat_tok(Token::Punc(Punc::Comma)).is_some() {
          parse_ident_typed(parser)
        } else {
          None
        }
      }));
    }

    if parser.eat_tok(Token::Punc(Punc::RightParen)).is_some() {
      return Some(idents);
    }
  }

  None
}

pub fn parse_fun_decl(parser: &mut Parser) -> Option<ProgramStmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Fun)).is_some() {
    if let Some(ident) = parse_ident(parser) {
      let params = parser.repeating(parse_params);

      if parser.eat_tok(Token::Punc(Punc::Colon)).is_some() {
        if let Some(ret_type) = parse_ident_type(parser) {
          if let Some(stmt) = parse_stmt(parser) {
            let stmt = Box::new(stmt);

            return Some(ProgramStmt::FunDecl {
              ident,
              params,
              ret_type,
              stmt,
            });
          }
        }
      }
    }
  }

  None
}

pub fn parse_import(parser: &mut Parser) -> Option<ProgramStmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Import)).is_some() {
    let mut idents = Vec::new();

    if let Some(first) = parse_ident_import(parser) {
      idents.push(first);

      idents.append(&mut parser.repeating(|parser| {
        if parser.eat_tok(Token::Punc(Punc::Comma)).is_some() {
          parse_ident_import(parser)
        } else {
          None
        }
      }));

      parser.eat_tok(Token::Keyword(Keyword::From))?;
    }

    if let Some(Token::StrLit(from)) = parser.eat_type(Token::StrLit(String::new())) {
      let from = from.to_owned();
      return Some(ProgramStmt::Import { idents, from });
    }
  }

  None
}

pub fn parse_var_decl(parser: &mut Parser) -> Option<ProgramStmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
    if let Some(ident_typed) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        let val = Box::new(value);

        return Some(ProgramStmt::VarDecl { ident_typed, val });
      }
    }
  }

  None
}

pub fn parse_val_decl(parser: &mut Parser) -> Option<ProgramStmt> {
  if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
    if let Some(ident_typed) = parse_ident_typed(parser) {
      if let Some(value) = parse_assign(parser) {
        let val = Box::new(value);

        return Some(ProgramStmt::ValDecl { ident_typed, val });
      }
    }
  }

  None
}
