use whistle_common::Keyword;
use whistle_common::Literal;
use whistle_common::Span;
use whistle_common::Token;
use whistle_common::TokenItem;
use whistle_lexer::Lexer;
use whistle_lexer::LexerError;
use whistle_lexer::LexerErrorKind;

#[derive(Default, Clone)]
pub struct Preprocessor {
  token_list: Vec<Vec<TokenItem>>,
}

impl Preprocessor {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn process(&mut self, src: &str) -> Result<(), LexerError> {
    let mut lexer = Lexer::new(src);
    let mut tokens: Vec<TokenItem> = Vec::new();

    let mut imports: Vec<String> = Vec::new();
    loop {
      let item = match lexer.next() {
        Some(v) => v?,
        None => break,
      };

      if item.token == Token::Keyword(Keyword::Import) {
        let import = match lexer.next() {
          Some(v) => v?,
          None => return Err(LexerError::new(LexerErrorKind::Eof, item.span)),
        };

        let import_file = match import.token {
          Token::Literal(lit) => match lit {
            Literal::Str(s) => s,
            _ => {
              return Err(LexerError::new(
                LexerErrorKind::ExpectedStringStartDelim,
                import.span,
              ))
            }
          },
          _ => {
            return Err(LexerError::new(
              LexerErrorKind::ExpectedStringStartDelim,
              import.span,
            ))
          }
        };

        imports.push(import_file);
      } else {
        tokens.push(item);
      }
    }
    for mut file_name in imports {
      if file_name.starts_with("@") {
        file_name.remove(0);
        file_name =
          "https://raw.githubusercontent.com/whistle-lang/std/main/".to_owned() + &file_name;
      }
      let file_data = if url::Url::parse(&file_name).is_ok() {
        match reqwest::blocking::get(file_name) {
          Ok(v) => match v.text() {
            Ok(v) => v,
            Err(_) => {
              return Err(LexerError::new(
                LexerErrorKind::UnexpectedEof,
                Span { start: 0, end: 0 },
              ))
            }
          },
          Err(_) => {
            return Err(LexerError::new(
              LexerErrorKind::UnexpectedEof,
              Span { start: 0, end: 0 },
            ))
          }
        }
      } else {
        match std::fs::read_to_string(file_name) {
          Ok(v) => v,
          Err(_) => {
            return Err(LexerError::new(
              LexerErrorKind::UnexpectedEof,
              Span { start: 0, end: 0 },
            ))
          }
        }
      };
      self.process(&file_data)?;
    }

    self.token_list.push(tokens);
    Ok(())
  }

  pub fn finalize(self) -> Vec<TokenItem> {
    // self.token_list.reverse();
    self.token_list.iter().fold(Vec::new(), |mut acc, v| {
      acc.extend_from_slice(v);

      acc
    })
  }
}
