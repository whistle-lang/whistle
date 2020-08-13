pub use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub struct Parser {
  pub tokens: Vec<Token>,
  pub index: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn within_index(&self, i: usize) -> bool {
    if i < self.tokens.len() {
      return true;
    }

    false
  }

  pub fn within_offset(&self, offset: usize) -> bool {
    self.within_index(self.index + offset)
  }

  pub fn within(&self) -> bool {
    self.within_index(self.index)
  }

  pub fn peek_index(&self, i: usize) -> Option<&Token> {
    if self.within_index(i) {
      Some(&self.tokens[i])
    } else {
      None
    }
  }

  pub fn peek_offset(&self, offset: isize) -> Option<&Token> {
    self.peek_index((self.index as isize + offset) as usize)
  }

  pub fn peek(&self) -> Option<&Token> {
    self.peek_index(self.index)
  }

  pub fn is_tok(&self, tok: Token) -> bool {
    let curr = self.peek();

    if let Some(curr) = curr {
      if tok == *curr {
        return true;
      }
    }

    false
  }

  pub fn step(&mut self) -> Option<&Token> {
    if self.within() {
      self.index += 1;
    }

    self.peek_offset(-1)
  }
}

impl From<Lexer> for Parser {
  fn from(lexer: Lexer) -> Self {
    let mut toks: Vec<Token> = Vec::new();

    for tok in lexer {
      if tok.is_err() {
        //raiseException
        break;
      }

      toks.push(tok.unwrap().token);
    }

    Parser::new(toks)
  }
}
