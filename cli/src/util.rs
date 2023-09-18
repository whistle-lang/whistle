use whistle_ast::Grammar;
use whistle_common::{DiagnosticHandler, TokenItem};
use whistle_compiler::*;
use whistle_parser::*;
use whistle_preprocessor::Preprocessor;

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

pub fn check(text: &str) -> (Checker, Grammar) {
  let (parser, mut grammar) = parse(text, false);
  let mut checker = Checker::new(parser);
  check_all(&mut checker, &mut grammar);
  handle_errors(&mut checker.handler);

  (checker, grammar)
}

pub fn compile(text: &str) -> Vec<u8> {
  let (checker, grammar) = check(text);
  let mut compiler = Compiler::new(checker);
  let res = compile_all(&mut compiler, grammar);
  handle_errors(&mut compiler.handler);

  res
}

pub fn handle_errors(handler: &mut DiagnosticHandler) {
  // TODO: format errors
  if handler.errors.len() > 0 {
    println!("{:#?}", handler.errors);
    std::process::exit(1);
  };
}