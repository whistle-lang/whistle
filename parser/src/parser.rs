use super::error::ParserError;
use super::error::ParserErrorKind;
use super::parse_program;

use whistle_ast::ProgramStmt;
use whistle_common::Token;
use whistle_common::TokenItem;

#[derive(Debug, Clone)]
pub struct Parser {
  pub tokens: Vec<Token>,
  pub index: usize,
}

#[macro_export]
macro_rules! eat_type {
  ($parser: ident, $t1:ident::$v1:ident$(($t2:ident::$v2:ident))?) => {
    if let $t1::$v1($($t2::$v2)?(val)) = $parser.peek()?.clone() {
      $parser.step();
      Ok(val)
    } else {
      Err(ParserError::new(
        vec![ParserErrorKind::ExpectedTokenType(
          stringify!($t1::$v1$(($t2::$v2))?).to_string()
        )],
        $parser.index,
      ))
    }
  };
}

impl Parser {
  pub fn new(items: Vec<TokenItem>) -> Self {
    let mut tokens = vec![];
    for token in items {
      tokens.push(token.token.clone())
    }

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

  pub fn peek_index(&self, i: usize) -> Result<&Token, ParserError> {
    if self.within_index(i) {
      return Ok(&self.tokens[i]);
    }
    Err(ParserError::new(
      vec![ParserErrorKind::UnexpectedEOF],
      self.index,
    ))
  }

  pub fn peek_offset(&self, offset: isize) -> Result<&Token, ParserError> {
    self.peek_index((self.index as isize + offset) as usize)
  }

  pub fn peek(&self) -> Result<&Token, ParserError> {
    self.peek_index(self.index)
  }

  pub fn is_type(&self, tok: Token) -> bool {
    let curr = self.peek();

    if let Ok(curr) = curr {
      if core::mem::discriminant(curr) == core::mem::discriminant(&tok) {
        return true;
      }
    }

    false
  }

  pub fn is_tok(&self, tok: Token) -> bool {
    let curr = self.peek();

    if let Ok(curr) = curr {
      if tok == *curr {
        return true;
      }
    }

    false
  }

  pub fn step(&mut self) {
    if self.within() {
      self.index += 1;
    }
  }

  pub fn eat_type(&mut self, tok: Token) -> Result<(), ParserError> {
    if self.is_type(tok.clone()) {
      self.step();
      return Ok(());
    }
    Err(ParserError::new(
      vec![ParserErrorKind::ExpectedTokenType(
        stringify!(tok).to_string(),
      )],
      self.index,
    ))
  }

  pub fn eat_tok(&mut self, tok: Token) -> Result<(), ParserError> {
    if self.is_tok(tok.clone()) {
      self.step();
      return Ok(());
    }
    Err(ParserError::new(
      vec![ParserErrorKind::ExpectedToken(tok)],
      self.index,
    ))
  }

  pub fn eat_repeat<P, T>(&mut self, parse: P) -> Vec<T>
  where
    P: Fn(&mut Parser) -> Result<T, ParserError> + Copy,
  {
    let mut res = Vec::new();
    while let Some(val) = self.maybe(parse) {
      res.push(val);
    }
    res
  }

  pub fn maybe<P, T>(&mut self, parse: P) -> Option<T>
  where
    P: Fn(&mut Parser) -> Result<T, ParserError>,
  {
    let pre = self.index;
    match parse(self) {
      Ok(val) => Some(val),
      Err(_) => {
        self.index = pre;
        return None;
      }
    }
  }
}

impl Iterator for Parser {
  type Item = Result<ProgramStmt, ParserError>;

  fn next(&mut self) -> Option<Result<ProgramStmt, ParserError>> {
    if !self.within() {
      return None;
    }

    Some(parse_program(self))
  }
}
