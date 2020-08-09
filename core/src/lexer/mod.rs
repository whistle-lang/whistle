use unic_ucd_category::GeneralCategory;

mod error;
pub use error::*;
mod token;
pub use token::*;
mod tokenizer;

use tokenizer::Tokenizer;

macro_rules! ok_or_term {
  ($self:ident, $token:expr) => {
    let start = $self.tokenizer.index;
    let token: Result<Token, ErrorKind> = $token;
    let end = $self.tokenizer.index;

    if let Ok(token) = token {
      return Some(Ok(TokenItem {
        token,
        pos: TokenPos { start, end },
      }));
    } else if let Err(err) = token {
      if err.terminable() {
        return Some(Err(LexerError::new(err, TokenPos { start, end })));
      } else {
        $self.tokenizer.index = start;
      }
    }
  };
}

#[derive(Debug, Clone)]
pub struct Lexer {
  tokenizer: Tokenizer,
}

impl Lexer {
  pub fn new(source: String) -> Self {
    Self {
      tokenizer: Tokenizer::new(&*source),
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

  fn comment_line(&mut self) -> Result<Token, ErrorKind> {
    let mut comment = String::new();

    if self.tokenizer.eat_str("//").is_some() {
      loop {
        if let Some(ch) = self.tokenizer.peek() {
          match ch {
            '\n' => break,
            _ => comment.push(self.tokenizer.step().unwrap()),
          }
        }
      }

      Ok(Token::CommentLine(comment))
    } else {
      Err(ErrorKind::ExpectedCommentLine)
    }
  }

  fn comment_inline(&mut self) -> Result<Token, ErrorKind> {
    let mut comment = String::new();

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

      Ok(Token::CommentInline(comment))
    } else {
      Err(ErrorKind::ExpectedCommentInline)
    }
  }

  fn ident_or_keyword(&mut self) -> Result<Token, ErrorKind> {
    if let Some(ident) = self.read_ident() {
      if let Some(keyword) = Keyword::from(&*ident) {
        Ok(Token::Keyword(keyword))
      } else {
        Ok(Token::Ident(ident))
      }
    } else {
      Err(ErrorKind::ExpectedIdentOrKeyword)
    }
  }

  fn operator(&mut self) -> Result<Token, ErrorKind> {
    for operator in Operator::operators().clone().iter() {
      if self.tokenizer.eat_str(operator).is_some() {
        if let Some(op) = Operator::from(operator) {
          return Ok(Token::Operator(op));
        }
      }
    }

    Err(ErrorKind::ExpectedOperator)
  }

  fn float_lit(&mut self) -> Result<Token, ErrorKind> {
    let mut float = String::new();
    let mut dec_or_exp = false;

    if let Some(start) = self.tokenizer.read_while(Lexer::is_decimal) {
      float.push_str(&*start);

      if self.tokenizer.eat_char('.').is_some() {
        float.push('.');

        if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
          float.push_str(&*dec);
        } else {
          return Err(ErrorKind::ExpectedDec);
        }

        dec_or_exp = true;
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
            return Err(ErrorKind::ExpectedExp);
          }

          dec_or_exp = true;
        }
      } else {
        return Err(ErrorKind::UnexpectedEOF);
      }

      if !dec_or_exp {
        return Err(ErrorKind::ExpectedDecOrExp);
      }
    } else {
      return Err(ErrorKind::ExpectedFloatLit);
    }

    if let Ok(float) = float.parse::<f64>() {
      Ok(Token::FloatLit(float))
    } else {
      Err(ErrorKind::CouldNotParseFloat)
    }
  }

  fn int_lit(&mut self) -> Result<Token, ErrorKind> {
    if self.tokenizer.eat_str("0b").is_some() {
      if let Some(bin) = self.tokenizer.read_while(Lexer::is_binary) {
        Ok(Token::IntLit(Lexer::usize_from_binary(&*bin)))
      } else {
        Err(ErrorKind::ExpectedBin)
      }
    } else if self.tokenizer.eat_str("0o").is_some() {
      if let Some(oct) = self.tokenizer.read_while(Lexer::is_octal) {
        Ok(Token::IntLit(Lexer::usize_from_octal(&*oct)))
      } else {
        Err(ErrorKind::ExpectedOct)
      }
    } else if self.tokenizer.eat_str("0x").is_some() {
      if let Some(hex) = self.tokenizer.read_while(Lexer::is_hex) {
        Ok(Token::IntLit(Lexer::usize_from_hex(&*hex)))
      } else {
        Err(ErrorKind::ExpectedHex)
      }
    } else if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
      Ok(Token::IntLit(Lexer::usize_from_decimal(&*dec)))
    } else {
      Err(ErrorKind::ExpectedIntLit)
    }
  }

  fn string_lit(&mut self) -> Result<Token, ErrorKind> {
    let mut inner = String::new();

    if self.tokenizer.eat_char('"').is_none() {
      return Err(ErrorKind::ExpectedStringStartDelim);
    }

    if let Some(string) = self.read_inner() {
      inner.push_str(&*string);
    } else {
      return Err(ErrorKind::ExpectedStringInner);
    }

    if self.tokenizer.eat_char('"').is_none() {
      return Err(ErrorKind::ExpectedStringEndDelim);
    }

    Ok(Token::StringLit(inner))
  }

  fn char_lit(&mut self) -> Result<Token, ErrorKind> {
    if self.tokenizer.eat_char('\'').is_none() {
      return Err(ErrorKind::ExpectedCharStartDelim);
    }

    let inner = if let Some(esc) = self.read_esc() {
      esc
    } else if let Some(ch) = self.tokenizer.step() {
      ch
    } else {
      return Err(ErrorKind::UnexpectedEOF);
    };

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(ErrorKind::ExpectedCharEndDelim);
    }

    Ok(Token::CharLit(inner))
  }

  fn bool_lit(&mut self) -> Result<Token, ErrorKind> {
    if self.tokenizer.eat_str("true").is_some() {
      Ok(Token::BoolLit(true))
    } else if self.tokenizer.eat_str("false").is_some() {
      Ok(Token::BoolLit(false))
    } else {
      Err(ErrorKind::ExpectedBoolLit)
    }
  }

  fn none_lit(&mut self) -> Result<Token, ErrorKind> {
    if self.tokenizer.eat_str("none").is_some() {
      Ok(Token::NoneLit)
    } else {
      Err(ErrorKind::ExpectedNoneLit)
    }
  }

  fn tip(&mut self) -> Result<Token, ErrorKind> {
    if self.tokenizer.eat_char('#').is_none() {
      return Err(ErrorKind::ExpectedHash);
    }

    if self.tokenizer.eat_char('(').is_none() {
      return Err(ErrorKind::ExpectedLeftParen);
    }

    self.whitespace();

    let ident = if let Some(i) = self.read_ident() {
      i
    } else {
      return Err(ErrorKind::ExpectedTipIdent);
    };

    self.whitespace();

    if self.tokenizer.eat_char(')').is_none() {
      return Err(ErrorKind::ExpectedRightParen);
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
      return Err(ErrorKind::ExpectedNewline);
    };

    Ok(Token::Tip(Tip { ident, value }))
  }

  fn punc(&mut self) -> Result<Token, ErrorKind> {
    if let Some(ch) = self.tokenizer.peek() {
      if let Some(punc) = Punc::from(ch) {
        self.tokenizer.step();
        Ok(Token::Punc(punc))
      } else {
        Err(ErrorKind::ExpectedPunc)
      }
    } else {
      Err(ErrorKind::UnexpectedEOF)
    }
  }
}

impl Iterator for Lexer {
  type Item = Result<TokenItem, LexerError>;

  fn next(&mut self) -> Option<Result<TokenItem, LexerError>> {
    if !self.tokenizer.within() {
      return None;
    }

    if self.whitespace() || self.comment() {
      return self.next();
    }

    ok_or_term!(self, self.ident_or_keyword());
    ok_or_term!(self, self.operator());
    ok_or_term!(self, self.float_lit());
    ok_or_term!(self, self.int_lit());
    ok_or_term!(self, self.string_lit());
    ok_or_term!(self, self.char_lit());
    ok_or_term!(self, self.bool_lit());
    ok_or_term!(self, self.none_lit());
    ok_or_term!(self, self.tip());
    ok_or_term!(self, self.punc());

    Some(Err(LexerError::new(
      ErrorKind::NoMatch,
      TokenPos {
        start: self.tokenizer.index,
        end: self.tokenizer.index,
      },
    )))
  }
}
