mod lexer;
pub use lexer::*;
mod parser;
pub use parser::*;
mod compiler;
pub use compiler::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
  LexerError(LexerError),
  ParserError(ParserError),
  CompilerError(CompilerError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Warning {}

#[derive(Debug, Clone)]
pub struct DiagnosticHandler {
  pub errors: Vec<Error>,
  pub warnings: Vec<Warning>,
}

impl DiagnosticHandler {
  pub fn new() -> Self {
    Self {
      errors: Vec::new(),
      warnings: Vec::new(),
    }
  }
}
