use crate::parse_assign;
use crate::parse_ident;
use crate::parse_ident_import;
use crate::parse_ident_type;
use crate::parse_ident_typed;
use crate::parse_stmt;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::Literal;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_common::Keyword;
use whistle_common::Punc;
use whistle_common::Token;

pub fn parse_program(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  match parser.peek()? {
    Token::Keyword(Keyword::Fun) | Token::Keyword(Keyword::Export) => {
      parse_fun_decl(parser)
    }
    Token::Keyword(Keyword::Import) => parse_import(parser),
    Token::Keyword(Keyword::Val) => parse_val_decl(parser),
    Token::Keyword(Keyword::Var) => parse_var_decl(parser),
    _ => Ok(ProgramStmt::Stmt(parse_stmt(parser)?))
    // _ => Err(ParserError::new(
    //   ParserErrorKind::ExpectedProgramStmt,
    //   parser.index,
    // )),
  }
}

pub fn parse_params(parser: &mut Parser) -> Result<Vec<IdentTyped>, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let mut idents = Vec::new();
  if let Some(first) = parser.maybe(parse_ident_typed) {
    idents.push(first);
    idents.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_ident_typed(parser)
    }));
  }
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(idents)
}

pub fn parse_fun_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Fun))?;
  let ident = parse_ident(parser)?;
  let params = parse_params(parser)?;
  parser.eat_tok(Token::Punc(Punc::Colon))?;
  let ret_type = parse_ident_type(parser)?;
  let stmt  = parse_stmt(parser)?;
  Ok(ProgramStmt::FunDecl {
    export, 
    ident, 
    params, 
    ret_type, 
    stmt: Box::new(stmt) 
  })
}

pub fn parse_import(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Import))?;
  let mut idents = Vec::new();
  if let Some(first) = parser.maybe(parse_ident_import) {
    idents.push(first);
    idents.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_ident_import(parser)
    }));
  }
  parser.eat_tok(Token::Keyword(Keyword::From))?;
  let from = parse_import_str(parser)?;
  Ok(ProgramStmt::Import { idents, from })
}

pub fn parse_import_str(parser: &mut Parser) -> Result<String, ParserError> {
  if let Token::Literal(Literal::Str(val)) = parser.clone().peek()? {
    parser.eat_type(Token::Literal(Literal::Str(val.clone())))?;
    return Ok(val.clone())
  }
  Err(ParserError::new(ParserErrorKind::ExpectedImportLocation, parser.index))
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  let assign = parse_assign(parser)?;
  Ok(ProgramStmt::VarDecl { ident_typed, val: Box::new(assign) })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  let assign = parse_assign(parser)?;
  Ok(ProgramStmt::ValDecl { ident_typed, val: Box::new(assign) })
}
