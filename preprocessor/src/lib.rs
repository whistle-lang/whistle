use whistle_lexer::Lexer;
use whistle_lexer::LexerError;
use whistle_lexer::LexerErrorKind;
use whistle_common::TokenItem;
use whistle_common::Literal;
use whistle_common::Range;
use whistle_common::Keyword;
use whistle_common::Token;

#[derive(Default, Clone)]
pub struct Preprocessor {
    token_list: Vec<Vec<TokenItem>>,
}

impl Preprocessor {
    pub fn new() -> Self {
        Self::default()
    }

    fn merge_lists(&mut self, tokens: Vec<TokenItem>) {
        let new_list = Vec::with_capacity(self.token_list.len() + tokens.len());
        
        self.token_list = new_list;
    }

    pub fn process(&mut self, src: &str) -> Result<(), LexerError> {
        let mut lexer = Lexer::new(src);
        let mut tokens: Vec<TokenItem> = Vec::new();
        let mut offset: usize = 0;

        let mut imports: Vec<String> = Vec::new();
        loop {
            let item = match lexer.next() {
                Some(v) => v?,
                None => break,
            };

            if item.token == Token::Keyword(Keyword::Import) {
                let import = match lexer.next() {
                    Some(v) => v?,
                    None => return Err(LexerError::new(LexerErrorKind::Eof, item.range))
                };

                let import_file = match import.token {
                    Token::Literal(lit) => {
                        match lit {
                            Literal::Str(s) => s,
                            _ => return Err(LexerError::new(LexerErrorKind::ExpectedStringStartDelim, import.range))
                        }
                    },
                    _ => return Err(LexerError::new(LexerErrorKind::ExpectedStringStartDelim, import.range))
                };
                
                imports.push(import_file);
            } else {
                tokens.push(item);
            }
        }

        Ok(())
    }

    pub fn finalize(mut self) -> Vec<TokenItem> {
        self.token_list.reverse();
        self.token_list
            .iter()
            .fold(Vec::new(), |mut acc, v| {
                acc.extend_from_slice(v);

                acc
        })
    }
}