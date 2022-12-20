use crate::eat_type;
use crate::parse_expr;
use crate::parse_ident;
use crate::parse_ident_extern;
use crate::parse_ident_import;
use crate::parse_ident_type;
use crate::parse_ident_typed;
use crate::parse_stmts;
use crate::parser::Parser;
use crate::ParserError;
use crate::ParserErrorKind;

use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Literal;
use whistle_ast::ProgramStmt;

use whistle_common::Keyword;
use whistle_common::Operator;
use whistle_common::Primitive;
use whistle_common::Punc;
use whistle_common::Span;
use whistle_common::Token;

pub fn parse_program(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  match parser.peek()?.token {
    Token::Keyword(Keyword::Fn)
    | Token::Keyword(Keyword::Export)
    | Token::Keyword(Keyword::Inline) => parse_fn_decl(parser),
    Token::Keyword(Keyword::Extern) => parse_extern_decl(parser),
    Token::Keyword(Keyword::Import) => parse_import(parser),
    Token::Keyword(Keyword::Val) => parse_val_decl(parser),
    Token::Keyword(Keyword::Var) => parse_var_decl(parser),
    Token::Keyword(Keyword::Struct) => parse_struct_decl(parser),
    // Token::Keyword(Keyword::Type) => parse_type_decl(parser),
    // _ => Ok(ProgramStmt::Stmt(parse_stmt(parser)?)),
    _ => Err(ParserError::new(
      ParserErrorKind::ExpectedProgramStmt,
      parser.peek()?.span,
    )),
  }
}

pub fn parse_params(parser: &mut Parser) -> Result<Vec<IdentTyped>, ParserError> {
  parser.eat_tok(Token::Punc(Punc::LeftParen))?;
  let idents = parser.eat_repeat(
    parse_ident_typed,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightParen),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightParen))?;
  Ok(idents)
}

// pub fn parse_type_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
//   let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
//   parser.eat_tok(Token::Keyword(Keyword::Type))?;
//   let ident = parse_ident(parser)?;
//   let mut params = Vec::new();
//   if let Some(first) = parser.maybe(parse_ident_typed) {
//     params.push(first);
//     params.append(&mut parser.eat_repeat(|parser| {
//       parser.eat_tok(Token::Punc(Punc::Comma))?;
//       parse_ident_typed(parser)
//     }));
//   }
//   Ok(ProgramStmt::TypeDecl {
//     export,
//     ident,
//     params,
//   })
// }

pub fn parse_struct_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Struct))?;
  let ident = parse_ident(parser)?;
  parser.eat_tok(Token::Punc(Punc::LeftBrace))?;
  let params = parser.eat_repeat(
    parse_ident_typed,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightBrace),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightBrace))?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::StructDecl {
    export,
    ident,
    params,
    span: Span { start, end },
  })
}

pub fn parse_fn_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  let export = parser.eat_tok(Token::Keyword(Keyword::Export)).is_ok();
  let inline = parser.eat_tok(Token::Keyword(Keyword::Inline)).is_ok();
  parser.eat_tok(Token::Keyword(Keyword::Fn))?;
  let ident = parse_ident(parser)?;
  let params = parse_params(parser)?;
  let ret_type = if parser.eat_tok(Token::Punc(Punc::Colon)).is_ok() {
    parse_ident_type(parser)?
  } else {
    let span = Some(parser.peek()?.span);
    IdentType::Primitive {
      prim: Primitive::None,
      span,
    }
  };
  let stmt = parse_stmts(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::FunctionDecl {
    inline,
    export,
    ident,
    params,
    ret_type,
    stmt,
    span: Span { start, end },
  })
}

pub fn parse_extern_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Extern))?;
  let namespace = eat_type!(parser, Token::Literal(Literal::Str))?;
  parser.eat_tok(Token::Punc(Punc::LeftBrace))?;
  let idents = parser.eat_repeat(
    parse_ident_extern,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightBrace),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightBrace))?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::Extern {
    idents,
    namespace,
    span: Span { start, end },
  })
}

pub fn parse_import(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Import))?;
  parser.eat_tok(Token::Punc(Punc::LeftBrace))?;
  let idents = parser.eat_repeat(
    parse_ident_import,
    Some(Token::Punc(Punc::Comma)),
    Token::Punc(Punc::RightBrace),
  )?;
  parser.eat_tok(Token::Punc(Punc::RightBrace))?;
  parser.eat_tok(Token::Keyword(Keyword::From))?;
  let from = eat_type!(parser, Token::Literal(Literal::Str))?;

  // TODO: Do this better...
  let imp_type = if from.starts_with('@') { "js" } else { "file" };
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::Import {
    idents,
    from,
    imp_type: imp_type.to_string(),
    span: Span { start, end },
  })
}

pub fn parse_var_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Var))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::VarDecl {
    ident_typed,
    val,
    span: Span { start, end },
  })
}

pub fn parse_val_decl(parser: &mut Parser) -> Result<ProgramStmt, ParserError> {
  let start = parser.peek()?.span.start;
  parser.eat_tok(Token::Keyword(Keyword::Val))?;
  let ident_typed = parse_ident_typed(parser)?;
  parser.eat_tok(Token::Operator(Operator::Assign))?;
  let val = parse_expr(parser)?;
  let end = parser.peek_offset(-1)?.span.end;
  Ok(ProgramStmt::ValDecl {
    ident_typed,
    val,
    span: Span { start, end },
  })
}
