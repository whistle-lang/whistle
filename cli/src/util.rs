use whistle_ast::Grammar;
use whistle_common::TokenItem;
use whistle_compiler::*;
use whistle_lexer::*;
use whistle_parser::*;
use whistle_preprocessor::Preprocessor;

pub fn preprocess(text: &str) -> Vec<TokenItem> {
  let mut processor = Preprocessor::new();
  match processor.process(text) {
    Ok(_) => {}
    Err(e) => println!("{:?}", e),
  };

  processor.finalize()
}

pub fn lex(text: &str, print: bool) -> Vec<TokenItem> {
  let lexer = Lexer::new(text);
  let mut toks = Vec::new();

  for tok in lexer {
    match tok {
      Ok(tok) => {
        if print {
          print!("{:?}", tok);
        }
        toks.push(tok.clone())
      }
      Err(err) => {
        println!("{:?}", err);
        std::process::exit(1);
      }
    }
  }

  toks
}

pub fn parse(text: &str, print: bool) -> Grammar {
  let tokens = preprocess(text);
  let parser = &mut Parser::new(tokens);

  match parse_all(parser) {
    Ok(val) => {
      if print {
        print!("{:?}", val);
      }
      val
    }
    Err(err) => {
      println!("{:?}", err);
      std::process::exit(1);
    }
  }
}

pub fn check(text: &str, print: bool) {
  let mut grammar = parse(text, false);
  let checker = &mut Checker::new();

  check_grammar(checker, &mut grammar);

  if checker.errors.len() > 0 && print {
    println!("{:#?}", checker.errors);
  }
}

pub fn compile(text: &str, print: bool) -> Vec<u8> {
  let mut grammar = parse(text, false);
  let compiler = &mut Compiler::new();
  let checker = &mut Checker::new();

  check_grammar(checker, &mut grammar);

  if checker.errors.len() > 0 {
    if print {
      println!("{:#?}", checker.errors);
    }
    std::process::exit(1);
  }

  match compile_grammar(compiler, grammar) {
    Ok(val) => val,
    Err(errs) => {
      for err in errs {
        println!("{:?}", err);
      }
      std::process::exit(1);
    }
  }
}
