use crate::parse_expr;
use crate::parse_ident;
use crate::parse_ident_import;
use crate::parse_ident_type;
use crate::parse_ident_typed;
use crate::parse_stmt;
use crate::eat_type;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentTyped;
use whistle_ast::Literal;
use whistle_ast::ProgramStmt;
use whistle_common::Operator;
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
    Token::Keyword(Keyword::Struct) => parse_struct_decl(parser),
    Token::Keyword(Keyword::Type) => parse_type_decl(parser),
    _ => Ok(ProgramStmt::Stmt(parse_stmt(parser)?))
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

pub fn parse_type_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Type))?;
  let ident = parse_ident(parser)?;
  let mut params = Vec::new();
  if let Some(first) = parser.maybe(parse_ident_typed) {
    params.push(first);
    params.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_ident_typed(parser)
    }));
  }
  Ok(ProgramStmt::StructDecl { export, ident, params })
}

pub fn parse_struct_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Struct))?;
  let ident = parse_ident(parser)?;
  let mut params = Vec::new();
  if let Some(first) = parser.maybe(parse_ident_typed) {
    params.push(first);
    params.append(&mut parser.eat_repeat(|parser| {
      parser.eat_tok(Token::Punc(Punc::Comma))?;
      parse_ident_typed(parser)
    }));
  }
  Ok(ProgramStmt::StructDecl { export, ident, params })
}

pub fn parse_fun_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Fun))?;
  let ident = parse_ident(parser)?;
  let params = parse_params(parser)?;
  let mut ret_type = None;
  if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    ret_type = Some(parse_ident_type(parser)?);
  }
  let stmt = parse_stmt(parser)?;
  Ok(ProgramStmt::FunDecl {
    export,
    ident,
    params,
    ret_type,
    stmt: Box::new(stmt),
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
  let from = eat_type!(parser, Token::Literal(Literal::Str))?;
  Ok(ProgramStmt::Import { idents, from })
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let assign = parse_expr(parser)?;
  Ok(ProgramStmt::VarDecl {
    ident_typed,
    val: Box::new(assign),
  })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let assign = parse_expr(parser)?;
  Ok(ProgramStmt::ValDecl {
    ident_typed,
    val: Box::new(assign),
  })
}
