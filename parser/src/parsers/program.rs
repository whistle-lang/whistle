use crate::parse_assign;
use crate::parse_ident;
use crate::parse_ident_import;
use crate::parse_ident_type;
use crate::parse_ident_typed;
use crate::parse_stmt;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_program(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  match parser.peek() {
    Some(Token::Keyword(Keyword::Fun)) | Some(Token::Keyword(Keyword::Export)) => {
      parse_fun_decl(parser)
    }
    Some(Token::Keyword(Keyword::Import)) => parse_import(parser),
    Some(Token::Keyword(Keyword::Val)) => parse_val_decl(parser),
    Some(Token::Keyword(Keyword::Var)) => parse_var_decl(parser),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedProgramStmt,
      parser.index,
    )),
  }
}

pub fn parse_params(parser: &mut Parser) -> Option<Result<Vec<IdentTyped>, ParserError>> {
  if parser.eat_tok(Token::Punc(Punc::LeftParen)).is_some() {
    let mut idents = Vec::new();

    if let Ok(first) = parse_ident_typed(parser) {
      idents.push(first);

      for ident in &mut parser.repeating(|parser| {
        if parser.eat_tok(Token::Punc(Punc::Comma)).is_some() {
          Some(parse_ident_typed(parser))
        } else {
          None
        }
      }) {
        match ident {
          Ok(ident) => idents.push(ident.clone()),
          Err(err) => return Some(Err(err.clone())),
        }
      }
    }

    if parser.eat_tok(Token::Punc(Punc::RightParen)).is_some() {
      return Some(Ok(idents));
    }
  }

  None
}

pub fn parse_fun_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_some();

  if parser.eat_tok(Token::Keyword(Keyword::Fun)).is_some() {
    if let Some(ident) = parse_ident(parser) {
      let mut params = Vec::new();

      for param in parser.repeating(parse_params) {
        match param {
          Ok(param) => params.push(param),
          Err(err) => return Err(err)
        }
      }
      
      if parser.eat_tok(Token::Punc(Punc::Colon)).is_some() {
        if let Some(ret_type) = parse_ident_type(parser) {
          if let Ok(stmt) = parse_stmt(parser) {
            let stmt = Box::new(stmt);

            return Ok(ProgramStmt::FunDecl {
              export,
              ident,
              params,
              ret_type,
              stmt,
            });
          } else {
            Err(ParserError::new(
              ParserErrorKind::ExpectedFunBody,
              parser.index,
            ))
          }
        } else {
          Err(ParserError::new(
            ParserErrorKind::ExpectedReturnType,
            parser.index,
          ))
        }
      } else {
        Err(ParserError::new(
          ParserErrorKind::ExpectedReturnType,
          parser.index,
        ))
      }
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedFunIdent,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Fun),
      parser.index,
    ))
  }
}

pub fn parse_import(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Import)).is_some() {
    let mut idents = Vec::new();

    match parse_ident_import(parser) {
      Ok(first) => {
        idents.push(first);

        for ident in &mut parser.repeating(|parser| {
          if parser.eat_tok(Token::Punc(Punc::Comma)).is_some() {
            Some(parse_ident_import(parser))
          } else {
            None
          }
        }) {
          match ident {
            Ok(ident) => idents.push(ident.clone()),
            Err(err) => return Err(err.clone()),
          }
        }
        if parser.eat_tok(Token::Keyword(Keyword::From)).is_none() {
          return Err(ParserError::new(
            ParserErrorKind::ExpectedKeyword(Keyword::From),
            parser.index,
          ));
        }
      }
      Err(err) => {
        if err.kind != ParserErrorKind::ExpectedImportIdent {
          return Err(err);
        }
      }
    }

    if let Some(Token::StrLit(from)) = parser.eat_type(Token::StrLit(String::new())) {
      let from = from.to_owned();
      Ok(ProgramStmt::Import { idents, from })
    } else {
      Err(ParserError::new(
        ParserErrorKind::ExpectedImportLocation,
        parser.index,
      ))
    }
  } else {
    Err(ParserError::new(
      ParserErrorKind::ExpectedKeyword(Keyword::Import),
      parser.index,
    ))
  }
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
    let ident_typed = parse_ident_typed(parser)?;
    let assign = parse_assign(parser)?;
    let val = Box::new(assign);

    Ok(ProgramStmt::VarDecl { ident_typed, val })
  } else {
    Err(ParserError::new(ParserErrorKind::ExpectedKeyword(Keyword::Var), parser.index))
  }
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
    let ident_typed = parse_ident_typed(parser)?;
    let assign = parse_assign(parser)?;
    let val = Box::new(assign);

    Ok(ProgramStmt::ValDecl { ident_typed, val })
  } else {
    Err(ParserError::new(ParserErrorKind::ExpectedKeyword(Keyword::Val), parser.index))
  }
}
