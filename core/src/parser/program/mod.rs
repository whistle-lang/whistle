mod ident;
mod statement;
mod expression;
mod literal;
mod operator;
pub use statement::*;
pub use super::*;
pub use crate::lexer::*;

pub fn parse_program(parser: &mut Parser) {
  //TODO: parse_functions, codeblocks
  while parser.within() {
    println!("{:?}", parse_statement(parser));
  }
}

pub fn decode(lexer: Lexer) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    for tok in lexer {
      if let Some(tok) = tok.ok(){
        tokens.push(tok.token)
      } else {
        break
      }
    }
    tokens
}

