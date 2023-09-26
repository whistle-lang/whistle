

use whistle_ast::Grammar;
use whistle_common::{DiagnosticHandler, TokenItem};
use whistle_parser::*;
use whistle_preprocessor::Preprocessor;

pub fn file_text_has_ignore_comment(file_text: &str, ignore_file_comment_text: &str) -> bool {
  let mut has_ignore_comment = false;
  for line in file_text.lines() {
    if line.trim().starts_with(ignore_file_comment_text) {
      has_ignore_comment = true;
    }
  }
  has_ignore_comment
}

pub fn preprocess(text: &str, print: bool) -> (Preprocessor, Vec<TokenItem>) {
  let handler = DiagnosticHandler::new();
  let mut processor = Preprocessor::new(handler);
  processor.process(text);
  handle_errors(&mut processor.handler);
  let tokens = processor.finalize();

  if print {
    println!("{:#?}", tokens);
  }

  (processor, tokens)
}

pub fn parse(text: &str, print: bool) -> (Parser, Grammar) {
  let (processor, tokens) = preprocess(text, false);
  let mut parser = Parser::new(processor, tokens);
  let grammar = parse_all(&mut parser);
  handle_errors(&mut parser.handler);

  if print {
    println!("{:#?}", grammar);
  }

  (parser, grammar)
}

pub fn handle_errors(handler: &mut DiagnosticHandler) {
  // TODO: format errors
  if handler.errors.len() > 0 {
    println!("{:#?}", handler.errors);
  };
}