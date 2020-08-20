use crate::parser::ast::*;
use crate::parser::Parser;
use super::expression::*;
use super::ident::*;
use crate::lexer::*;

pub fn parse_statement(parser: &mut Parser) -> Option<Stmt> {
    //TODO: parse_ident
    match parser.peek() {
        Some(Token::Tip(_tip)) => parse_tip(parser),
        Some(Token::Keyword(keyword)) => {
            match keyword {
                Keyword::If => parse_if_statement(parser),
                Keyword::Return => parse_return_statement(parser),
                Keyword::Var => parse_var_statement(parser),
                Keyword::Val => parse_val_statement(parser),
                Keyword::While => parse_while_statement(parser),
                Keyword::Continue => parse_continue_statement(parser),
                Keyword::Break => parse_break_statement(parser),
                _ => {println!("Could not parse statement {:?}", keyword); None}
            }
        }
        Some(Token::Punc(Punc::LeftBrace)) => parse_block_statement(parser),
        _ => {println!("Could not parse statement {:?}", parser.peek()); None}
      }
}

//TODO: parse_tip
pub fn parse_tip(parser:&mut Parser) -> Option<Stmt> {
    Some(Stmt::Continue)
}

pub fn parse_if_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
        if let Some(cond) = parse_expression(parser) {
            if let Some(then_stmt) = parse_statement(parser) {
                return Some(Stmt::If {
                    cond: Box::new(cond),
                    then_stmt: Box::new(then_stmt),
                    else_stmt: parse_else_statement(parser)
                })
            }
        }
    }
    None
}

pub fn parse_else_statement(parser:&mut Parser) -> Option<Box<Stmt>> {
    if parser.eat_tok(Token::Keyword(Keyword::Else)).is_some() {
        if let Some(else_stmt) = parse_statement(parser) {
            return Some(Box::new(else_stmt))
        }
    }
    None
}

pub fn parse_return_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::Return)).is_some() {
        if let Some(expr) = parse_expression(parser) {
            return Some(Stmt::Return(Some(Box::new(expr))))
        }
    }
    None
}

pub fn parse_var_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::Var)).is_some() {
        if let Some(ident_typed) = parse_ident_typed(parser) {
            if let Some(value) = parse_assign(parser) {
                return Some(Stmt::VarDecl {
                    ident_typed: ident_typed,
                    val: Box::new(value)
                })
            }
        }
    }
    None
}

pub fn parse_assign(parser:&mut Parser) -> Option<Expr> {
    if parser.eat_tok(Token::Operator(Operator::Assign)).is_some() {
        return parse_expression(parser)
    }
    None
}

pub fn parse_val_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::Val)).is_some() {
        if let Some(ident_typed) = parse_ident_typed(parser) {
            if let Some(value) = parse_assign(parser) {
                return Some(Stmt::ValDecl {
                    ident_typed: ident_typed,
                    val: Box::new(value)
                })
            }
        }
    }
    None
}

pub fn parse_while_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::If)).is_some() {
        if let Some(cond) = parse_expression(parser) {
            if let Some(do_stmt) = parse_statement(parser) {
                return Some(Stmt::While {
                    cond: Some(Box::new(cond)),
                    do_stmt: Box::new(do_stmt),
                })
            }
        }
    }
    None
}

pub fn parse_continue_statement(parser:&mut Parser) -> Option<Stmt> {
    parser.eat_tok(Token::Keyword(Keyword::Continue));
    return Some(Stmt::Continue)
}

pub fn parse_break_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Keyword(Keyword::Break)).is_some() {
        return Some(Stmt::Break)
    }
    None
}

pub fn parse_block_statement(parser:&mut Parser) -> Option<Stmt> {
    if parser.eat_tok(Token::Punc(Punc::LeftBrace)).is_some() {
        let statements = parser.until_is(parse_statement, Token::Punc(Punc::RightBrace));
        if parser.eat_tok(Token::Punc(Punc::RightBrace)).is_some() {
            return Some(Stmt::Block(statements))
        }
    }
    None
}

pub fn parse_expression_statement(parser:&mut Parser) -> Option<Stmt> {
    if let Some(expr) = parse_expression(parser) {
        return Some(Stmt::Expr(expr))
    }
    None
}