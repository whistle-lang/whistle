use crate::lexer::{Lexer, Token};

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

  pub fn is_type(&self, tok: Token) -> bool {
    let curr = self.peek();

    if let Some(curr) = curr {
      if std::mem::discriminant(curr) == std::mem::discriminant(&tok) {
        return true;
      }
    }

    false
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

  pub fn is_tok_at(&self, tok: Token, offset: isize) -> bool {
    if let Some(curr) = self.peek_offset(offset) {
      if tok == *curr {
        return true;
      }
    }

    false
  }

  pub fn step(&mut self) {
    if self.within() {
      // println!("idx {}, tok {:?}", self.index, self.peek());
      self.index += 1;
    }
  }

  pub fn step_peek(&mut self) -> Option<&Token> {
    self.step();
    self.peek_offset(-1)
  }

  pub fn eat_type(&mut self, tok: Token) -> Option<&Token> {
    // let clone = tok.clone();
    if self.is_type(tok) {
      self.step_peek()
    } else {
      /* println!(
        "Expected type {:?} but got type {:?} instead",
        clone,
        self.peek()
      ); */
      None
    }
  }

  pub fn eat_tok(&mut self, tok: Token) -> Option<&Token> {
    // let clone = tok.clone();
    if self.is_tok(tok) {
      self.step_peek()
    } else {
      /* println!(
        "Expected token {:?} but got token {:?} instead",
        clone,
        self.peek()
      ); */
      None
    }
  }

  pub fn maybe<P, T>(&mut self, parse: P) -> Option<T>
  where
    P: Fn(&mut Parser) -> Option<T>,
  {
    let pre = self.index;

    if let Some(val) = parse(self) {
      Some(val)
    } else {
      self.index = pre;

      None
    }
  }

  pub fn check<P, T>(&mut self, parse: P) -> Option<T>
  where
    P: Fn(&mut Parser) -> Option<T>,
  {
    let pre = self.index;
    let ret = parse(self);
    self.index = pre;

    ret
  }

  pub fn or<P, T>(&mut self, parsers: Vec<P>) -> Option<T>
  where
    P: Fn(&mut Parser) -> Option<T>,
  {
    for parser in parsers {
      if let Some(val) = self.maybe(parser) {
        return Some(val);
      }
    }

    None
  }

  pub fn repeating<P, T>(&mut self, parse: P) -> Vec<T>
  where
    P: Fn(&mut Parser) -> Option<T> + Copy,
  {
    let mut res = Vec::new();

    while let Some(val) = self.maybe(parse) {
      res.push(val);
    }

    res
  }

  pub fn until_is<P, T>(&mut self, parse: P, token: Token) -> Vec<T>
  where
    P: Fn(&mut Parser) -> Option<T> + Copy,
  {
    let mut res = Vec::new();

    while let Some(val) = self.maybe(parse) {
      let clone = token.clone();
      res.push(val);

      if !self.is_tok_at(clone, 1) {
        break;
      }
    }

    res
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
