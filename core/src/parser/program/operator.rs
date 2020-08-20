pub use crate::parser::parser::*;
pub use crate::lexer::*;

// pub fn parse_binary_operator(parser:Parser) -> Operator {
//     let Some(Token::Operator(curr)) = parser.eat_type(Token::Operator(Operator::Add));

//     Operator::Add
// }

// pub fn parse_unary_operator(parser:Parser) -> Operator {
//     let Some(Token::Operator(curr)) = parser.eat_type(Token::Operator(Operator::Add));

//     Operator::Add
// }

// pub fn parse_primary_operator(parser:Parser) -> Operator {
//     let Some(Token::Operator(curr)) = parser.eat_type(Token::Operator(Operator::Add));

//     Operator::Add
// }

pub fn parse_binary_operator(parser:&mut Parser) -> Operator {
    Operator::Add
}

pub fn parse_unary_operator(parser:&mut Parser) -> Operator {
    Operator::Add
}

pub fn parse_primary_operator(parser:&mut Parser) -> Operator {
    Operator::Add
}