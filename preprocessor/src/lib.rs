use resolve_path::PathResolveExt;
use std::path;
use whistle_common::DiagnosticHandler;
use whistle_common::Keyword;
use whistle_common::LexerHandler;
use whistle_common::Literal;
use whistle_common::Span;
use whistle_common::Token;
use whistle_common::TokenItem;
use whistle_lexer::Lexer;
use whistle_lexer::LexerErrorKind;

#[derive(Clone)]
pub struct Preprocessor {
  pub token_list: Vec<Vec<TokenItem>>,
  pub handler: DiagnosticHandler,
}

impl Preprocessor {
  pub fn new(handler: DiagnosticHandler) -> Self {
    Self {
      token_list: Vec::new(),
      handler,
    }
  }

  pub fn process(&mut self, src: &str, path: &path::Path) {
    let mut lexer = Lexer::new(src);
    let mut tokens: Vec<TokenItem> = Vec::new();
    let mut imports: Vec<String> = Vec::new();
    loop {
      let item = match lexer.next() {
        Some(Ok(v)) => v,
        Some(Err(err)) => return self.handler.throw(err.kind, err.span),
        None => break,
      };

      if item.token == Token::Keyword(Keyword::Import) {
        let import = match lexer.next() {
          Some(Ok(v)) => v,
          Some(Err(err)) => return self.handler.throw(err.kind, err.span),
          None => return self.handler.throw(LexerErrorKind::Eof, item.span),
        };

        let import_file = match import.token {
          Token::Literal(lit) => match lit {
            Literal::Str(s) => s,
            _ => {
              return self
                .handler
                .throw(LexerErrorKind::ExpectedStringStartDelim, import.span)
            }
          },
          _ => {
            return self
              .handler
              .throw(LexerErrorKind::ExpectedStringStartDelim, import.span)
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

      let new_file_name = if url::Url::parse(&file_name).is_err() {
        match file_name.resolve_in(&path.canonicalize().unwrap()) {
          std::borrow::Cow::Owned(v) => v,
          std::borrow::Cow::Borrowed(v) => v.to_owned(),
        }
      } else {
        path.to_path_buf()
      };

      let file_data = if url::Url::parse(&file_name).is_ok() {
        match reqwest::blocking::get(file_name) {
          Ok(v) => match v.text() {
            Ok(v) => v,
            Err(_) => {
              return self
                .handler
                .throw(LexerErrorKind::Eof, Span { start: 0, end: 0 })
            }
          },
          Err(_) => {
            return self
              .handler
              .throw(LexerErrorKind::Eof, Span { start: 0, end: 0 })
          }
        }
      } else {
        match std::fs::read_to_string(&new_file_name) {
          Ok(v) => v,
          Err(_) => {
            return self
              .handler
              .throw(LexerErrorKind::Eof, Span { start: 0, end: 0 })
          }
        }
      };
      self.process(&file_data, &new_file_name);
    }

    self.token_list.push(tokens);
  }

  pub fn finalize(&self) -> Vec<TokenItem> {
    // self.token_list.reverse();
    self.token_list.iter().fold(Vec::new(), |mut acc, v| {
      acc.extend_from_slice(v);

      acc
    })
  }
}
