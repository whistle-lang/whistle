#![allow(clippy::from_str_radix_10)]

use whistle_common::Keyword;
use whistle_common::Literal;
use whistle_common::Operator;
use whistle_common::Punc;
use whistle_common::Range;
use whistle_common::Tip;
use whistle_common::Token;
use whistle_common::TokenItem;

mod error;
pub use error::LexerError;
pub use error::LexerErrorKind;
mod tokenizer;
use tokenizer::Tokenizer;

macro_rules! ok_or_term {
  ($self:ident, $token:expr) => {
    let start = $self.tokenizer.index;
    let token: Result<Token, LexerErrorKind> = $token;
    let end = $self.tokenizer.index;

    if let Ok(token) = token {
      return Some(Ok(TokenItem {
        token,
        range: Range { start, end },
      }));
    } else if let Err(err) = token {
      if err.is_terminable() {
        return Some(Err(LexerError::new(err, Range { start, end })));
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
  pub fn new(source: &str) -> Self {
    Self {
      tokenizer: Tokenizer::new(source),
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
    ch == '_' || ch.is_alphabetic()
  }

  fn is_number(ch: char) -> bool {
    ch.is_numeric()
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

  fn comment_line(&mut self) -> Result<Token, LexerErrorKind> {
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
      Err(LexerErrorKind::ExpectedCommentLine)
    }
  }

  fn comment_inline(&mut self) -> Result<Token, LexerErrorKind> {
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
      Err(LexerErrorKind::ExpectedCommentInline)
    }
  }

  fn ident_or_keyword(&mut self) -> Result<Token, LexerErrorKind> {
    if let Some(ident) = self.read_ident() {
      if let Some(keyword) = Keyword::from(&*ident) {
        Ok(Token::Keyword(keyword))
      } else {
        Ok(Token::Ident(ident))
      }
    } else {
      Err(LexerErrorKind::ExpectedIdentOrKeyword)
    }
  }

  fn operator(&mut self) -> Result<Token, LexerErrorKind> {
    for operator in Operator::operators().iter() {
      if self.tokenizer.eat_str(operator).is_some() {
        if let Some(op) = Operator::from(operator) {
          return Ok(Token::Operator(op));
        }
      }
    }

    Err(LexerErrorKind::ExpectedOperator)
  }

  fn float_lit(&mut self) -> Result<Token, LexerErrorKind> {
    let mut float = String::new();
    let mut dec_or_exp = false;

    if let Some(start) = self.tokenizer.read_while(Lexer::is_decimal) {
      float.push_str(&*start);

      if self.tokenizer.eat_char('.').is_some() {
        float.push('.');

        if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
          float.push_str(&*dec);
        } else {
          return Err(LexerErrorKind::ExpectedDec);
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
            return Err(LexerErrorKind::ExpectedExp);
          }

          dec_or_exp = true;
        }
      }

      if !dec_or_exp {
        return Err(LexerErrorKind::ExpectedDecOrExp);
      }
    } else {
      return Err(LexerErrorKind::ExpectedFloatLit);
    }

    if let Ok(float) = float.parse::<f64>() {
      Ok(Token::Literal(Literal::Float(float)))
    } else {
      Err(LexerErrorKind::CouldNotParseFloat)
    }
  }

  fn int_lit(&mut self) -> Result<Token, LexerErrorKind> {
    if self.tokenizer.eat_str("0b").is_some() {
      if let Some(bin) = self.tokenizer.read_while(Lexer::is_binary) {
        Ok(Token::Literal(Literal::Int(Lexer::usize_from_binary(
          &*bin,
        ))))
      } else {
        Err(LexerErrorKind::ExpectedBin)
      }
    } else if self.tokenizer.eat_str("0o").is_some() {
      if let Some(oct) = self.tokenizer.read_while(Lexer::is_octal) {
        Ok(Token::Literal(Literal::Int(Lexer::usize_from_octal(&*oct))))
      } else {
        Err(LexerErrorKind::ExpectedOct)
      }
    } else if self.tokenizer.eat_str("0x").is_some() {
      if let Some(hex) = self.tokenizer.read_while(Lexer::is_hex) {
        Ok(Token::Literal(Literal::Int(Lexer::usize_from_hex(&*hex))))
      } else {
        Err(LexerErrorKind::ExpectedHex)
      }
    } else if let Some(dec) = self.tokenizer.read_while(Lexer::is_decimal) {
      Ok(Token::Literal(Literal::Int(Lexer::usize_from_decimal(
        &*dec,
      ))))
    } else {
      Err(LexerErrorKind::ExpectedIntLit)
    }
  }

  fn str_lit(&mut self) -> Result<Token, LexerErrorKind> {
    let mut inner = String::new();

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerErrorKind::ExpectedStringStartDelim);
    }

    if let Some(string) = self.read_inner() {
      inner.push_str(&*string);
    } else {
      return Err(LexerErrorKind::ExpectedStringInner);
    }

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerErrorKind::ExpectedStringEndDelim);
    }

    Ok(Token::Literal(Literal::Str(inner)))
  }

  fn char_lit(&mut self) -> Result<Token, LexerErrorKind> {
    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerErrorKind::ExpectedCharStartDelim);
    }

    let inner = if let Some(esc) = self.read_esc() {
      esc
    } else if let Some(ch) = self.tokenizer.step() {
      ch
    } else {
      return Err(LexerErrorKind::UnexpectedEof);
    };

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerErrorKind::ExpectedCharEndDelim);
    }

    Ok(Token::Literal(Literal::Char(inner)))
  }

  fn bool_lit(&mut self) -> Result<Token, LexerErrorKind> {
    if self.tokenizer.eat_str("true").is_some() {
      Ok(Token::Literal(Literal::Bool(true)))
    } else if self.tokenizer.eat_str("false").is_some() {
      Ok(Token::Literal(Literal::Bool(false)))
    } else {
      Err(LexerErrorKind::ExpectedBoolLit)
    }
  }

  fn tip(&mut self) -> Result<Token, LexerErrorKind> {
    if self.tokenizer.eat_char('#').is_none() {
      return Err(LexerErrorKind::ExpectedHash);
    }

    if self.tokenizer.eat_char('(').is_none() {
      return Err(LexerErrorKind::ExpectedLeftParen);
    }

    self.whitespace();

    let ident = if let Some(i) = self.read_ident() {
      i
    } else {
      return Err(LexerErrorKind::ExpectedTipIdent);
    };

    self.whitespace();

    if self.tokenizer.eat_char(')').is_none() {
      return Err(LexerErrorKind::ExpectedRightParen);
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
      return Err(LexerErrorKind::ExpectedNewline);
    };

    Ok(Token::Tip(Tip { ident, value }))
  }

  fn punc(&mut self) -> Result<Token, LexerErrorKind> {
    if let Some(ch) = self.tokenizer.peek() {
      if let Some(punc) = Punc::from(ch) {
        self.tokenizer.step();
        Ok(Token::Punc(punc))
      } else {
        Err(LexerErrorKind::ExpectedPunc)
      }
    } else {
      Err(LexerErrorKind::UnexpectedEof)
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

    ok_or_term!(self, self.bool_lit());
    ok_or_term!(self, self.ident_or_keyword());
    ok_or_term!(self, self.operator());
    ok_or_term!(self, self.float_lit());
    ok_or_term!(self, self.int_lit());
    ok_or_term!(self, self.str_lit());
    ok_or_term!(self, self.char_lit());
    ok_or_term!(self, self.tip());
    ok_or_term!(self, self.punc());

    Some(Err(LexerError::new(
      LexerErrorKind::NoMatch,
      Range {
        start: self.tokenizer.index,
        end: self.tokenizer.index,
      },
    )))
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use whistle_common::Range;

  #[test]
  fn whitespace() {
    let mut lexer = Lexer::new(" \t\r\n");

    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn comments() {
    let mut lexer = Lexer::new(
      "// line comment
                                /* inline comment */",
    );

    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn ident() {
    let mut lexer = Lexer::new("hello_w0r1d 你好吗");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Ident("hello_w0r1d".to_string()),
        range: Range { start: 0, end: 11 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Ident("你好吗".to_string()),
        range: Range { start: 12, end: 15 }
      }))
    );
  }

  #[test]
  fn keyword() {
    let lexer = Lexer::new("import as from export fun return if else while break continue var val for in match type struct trait");

    for tok in lexer {
      assert!(tok.is_ok());
      assert!(matches!(tok.unwrap().token, Token::Keyword(_)));
    }
  }

  #[test]
  fn operator() {
    let lexer = Lexer::new("~ ! + - * / % ** == != <= < > >= && || << >> & | ^ += -= * /= %= **= &&= ||= <<= >>= &= |= ^=");

    for tok in lexer {
      assert!(tok.is_ok());
      assert!(matches!(tok.unwrap().token, Token::Operator(_)));
    }
  }

  #[test]
  fn float_lit() {
    let mut lexer = Lexer::new("123e10 123.123e10 123.123");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Float(123e10)),
        range: Range { start: 0, end: 6 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Float(123.123e10)),
        range: Range { start: 7, end: 17 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Float(123.123)),
        range: Range { start: 18, end: 25 }
      }))
    );
  }

  #[test]
  fn int_lit() {
    let mut lexer = Lexer::new("123 0b01 0o07 0x0f");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Int(123)),
        range: Range { start: 0, end: 3 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Int(1)),
        range: Range { start: 4, end: 8 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Int(7)),
        range: Range { start: 9, end: 13 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Int(15)),
        range: Range { start: 14, end: 18 }
      }))
    );
  }

  #[test]
  fn string_lit() {
    let mut lexer = Lexer::new("\"\" \"asd\" \"\\\"\"");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Str(String::new())),
        range: Range { start: 0, end: 2 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Str("asd".to_string())),
        range: Range { start: 3, end: 8 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Str("\"".to_string())),
        range: Range { start: 9, end: 13 }
      }))
    );
  }

  #[test]
  fn char_lit() {
    let mut lexer = Lexer::new("'a' '\''");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Char('a')),
        range: Range { start: 0, end: 3 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Char('\'')),
        range: Range { start: 4, end: 7 }
      }))
    );
  }

  #[test]
  fn bool_lit() {
    let mut lexer = Lexer::new("true false");

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Bool(true)),
        range: Range { start: 0, end: 4 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Literal(Literal::Bool(false)),
        range: Range { start: 5, end: 10 }
      }))
    );
  }

  #[test]
  fn tip() {
    let mut lexer = Lexer::new(
      "#(tip) tip
                                #(tip) { tip }",
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Tip(Tip {
          ident: "tip".to_string(),
          value: "tip".to_string()
        }),
        range: Range { start: 0, end: 10 }
      }))
    );

    assert_eq!(
      lexer.next(),
      Some(Ok(TokenItem {
        token: Token::Tip(Tip {
          ident: "tip".to_string(),
          value: " tip ".to_string()
        }),
        range: Range { start: 43, end: 57 }
      }))
    );
  }

  #[test]
  fn punc() {
    let lexer = Lexer::new(", : . [ ] { } ( )");

    for tok in lexer {
      assert!(tok.is_ok());
      assert!(matches!(tok.unwrap().token, Token::Punc(_)));
    }
  }

  #[test]
  fn no_match() {
    let mut lexer = Lexer::new("¨");

    assert_eq!(
      lexer.next(),
      Some(Err(LexerError::new(
        LexerErrorKind::NoMatch,
        Range { start: 0, end: 0 }
      )))
    );
  }
}
