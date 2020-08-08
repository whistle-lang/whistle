use unic_ucd_category::GeneralCategory;

mod error;
pub use error::*;
mod token;
pub use token::Token;
mod tokens;
pub use tokens::*;
mod tokenizer;

use tokenizer::Tokenizer;

macro_rules! ok_or_term {
  ($token:expr) => {
    let token: Result<Token, LexerError> = $token;
    if token.is_ok() || token.clone().unwrap_err().kind.terminable() {
      return Some(token);
    }
  };
}

#[derive(Debug, Clone)]
pub struct Lexer {
  tokenizer: Tokenizer,
  keywords: Vec<String>,
  operators: Vec<String>,
  punc: Vec<String>,
}

impl Lexer {
  pub fn new(source: String) -> Self {
    Self {
      tokenizer: Tokenizer::new(&*source),
      keywords: vec![
        String::from("import"),
        String::from("as"),
        String::from("from"),
        String::from("export"),
        String::from("fun"),
        String::from("return"),
        String::from("if"),
        String::from("else"),
        String::from("while"),
        String::from("break"),
        // Planned:
        String::from("for"),
        String::from("in"),
        String::from("match"),
        String::from("type"),
        String::from("struct"),
        String::from("trait"),
      ],

      // When adding new operators sort using: https://repl.it/@eliassjogreen/sort-by-substring
      operators: vec![
        String::from("&&="),
        String::from("**="),
        String::from("<<="),
        String::from(">>="),
        String::from("||="),
        String::from("!="),
        String::from("%="),
        String::from("&&"),
        String::from("&="),
        String::from("**"),
        String::from("*="),
        String::from("+="),
        String::from("-="),
        String::from("/="),
        String::from("<<"),
        String::from("<="),
        String::from("=="),
        String::from(">="),
        String::from(">>"),
        String::from("^="),
        String::from("|="),
        String::from("||"),
        String::from("!"),
        String::from("%"),
        String::from("&"),
        String::from("*"),
        String::from("+"),
        String::from("-"),
        String::from("/"),
        String::from("<"),
        String::from("="),
        String::from(">"),
        String::from("^"),
        String::from("|"),
        String::from("~"),
      ],

      punc: vec![
        String::from("("),
        String::from(")"),
        String::from("{"),
        String::from("}"),
        String::from("["),
        String::from("]"),
        String::from("."),
        String::from(","),
        String::from(":"),
      ],
    }
  }

  fn usize_from_binary(bin: &str) -> usize {
    usize::from_str_radix(&*bin, 2).unwrap()
  }

  fn usize_from_octal(oct: &str) -> usize {
    usize::from_str_radix(&*oct, 8).unwrap()
  }

  fn usize_from_hex(hex: &str) -> usize {
    usize::from_str_radix(&*hex, 16).unwrap()
  }

  fn usize_from_decimal(dec: &str) -> usize {
    usize::from_str_radix(dec, 10).unwrap()
  }

  fn read_ident(&mut self) -> Option<String> {
    if let Some(ch) = self.tokenizer.peek() {
      if Lexer::is_letter(ch) {
        let mut ident = String::new();
        if let Some(ch) = self.tokenizer.step() {
          ident.push(ch);
        }

        ident.push_str(
          &self
            .tokenizer
            .read_while(|c| Lexer::is_letter(c) || Lexer::is_number(c))
            .unwrap_or_default(),
        );

        return Some(ident);
      }
    }

    None
  }

  fn read_esc(&mut self) -> Option<char> {
    if self.tokenizer.eat_char('\\').is_some() {
      match self.tokenizer.peek() {
        Some('"') => {
          self.tokenizer.step();
          Some('"')
        }
        Some('\\') => {
          self.tokenizer.step();
          Some('\\')
        }
        Some('r') => {
          self.tokenizer.step();
          Some('\r')
        }
        Some('n') => {
          self.tokenizer.step();
          Some('\n')
        }
        Some('t') => {
          self.tokenizer.step();
          Some('\t')
        }
        Some('0') => {
          self.tokenizer.step();
          Some('\0')
        }
        Some('\'') => {
          self.tokenizer.step();
          Some('\'')
        }
        Some(_) => None,
        None => None,
      }
    } else {
      None
    }
  }

  fn read_inner(&mut self) -> Option<String> {
    let mut inner = self
      .tokenizer
      .read_while(|ch| ch != '\\' && ch != '"')
      .unwrap_or_default();

    if let Some(esc) = self.read_esc() {
      inner.push(esc);

      if let Some(string) = self.read_inner() {
        inner.push_str(&*string);
      }
    }

    Some(inner)
  }

  fn is_letter(ch: char) -> bool {
    ch == '_' || GeneralCategory::of(ch).is_letter()
  }

  fn is_number(ch: char) -> bool {
    GeneralCategory::of(ch).is_number()
  }

  fn is_decimal(ch: char) -> bool {
    Lexer::is_octal(ch) || ch == '8' || ch == '9'
  }

  fn is_binary(ch: char) -> bool {
    ch == '0' || ch == '1'
  }

  fn is_octal(ch: char) -> bool {
    Lexer::is_binary(ch)
      || ch == '2'
      || ch == '3'
      || ch == '4'
      || ch == '5'
      || ch == '6'
      || ch == '7'
  }

  fn is_hex(ch: char) -> bool {
    Lexer::is_decimal(ch)
      || ch == 'a'
      || ch == 'b'
      || ch == 'c'
      || ch == 'd'
      || ch == 'e'
      || ch == 'f'
      || ch == 'A'
      || ch == 'B'
      || ch == 'C'
      || ch == 'D'
      || ch == 'E'
      || ch == 'F'
  }

  fn whitespace(&mut self) -> bool {
    let index = self.tokenizer.index;

    while let Some(' ') | Some('\t') | Some('\r') | Some('\n') = self.tokenizer.peek() {
      self.tokenizer.step();
    }

    index != self.tokenizer.index
  }

  fn comment(&mut self) -> bool {
    self.comment_line().is_ok() || self.comment_inline().is_ok()
  }

  fn comment_line(&mut self) -> Result<Token, LexerError> {
    let mut comment = String::new();
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("//").is_some() {
      loop {
        if let Some(ch) = self.tokenizer.peek() {
          match ch {
            '\n' => break,
            _ => comment.push(self.tokenizer.step().unwrap()),
          }
        }
      }

      Ok(Token::new(TokenValue::CommentLine(comment), index))
    } else {
      Err(LexerError::new(ErrorKind::ExpectedCommentLine, index))
    }
  }

  fn comment_inline(&mut self) -> Result<Token, LexerError> {
    let mut comment = String::new();
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("/*").is_some() {
      let mut depth = 1;

      loop {
        if self.tokenizer.eat_str("/*").is_some() {
          depth += 1;
        } else if self.tokenizer.eat_str("*/").is_some() {
          depth -= 1;
        } else if let Some(ch) = self.tokenizer.step() {
          comment.push(ch);
        }

        if depth == 0 {
          break;
        }
      }

      Ok(Token::new(TokenValue::CommentInline(comment), index))
    } else {
      Err(LexerError::new(ErrorKind::ExpectedCommentInline, index))
    }
  }

  fn ident(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if let Some(ident) = self.read_ident() {
      if !self.keywords.contains(&ident) {
        Ok(Token::new(TokenValue::Ident(ident), index))
      } else {
        self.tokenizer.index = index;
        Err(LexerError::new(ErrorKind::ExpectedIdent, index))
      }
    } else {
      Err(LexerError::new(ErrorKind::ExpectedIdent, index))
    }
  }

  fn keyword(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if let Some(ident) = self.read_ident() {
      if let Some(keyword) = Keyword::from(&*ident) {
        Ok(Token::new(TokenValue::Keyword(keyword), index))
      } else {
        self.tokenizer.index = index;
        Err(LexerError::new(ErrorKind::ExpectedKeyword, index))
      }
    } else {
      Err(LexerError::new(ErrorKind::ExpectedKeyword, index))
    }
  }

  fn operator(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    for operator in self.operators.clone().iter() {
      if self.tokenizer.eat_str(operator).is_some() {
        if let Some(op) = Operator::from(operator) {
          return Ok(Token::new(TokenValue::Operator(op), index));
        }
      }
    }

    self.tokenizer.index = index;
    Err(LexerError::new(ErrorKind::ExpectedOperator, index))
  }

  fn float_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;
    let mut float = String::new();
    let mut dec = false;
    let mut exp = false;

    if let Some(start) = self.tokenizer.read_while(Lexer::is_decimal) {
      float.push_str(&*start);

      if self.tokenizer.eat_char('.').is_some() {
        float.push('.');

        if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
          float.push_str(&*dec);
        } else {
          return Err(LexerError::new(
            ErrorKind::ExpectedDec,
            self.tokenizer.index,
          ));
        }

        dec = true;
      }

      if let Some(next) = self.tokenizer.peek() {
        if next == 'e' || next == 'E' {
          self.tokenizer.step();
          float.push('e');

          if self.tokenizer.eat_char('+').is_some() {
            float.push('+');
          } else if self.tokenizer.eat_char('-').is_some() {
            float.push('-');
          }

          if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
            float.push_str(&*dec);
          } else {
            return Err(LexerError::new(
              ErrorKind::ExpectedExp,
              self.tokenizer.index,
            ));
          }

          exp = true;
        }
      } else {
        return Err(LexerError::new(
          ErrorKind::UnexpectedEOF,
          self.tokenizer.index,
        ));
      }

      if !dec && !exp {
        self.tokenizer.index = index;

        return Err(LexerError::new(
          ErrorKind::ExpectedDecOrExp,
          self.tokenizer.index,
        ));
      }
    } else {
      return Err(LexerError::new(
        ErrorKind::ExpectedFloatLit,
        self.tokenizer.index,
      ));
    }

    if let Ok(float) = float.parse::<f64>() {
      Ok(Token::new(TokenValue::FloatLit(float), index))
    } else {
      Err(LexerError::new(
        ErrorKind::CouldNotParseFloat,
        self.tokenizer.index,
      ))
    }
  }

  fn int_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("0b").is_some() {
      if let Some(bin) = self.tokenizer.read_while(Lexer::is_binary) {
        Ok(Token::new(
          TokenValue::IntLit(Lexer::usize_from_binary(&*bin)),
          index,
        ))
      } else {
        Err(LexerError::new(
          ErrorKind::ExpectedBin,
          self.tokenizer.index,
        ))
      }
    } else if self.tokenizer.eat_str("0o").is_some() {
      if let Some(oct) = self.tokenizer.read_while(Lexer::is_octal) {
        Ok(Token::new(
          TokenValue::IntLit(Lexer::usize_from_octal(&*oct)),
          index,
        ))
      } else {
        Err(LexerError::new(
          ErrorKind::ExpectedOct,
          self.tokenizer.index,
        ))
      }
    } else if self.tokenizer.eat_str("0x").is_some() {
      if let Some(hex) = self.tokenizer.read_while(Lexer::is_hex) {
        Ok(Token::new(
          TokenValue::IntLit(Lexer::usize_from_hex(&*hex)),
          index,
        ))
      } else {
        Err(LexerError::new(
          ErrorKind::ExpectedHex,
          self.tokenizer.index,
        ))
      }
    } else if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
      Ok(Token::new(
        TokenValue::IntLit(Lexer::usize_from_decimal(&*dec)),
        index,
      ))
    } else {
      Err(LexerError::new(ErrorKind::ExpectedIntLit, index))
    }
  }

  fn string_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;
    let mut inner = String::new();

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerError::new(ErrorKind::ExpectedStringStartDelim, index));
    }

    if let Some(string) = self.read_inner() {
      inner.push_str(&*string);
    } else {
      return Err(LexerError::new(
        ErrorKind::ExpectedStringInner,
        self.tokenizer.index,
      ));
    }

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedStringEndDelim,
        self.tokenizer.index,
      ));
    }

    Ok(Token::new(TokenValue::StringLit(inner), index))
  }

  fn char_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerError::new(ErrorKind::ExpectedCharStartDelim, index));
    }

    let inner = if let Some(esc) = self.read_esc() {
      esc
    } else if let Some(ch) = self.tokenizer.step() {
      ch
    } else {
      return Err(LexerError::new(
        ErrorKind::UnexpectedEOF,
        self.tokenizer.index,
      ));
    };

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedCharEndDelim,
        self.tokenizer.index,
      ));
    }

    Ok(Token::new(TokenValue::CharLit(inner), index))
  }

  fn bool_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("true").is_some() {
      Ok(Token::new(TokenValue::BoolLit(true), index))
    } else if self.tokenizer.eat_str("false").is_some() {
      Ok(Token::new(TokenValue::BoolLit(false), index))
    } else {
      Err(LexerError::new(
        ErrorKind::ExpectedBoolLit,
        self.tokenizer.index,
      ))
    }
  }

  fn none_lit(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("none").is_some() {
      Ok(Token::new(TokenValue::NoneLit, index))
    } else {
      Err(LexerError::new(
        ErrorKind::ExpectedNoneLit,
        self.tokenizer.index,
      ))
    }
  }

  fn tip(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_char('#').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedHash,
        self.tokenizer.index,
      ));
    }

    if self.tokenizer.eat_char('(').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedLeftParen,
        self.tokenizer.index,
      ));
    }

    self.whitespace();

    let ident = if let Some(i) = self.read_ident() {
      i
    } else {
      return Err(LexerError::new(
        ErrorKind::ExpectedIdent,
        self.tokenizer.index,
      ));
    };

    self.whitespace();

    if self.tokenizer.eat_char(')').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedRightParen,
        self.tokenizer.index,
      ));
    }

    self.whitespace();

    let value = if self.tokenizer.eat_char('{').is_some() {
      let mut val = String::new();
      let mut depth = 1;

      loop {
        if self.tokenizer.eat_char('{').is_some() {
          depth += 1;
        } else if self.tokenizer.eat_char('}').is_some() {
          depth -= 1;
        } else if let Some(ch) = self.tokenizer.step() {
          val.push(ch);
        }

        if depth == 0 {
          break;
        }
      }

      val
    } else if let Some(val) = self.tokenizer.read_while(|ch| ch != '\n') {
      val
    } else {
      return Err(LexerError::new(
        ErrorKind::ExpectedNewline,
        self.tokenizer.index,
      ));
    };

    Ok(Token::new(TokenValue::Tip(Tip { ident, value }), index))
  }

  fn punc(&mut self) -> Result<Token, LexerError> {
    let index = self.tokenizer.index;

    if let Some(ch) = self.tokenizer.peek() {
      if let Some(punc) = Punc::from(ch) {
        self.tokenizer.step();
        Ok(Token::new(TokenValue::Punc(punc), index))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedPunc, index))
      }
    } else {
      Err(LexerError::new(ErrorKind::UnexpectedEOF, index))
    }
  }
}

impl Iterator for Lexer {
  type Item = Result<Token, LexerError>;
  fn next(&mut self) -> Option<Result<Token, LexerError>> {
    if !self.tokenizer.within() {
      return None;
    }

    if self.whitespace() || self.comment() {
      return self.next();
    }

    ok_or_term!(self.ident());
    ok_or_term!(self.keyword());
    ok_or_term!(self.operator());
    ok_or_term!(self.float_lit());
    ok_or_term!(self.int_lit());
    ok_or_term!(self.string_lit());
    ok_or_term!(self.char_lit());
    ok_or_term!(self.bool_lit());
    ok_or_term!(self.none_lit());
    ok_or_term!(self.tip());
    ok_or_term!(self.punc());

    Some(Err(LexerError::new(
      ErrorKind::NoMatch,
      self.tokenizer.index,
    )))
  }
}
