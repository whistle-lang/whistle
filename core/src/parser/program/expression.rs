use crate::parser::parser::*;
use super::operator::*;
use crate::parser::ast::*;
use super::literal::*;
use super::ident::*;
pub use crate::lexer::*;

pub fn parse_expression(parser:&mut Parser) -> Option<Expr> {
    parse_expression_with_precedence(parser, -1)
}

pub fn parse_expression_with_precedence(parser:&mut Parser, precedence: i32) -> Option<Expr> {
    if let Some(expression) = parse_unary_or_primary_expression(parser) {
        let previous = Expr::Unary(expression);
        if let Some(Token::Operator(operator)) = parser.peek() {
            if Operator::is_binary(operator) {
                return parse_binary_expression(parser, previous)
            }
        } 
        return Some(previous)
    }
    None
}

pub fn parse_unary_or_primary_expression(parser:&mut Parser) -> Option<UnaryExpr> {
    if let Some(Token::Operator(operator)) = parser.peek(){
        if Operator::is_unary(operator) {
            return parse_unary_expression(parser)
        }
    }
    return parse_primary_expression(parser)
}

pub fn parse_unary_expression(parser:&mut Parser) -> Option<UnaryExpr> {
    if let Some(expression) = parse_unary_or_primary_expression(parser) {
        return Some(UnaryExpr::UnaryOp {
            op: parse_unary_operator(parser),
            expr: Box::new(expression)
        })
    }
    None
}

pub fn parse_binary_expression(parser:&mut Parser, previous: Expr) -> Option<Expr> {
    if let Some(expression) = parse_expression_with_precedence(parser , 0) { 
        return Some(Expr::Binary{
            lhs: Box::new(previous),
            op: parse_binary_operator(parser),
            //TODO: precedence
            rhs: Box::new(expression)
        })
    }
    None
}

pub fn parse_primary_expression(parser:&mut Parser) -> Option<UnaryExpr> {
    if let Some(current) = parser.peek() {
        //TODO: ParseConditional, ParseSelector
        let primary = match current {
            Token::BoolLit(_current) => parse_boolean_literal(parser),
            Token::IntLit(_current) => parse_integer_literal(parser),
            Token::FloatLit(_current) => parse_float_literal(parser),
            Token::CharLit(_current) => parse_char_literal(parser),
            Token::StrLit(_current) => parse_string_literal(parser),
            Token::Ident(_current) => 
                if parser.is_tok_eq(Token::Punc(Punc::LeftParen), 1) {
                    parse_function_call(parser)
                } else {
                    parse_variable_access(parser)
                }
            Token::Punc(Punc::LeftParen) => parse_grouping(parser),
            _ => {println!("Could not parse expression {:?}", current); None}
        };
        if let Some(primary) = primary {
            return Some(UnaryExpr::Primary(primary))
        }
    }
    None
}

pub fn parse_function_call(parser:&mut Parser) -> Option<PrimaryExpr> {
    if let Some(name) = parse_ident(parser){
        if parser.eat_type(Token::Punc(Punc::LeftParen)).is_some() {
            let expressions = parser.until_is(|parser| {
                let expr = parse_expression(parser);
                parser.eat_type(Token::Punc(Punc::Comma));
                expr
            }, Token::Punc(Punc::RightBrace));
            
            if parser.eat_type(Token::Punc(Punc::RightParen)).is_some() {
                return Some(PrimaryExpr::Arguments {
                    prim: Box::new(PrimaryExpr::Operand(Operand::Ident(name.to_string()))),
                    args: expressions
                })
            }
        }
    }
    None
}

pub fn parse_variable_access(parser:&mut Parser) -> Option<PrimaryExpr> {
    if let Some(name) = parse_ident(parser) {
        return Some(PrimaryExpr::Operand(Operand::Ident(name.to_string())))
    }
    None
}

pub fn parse_grouping(parser:&mut Parser) -> Option<PrimaryExpr> {
    if parser.eat_type(Token::Punc(Punc::LeftParen)).is_some() {
        if let Some(expr) = parse_expression(parser){
            if parser.eat_type(Token::Punc(Punc::RightParen)).is_some() {
                return Some(PrimaryExpr::Operand(Operand::Grouping(Box::new(expr))))
            }
        }
    }
    None
}