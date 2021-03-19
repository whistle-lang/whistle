use whistle_ast::Grammar;
use whistle_common::TokenItem;
use whistle_compiler::compile_grammar;
use whistle_compiler::Compiler;
use whistle_lexer::*;
use whistle_parser::*;

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
  let tokens = lex(text, false);
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

pub fn compile(text: &str) -> Vec<u8> {
  let grammar = parse(text, false);
  let compiler = &mut Compiler::new();

  compile_grammar(compiler, grammar)
}
