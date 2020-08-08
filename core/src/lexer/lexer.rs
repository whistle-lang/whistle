use unic_ucd_category::GeneralCategory;

use super::error::{ErrorKind, LexerError};
use super::token::{Token, TokenKind};
use super::tokens::{FloatLit, IntLit, Keyword, Operator, Punc, Tip};

use super::tokenizer::Tokenizer;

#[derive(Debug, Clone)]
pub struct Lexer {
  tokenizer: Tokenizer,
  keywords: Vec<String>,
  operators: Vec<String>,
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
    }
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
      .read_while(|ch| ch != '\\' || ch != '"')
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
    ('0'..'9').contains(&ch)
  }

  fn is_binary(ch: char) -> bool {
    ('0'..'1').contains(&ch)
  }

  fn is_octal(ch: char) -> bool {
    ('0'..'7').contains(&ch)
  }

  fn is_hex(ch: char) -> bool {
    ('0'..'9').contains(&ch) || ('a'..'f').contains(&ch) || ('A'..'F').contains(&ch)
  }

  fn whitespace(&mut self) {
    loop {
      match self.tokenizer.peek() {
        Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
          self.tokenizer.step();
        }
        _ => break,
      }
    }
  }

  fn comment(&mut self) {
    self.comment_line();
    self.comment_inline();
  }

  fn comment_line(&mut self) -> Option<Token<String>> {
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

      Some(Token::new(TokenKind::CommentLine, comment, index))
    } else {
      None
    }
  }

  fn comment_inline(&mut self) -> Result<Token<String>, LexerError> {
    let mut value = String::new();
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("/*").is_some() {
      let mut depth = 1;

      loop {
        if self.tokenizer.eat_str("/*").is_some() {
          depth += 1;
        } else if self.tokenizer.eat_str("*/").is_some() {
          depth -= 1;
        } else {
          if let Some(ch) = self.tokenizer.step() {
            value.push(ch);
          }
        }

        if depth == 0 {
          break;
        }
      }

      Ok(Token::new(TokenKind::CommentInline, value, index))
    } else {
      Err(LexerError::new(ErrorKind::ExpectedCommentInline, index))
    }
  }

  fn ident(&mut self) -> Result<Token<String>, LexerError> {
    let index = self.tokenizer.index;

    if let Some(ident) = self.read_ident() {
      if !self.keywords.contains(&ident) {
        Ok(Token::new(TokenKind::Ident, ident, index))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedIdent, index))
      }
    } else {
      Err(LexerError::new(ErrorKind::ExpectedIdent, index))
    }
  }

  fn keyword(&mut self) -> Result<Token<Keyword>, LexerError> {
    let index = self.tokenizer.index;

    if let Some(ident) = self.read_ident() {
      if let Some(keyword) = Keyword::from(&*ident) {
        Ok(Token::new(TokenKind::Keyword, keyword, index))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedKeyword, index))
      }
    } else {
      Err(LexerError::new(ErrorKind::ExpectedKeyword, index))
    }
  }

  fn operator(&mut self) -> Result<Token<Operator>, LexerError> {
    let index = self.tokenizer.index;

    for operator in self.operators.clone().iter() {
      if self.tokenizer.eat_str(operator).is_some() {
        if let Some(op) = Operator::from(operator) {
          return Ok(Token::new(TokenKind::Operator, op, index));
        }
      }
    }

    Err(LexerError::new(ErrorKind::ExpectedOperator, index))
  }

  fn float_lit(&mut self) -> Result<Token<FloatLit>, LexerError> {
    let index = self.tokenizer.index;
    let mut float = String::new();
    let mut has_dec_or_exp = false;

    if let Some(start) = self.tokenizer.read_while(|ch| Lexer::is_decimal(ch)) {
      float.push_str(&*start);

      if let Some(dot) = self.tokenizer.eat_char('.') {
        float.push('.');

        if let Some(dec) = self.tokenizer.read_while(|ch| Lexer::is_decimal(ch)) {
          float.push_str(&*dec);
        } else {
          return Err(LexerError::new(
            ErrorKind::ExpectedDec,
            self.tokenizer.index,
          ));
        }

        has_dec_or_exp = true;
      }

      if let Some(next) = self.tokenizer.peek() {
        if next == 'e' || next == 'E' {
          self.tokenizer.step();
          float.push('e');

          if let Some(plus) = self.tokenizer.eat_char('+') {
            float.push('+');
          } else if let Some(plus) = self.tokenizer.eat_char('-') {
            float.push('-');
          }

          if let Some(dec) = self.tokenizer.read_while(|ch| Lexer::is_decimal(ch)) {
            float.push_str(&*dec);
          } else {
            return Err(LexerError::new(
              ErrorKind::ExpectedExp,
              self.tokenizer.index,
            ));
          }

          has_dec_or_exp = true;
        }
      }

      if !has_dec_or_exp {
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

    Ok(Token::new(
      TokenKind::FloatLit,
      FloatLit::from(&*float),
      index,
    ))
  }

  fn int_lit(&mut self) -> Result<Token<IntLit>, LexerError> {
    let index = self.tokenizer.index;

    if let Some(_) = self.tokenizer.eat_str("0b") {
      if let Some(bin) = self.tokenizer.read_while(|ch| Lexer::is_binary(ch)) {
        Ok(Token::new(
          TokenKind::IntLit,
          IntLit::from_binary(&*bin),
          index,
        ))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedBin, index))
      }
    } else if let Some(_) = self.tokenizer.eat_str("0o") {
      if let Some(oct) = self.tokenizer.read_while(|ch| Lexer::is_octal(ch)) {
        Ok(Token::new(
          TokenKind::IntLit,
          IntLit::from_octal(&*oct),
          index,
        ))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedOct, index))
      }
    } else if let Some(_) = self.tokenizer.eat_str("0x") {
      if let Some(hex) = self.tokenizer.read_while(|ch| Lexer::is_hex(ch)) {
        Ok(Token::new(
          TokenKind::IntLit,
          IntLit::from_hex(&*hex),
          index,
        ))
      } else {
        Err(LexerError::new(ErrorKind::ExpectedHex, index))
      }
    } else if let Some(dec) = self.tokenizer.read_while(|ch| Lexer::is_decimal(ch)) {
      Ok(Token::new(
        TokenKind::IntLit,
        IntLit::from_decimal(&*dec),
        index,
      ))
    } else {
      Err(LexerError::new(ErrorKind::ExpectedIntLit, index))
    }
  }

  fn string_lit(&mut self) -> Result<Token<String>, LexerError> {
    let index = self.tokenizer.index;
    let mut inner = String::new();

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerError::new(ErrorKind::ExpectedStringStartDelim, index));
    }

    if let Some(string) = self.read_inner() {
      inner.push_str(&*string);
    } else {
      return Err(LexerError::new(ErrorKind::ExpectedStringInner, index));
    }

    if self.tokenizer.eat_char('"').is_none() {
      return Err(LexerError::new(ErrorKind::ExpectedStringEndDelim, index));
    }

    Ok(Token::new(TokenKind::StringLit, inner, index))
  }

  fn char_lit(&mut self) -> Result<Token<char>, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerError::new(ErrorKind::ExpectedCharStartDelim, index));
    }

    let inner = if let Some(esc) = self.read_esc() {
      esc
    } else if let Some(ch) = self.tokenizer.peek() {
      ch
    } else {
      return Err(LexerError::new(ErrorKind::ExpectedCharInner, index));
    };

    if self.tokenizer.eat_char('\'').is_none() {
      return Err(LexerError::new(
        ErrorKind::ExpectedCharEndDelim,
        self.tokenizer.index,
      ));
    }

    Ok(Token::new(TokenKind::CharLit, inner, index))
  }

  fn bool_lit(&mut self) -> Result<Token<bool>, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("true").is_some() {
      Ok(Token::new(TokenKind::BoolLit, true, index))
    } else if self.tokenizer.eat_str("false").is_some() {
      Ok(Token::new(TokenKind::BoolLit, false, index))
    } else {
      Err(LexerError::new(
        ErrorKind::ExpectedBoolLit,
        self.tokenizer.index,
      ))
    }
  }

  fn none_lit(&mut self) -> Result<Token<()>, LexerError> {
    let index = self.tokenizer.index;

    if self.tokenizer.eat_str("none").is_some() {
      Ok(Token::new(TokenKind::NoneLit, (), index))
    } else {
      Err(LexerError::new(
        ErrorKind::ExpectedNoneLit,
        self.tokenizer.index,
      ))
    }
  }
}
